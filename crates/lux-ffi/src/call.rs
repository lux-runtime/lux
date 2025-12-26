//! FFI Call - Function invocation
//!
//! Provides dynamic function calling using libffi low-level API.

use crate::callback::FfiCallback;
use crate::memory::{CBox, CData};
use crate::types::*;
use libffi::low::{ffi_cif, ffi_type, prep_cif, prep_cif_var};
use libffi::raw::ffi_call;
use mlua::prelude::*;
use std::ffi::{CString, c_void};
use std::ptr;

// Platform-specific ABI
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use libffi::raw::ffi_abi_FFI_MS_CDECL as FFI_CDECL;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use libffi::raw::ffi_abi_FFI_STDCALL as FFI_STDCALL;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
use libffi::raw::ffi_abi_FFI_WIN64 as FFI_WIN64;

// Default ABI
#[cfg(not(target_os = "windows"))]
use libffi::raw::ffi_abi_FFI_DEFAULT_ABI as FFI_DEFAULT_ABI;

/// Cached FFI function with pre-prepared CIF for zero-overhead repeated calls
#[allow(dead_code)]
pub struct CachedFunction {
    /// Function pointer (resolved once)
    pub fn_ptr: usize,
    /// Function signature
    pub sig: FuncSig,
    /// Pre-prepared libffi CIF (avoids prep_cif on each call)
    cif: Box<ffi_cif>,
    /// Pre-computed libffi argument types (must outlive CIF)
    arg_types: Vec<*mut ffi_type>,
    /// Return type pointer
    ret_type: *mut ffi_type,
}

// SAFETY: CachedFunction contains raw pointers but they point to static libffi data
unsafe impl Send for CachedFunction {}
unsafe impl Sync for CachedFunction {}

impl CachedFunction {
    /// Create a new cached function with pre-prepared CIF
    pub fn new(fn_ptr: usize, sig: FuncSig) -> Result<Self, String> {
        let arg_ctypes: Vec<CType> = sig.args.iter().map(|(_, t)| t.clone()).collect();

        // Build ffi_type pointers
        let mut arg_types: Vec<*mut ffi_type> = arg_ctypes.iter().map(ctype_to_ffi_type).collect();

        let ret_type = ctype_to_ffi_type(&sig.ret);

        // Prepare CIF once
        let mut cif: ffi_cif = unsafe { std::mem::zeroed() };
        let abi = get_abi(sig.conv);

        let status = if sig.variadic {
            unsafe {
                prep_cif_var(
                    &mut cif,
                    abi,
                    arg_types.len(),
                    arg_types.len(),
                    ret_type,
                    arg_types.as_mut_ptr(),
                )
            }
        } else {
            unsafe {
                prep_cif(
                    &mut cif,
                    abi,
                    arg_types.len(),
                    ret_type,
                    arg_types.as_mut_ptr(),
                )
            }
        };

        if status.is_err() {
            return Err("Failed to prepare CIF".to_string());
        }

        Ok(Self {
            fn_ptr,
            sig,
            cif: Box::new(cif),
            arg_types,
            ret_type,
        })
    }

    /// Get function name
    pub fn name(&self) -> &str {
        &self.sig.name
    }
}

/// Fast path signature types for direct calls without libffi
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FastPathType {
    None,
    DoubleDouble,       // double func(double)
    DoubleDoubleDouble, // double func(double, double)
    IntInt,             // int func(int)
    VoidVoid,           // void func(void)
}

impl CachedFunction {
    /// Detect if this function can use a fast path
    pub fn detect_fast_path(sig: &FuncSig) -> FastPathType {
        let args = &sig.args;
        let ret = &sig.ret;

        match (ret, args.len()) {
            (CType::Double, 1) if matches!(args[0].1, CType::Double) => FastPathType::DoubleDouble,
            (CType::Double, 2)
                if matches!(args[0].1, CType::Double) && matches!(args[1].1, CType::Double) =>
            {
                FastPathType::DoubleDoubleDouble
            }
            (CType::Int, 1) if matches!(args[0].1, CType::Int) => FastPathType::IntInt,
            (CType::Void, 0) => FastPathType::VoidVoid,
            _ => FastPathType::None,
        }
    }
}

impl LuaUserData for CachedFunction {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // __call metamethod for direct invocation: func(args...)
        methods.add_meta_method(LuaMetaMethod::Call, |lua, this, args: LuaMultiValue| {
            // Helper to extract f64 from either Number or Integer
            fn get_f64(v: &LuaValue) -> Option<f64> {
                match v {
                    LuaValue::Number(n) => Some(*n),
                    LuaValue::Integer(i) => Some(*i as f64),
                    _ => None,
                }
            }

            // Try fast path first
            let fast_path = CachedFunction::detect_fast_path(&this.sig);

            match fast_path {
                FastPathType::DoubleDouble => {
                    if args.len() == 1 {
                        if let Some(arg) = args.get(0).and_then(get_f64) {
                            let result = unsafe { invoke_double_double(this.fn_ptr, arg) };
                            return Ok(LuaValue::Number(result));
                        }
                    }
                }
                FastPathType::DoubleDoubleDouble => {
                    if args.len() == 2 {
                        if let (Some(a), Some(b)) =
                            (args.get(0).and_then(get_f64), args.get(1).and_then(get_f64))
                        {
                            let result = unsafe { invoke_double_double_double(this.fn_ptr, a, b) };
                            return Ok(LuaValue::Number(result));
                        }
                    }
                }
                FastPathType::IntInt => {
                    if args.len() == 1 {
                        if let Some(arg) = args.get(0).and_then(|v| v.as_i32()) {
                            let result = unsafe { invoke_int_int(this.fn_ptr, arg) };
                            return Ok(LuaValue::Integer(result as i64));
                        }
                    }
                }
                FastPathType::VoidVoid => {
                    if args.is_empty() {
                        unsafe { invoke_void_void(this.fn_ptr) };
                        return Ok(LuaValue::Nil);
                    }
                }
                FastPathType::None => {}
            }

            // Fallback to generic path
            unsafe { invoke_cached(lua, this, args) }
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("ffi.func<{}>", this.name()))
        });
    }
}

// ============== FAST PATH FUNCTIONS ==============
// Direct function calls without libffi overhead

/// Fast path: double func(double)
#[inline(always)]
pub unsafe fn invoke_double_double(fn_ptr: usize, arg: f64) -> f64 {
    let func: extern "C" fn(f64) -> f64 = std::mem::transmute(fn_ptr);
    func(arg)
}

/// Fast path: double func(double, double)
#[inline(always)]
pub unsafe fn invoke_double_double_double(fn_ptr: usize, a: f64, b: f64) -> f64 {
    let func: extern "C" fn(f64, f64) -> f64 = std::mem::transmute(fn_ptr);
    func(a, b)
}

/// Fast path: int func(int)
#[inline(always)]
pub unsafe fn invoke_int_int(fn_ptr: usize, arg: i32) -> i32 {
    let func: extern "C" fn(i32) -> i32 = std::mem::transmute(fn_ptr);
    func(arg)
}

/// Fast path: void func(void)
#[inline(always)]
pub unsafe fn invoke_void_void(fn_ptr: usize) {
    let func: extern "C" fn() = std::mem::transmute(fn_ptr);
    func()
}

// ============== GENERIC PATH ==============

/// Invoke a cached function (generic path - CIF already prepared)
pub unsafe fn invoke_cached(
    lua: &Lua,
    cached: &CachedFunction,
    args: LuaMultiValue,
) -> LuaResult<LuaValue> {
    let arg_types: Vec<CType> = cached.sig.args.iter().map(|(_, t)| t.clone()).collect();

    let mut values: Vec<u64> = Vec::with_capacity(arg_types.len());
    let mut cstrings: Vec<CString> = Vec::new();
    let mut refs: Vec<usize> = Vec::new();
    let mut arg_values: Vec<*mut c_void> = Vec::with_capacity(arg_types.len());

    // Validate argument count
    let provided = args.len();
    let expected = arg_types.len();

    if !cached.sig.variadic && provided != expected {
        return Err(LuaError::external(format!(
            "Bad argument count: expected {}, got {}",
            expected, provided
        )));
    }

    // Prepare arguments
    for (i, arg_val) in args.iter().enumerate() {
        if i < expected {
            let ctype = &arg_types[i];
            let ptr = prepare_arg(arg_val, ctype, &mut values, &mut cstrings, &mut refs)?;
            arg_values.push(ptr);
        }
    }

    // Execute call with pre-prepared CIF
    let mut result: u64 = 0;

    ffi_call(
        cached.cif.as_ref() as *const ffi_cif as *mut ffi_cif,
        Some(std::mem::transmute(cached.fn_ptr)),
        &mut result as *mut u64 as *mut c_void,
        arg_values.as_mut_ptr(),
    );

    // Convert result
    c_to_lua(lua, &cached.sig.ret, result)
}

fn get_abi(conv: CallConv) -> libffi::raw::ffi_abi {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    match conv {
        CallConv::Stdcall => FFI_STDCALL,
        CallConv::C => FFI_CDECL,
        _ => FFI_CDECL,
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    match conv {
        // On x64, usually there's only one calling convention (Microsoft x64)
        // verify if strict stdcall is needed or ignored.
        // FFI_WIN64 covers it.
        _ => FFI_WIN64,
    }

    #[cfg(not(target_os = "windows"))]
    FFI_DEFAULT_ABI
}

/// Convert CType to libffi ffi_type pointer
fn ctype_to_ffi_type(ctype: &CType) -> *mut ffi_type {
    match ctype {
        CType::Void => ptr::addr_of_mut!(libffi::low::types::void),
        CType::Bool | CType::Char | CType::Int8 => ptr::addr_of_mut!(libffi::low::types::sint8),
        CType::UChar | CType::UInt8 => ptr::addr_of_mut!(libffi::low::types::uint8),
        CType::Short | CType::Int16 => ptr::addr_of_mut!(libffi::low::types::sint16),
        CType::UShort | CType::UInt16 | CType::WChar => {
            ptr::addr_of_mut!(libffi::low::types::uint16)
        }
        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            ptr::addr_of_mut!(libffi::low::types::sint32)
        }
        CType::UInt | CType::UInt32 => ptr::addr_of_mut!(libffi::low::types::uint32),
        CType::Long | CType::LongLong | CType::Int64 => {
            ptr::addr_of_mut!(libffi::low::types::sint64)
        }
        CType::ULong | CType::ULongLong | CType::UInt64 => {
            ptr::addr_of_mut!(libffi::low::types::uint64)
        }
        CType::Float => ptr::addr_of_mut!(libffi::low::types::float),
        CType::Double => ptr::addr_of_mut!(libffi::low::types::double),
        CType::Pointer(_)
        | CType::Struct(_)
        | CType::Union(_)
        | CType::Array(_, _)
        | CType::Function(_)
        | CType::GUID => ptr::addr_of_mut!(libffi::low::types::pointer),
    }
}

/// Invoke a function dynamically
pub fn invoke(lua: &Lua, fn_ptr: usize, sig: &FuncSig, args: LuaMultiValue) -> LuaResult<LuaValue> {
    // Extract types from FuncSig (discarding names)
    let arg_types: Vec<CType> = sig.args.iter().map(|(_, t)| t.clone()).collect();
    unsafe {
        invoke_impl(
            lua.clone(),
            fn_ptr,
            &sig.ret,
            &arg_types,
            sig.variadic,
            sig.conv,
            args,
        )
    }
}

// New helper for calling a function pointer directly with arguments
pub unsafe fn ffi_call_ptr(
    lua: Lua,
    fn_ptr: usize,
    ctype: &CType,
    args: LuaMultiValue,
) -> LuaResult<LuaValue> {
    // Extract signature from ctype
    let sig = match ctype {
        CType::Function(sig) => sig,
        CType::Pointer(inner) => {
            if let Some(inner_type) = inner.as_ref() {
                if let CType::Function(sig) = inner_type.as_ref() {
                    sig
                } else {
                    return Err(LuaError::external(
                        "ffi.call: pointer target is not a function",
                    ));
                }
            } else {
                return Err(LuaError::external(
                    "ffi.call: void pointer is not a function",
                ));
            }
        }
        _ => return Err(LuaError::external("ffi.call: not a function type")),
    };

    // Reuse existing call logic?
    // call::ffi_call takes lua, args. It expects to find the function... wait.
    // The existing ffi_call in module exports finds the function by name from the library.
    // We already have the pointer here.

    // We need to implement the invocation logic reusing the body of call::invoke.
    // call::invoke takes (lua, fn_ptr, sig, args).
    // EXCEPT `invoke` is not public currently. It's inside `call.rs`.
    // Let's make `invoke` public or move its body to `ffi_call_ptr`.

    // Looking at file content, `invoke` handles the CIF setup and call.
    // So we can just call `invoke`.
    // So we can just call `invoke`.
    unsafe {
        invoke_impl(
            lua,
            fn_ptr,
            &sig.ret,
            &sig.args,
            sig.variadic,
            sig.conv,
            args,
        )
    }
}

unsafe fn invoke_impl(
    lua: Lua,
    fn_ptr: usize,
    ret_type: &CType,
    arg_types: &[CType],
    variadic: bool,
    conv: crate::types::CallConv,
    args: LuaMultiValue,
) -> LuaResult<LuaValue> {
    let mut values: Vec<u64> = Vec::new();
    let mut cstrings: Vec<CString> = Vec::new();
    let mut refs: Vec<usize> = Vec::new();
    let mut ffi_arg_types: Vec<*mut ffi_type> = Vec::new();
    let mut arg_values: Vec<*mut c_void> = Vec::new();

    // Prepare arguments
    let provided = args.len();
    let expected = arg_types.len();

    if !variadic && provided != expected {
        return Err(LuaError::external(format!(
            "Bad argument count: expected {}, got {}",
            expected, provided
        )));
    }

    if variadic && provided < expected {
        return Err(LuaError::external(format!(
            "Bad argument count: expected at least {}, got {}",
            expected, provided
        )));
    }

    // Process fixed arguments
    for (i, arg_val) in args.iter().enumerate() {
        if i < expected {
            let ctype = &arg_types[i];
            ffi_arg_types.push(ctype_to_ffi_type(ctype));
            let ptr = prepare_arg(arg_val, ctype, &mut values, &mut cstrings, &mut refs)?;
            arg_values.push(ptr);
        } else {
            // Variadic arguments
            // We need to guess the type or convert based on Lua type?
            // LuaJIT allows passing cdata types to varargs.
            // For now, let's assume primitives like int/double based on Lua value.
            // Simplification: treat integer as Int, number as Double, string as Char*.
            // This is tricky without type info.
            // A better approach for variadic:
            // Let user cast? or just support basic types.

            // Infer type from value
            let ctype = match arg_val {
                LuaValue::Integer(_) => CType::Int, // or Long? Default C behavior often Promotes to Int/Double
                LuaValue::Number(_) => CType::Double,
                LuaValue::String(_) => CType::Pointer(Some(Box::new(CType::Char))),
                LuaValue::Boolean(_) => CType::Int,
                LuaValue::LightUserData(_) | LuaValue::UserData(_) => CType::Pointer(None),
                _ => CType::Void, // Skip or error
            };

            if ctype == CType::Void {
                continue; // Skip nil/tables
            }

            ffi_arg_types.push(ctype_to_ffi_type(&ctype));
            let ptr = prepare_arg(arg_val, &ctype, &mut values, &mut cstrings, &mut refs)?;
            arg_values.push(ptr);
        }
    }

    // Call
    unsafe {
        let rtype = ctype_to_ffi_type(ret_type);
        let mut cif: ffi_cif = std::mem::zeroed();
        let abi = get_abi(conv);

        // Count total args (fixed + variable)
        let total_args = ffi_arg_types.len();

        let status = if variadic {
            prep_cif_var(
                &mut cif,
                abi,
                expected,
                total_args,
                rtype,
                ffi_arg_types.as_mut_ptr(),
            )
        } else {
            prep_cif(&mut cif, abi, total_args, rtype, ffi_arg_types.as_mut_ptr())
        };

        if status.is_err() {
            return Err(LuaError::external("Failed to prepare CIF"));
        }

        // Return value storage
        let mut result: u64 = 0;

        ffi_call(
            &mut cif,
            Some(std::mem::transmute(fn_ptr)),
            &mut result as *mut u64 as *mut c_void,
            arg_values.as_mut_ptr(),
        );
        // Convert result to Lua
        c_to_lua(&lua, ret_type, result)
    }
}

fn prepare_arg(
    val: &LuaValue,
    ctype: &CType,
    values: &mut Vec<u64>,
    cstrings: &mut Vec<CString>,
    _refs: &mut Vec<usize>,
) -> LuaResult<*mut c_void> {
    let slot_idx = values.len();
    values.push(0); // Reserve slot
    let slot_ptr = &mut values[slot_idx] as *mut u64;

    unsafe {
        match ctype {
            CType::Int | CType::Enum(_) => {
                *(slot_ptr as *mut i32) = val.as_i32().unwrap_or(0);
            }
            CType::UInt => {
                *(slot_ptr as *mut u32) = val.as_u32().unwrap_or(0);
            }
            CType::Long => {
                let v = if let LuaValue::LightUserData(ud) = val {
                    ud.0 as i64
                } else if let LuaValue::UserData(_ud) = val {
                    crate::memory::get_ptr_from_value(val)
                        .map(|p| p as i64)
                        .unwrap_or(0)
                } else {
                    val.as_i64().unwrap_or(0)
                };
                *(slot_ptr as *mut i64) = v;
            }
            CType::ULong => {
                let v = if let LuaValue::LightUserData(ud) = val {
                    ud.0 as u64
                } else if let LuaValue::UserData(_ud) = val {
                    crate::memory::get_ptr_from_value(val)
                        .map(|p| p as u64)
                        .unwrap_or(0)
                } else {
                    val.as_u64().unwrap_or(0)
                };
                *(slot_ptr as *mut u64) = v;
            }
            CType::Short => {
                *(slot_ptr as *mut i16) = val.as_i32().unwrap_or(0) as i16;
            }
            CType::UShort => {
                *(slot_ptr as *mut u16) = val.as_u32().unwrap_or(0) as u16;
            }
            CType::Char => {
                *(slot_ptr as *mut i8) = val.as_i32().unwrap_or(0) as i8;
            }
            CType::UChar => {
                *(slot_ptr as *mut u8) = val.as_u32().unwrap_or(0) as u8;
            }
            CType::Bool => {
                *(slot_ptr as *mut i8) = if matches!(val, LuaValue::Boolean(true)) {
                    1
                } else {
                    0
                };
            }
            CType::Float => {
                *(slot_ptr as *mut f32) = val.as_f64().unwrap_or(0.0) as f32;
            }
            CType::Double => {
                *(slot_ptr as *mut f64) = val.as_f64().unwrap_or(0.0);
            }
            CType::Pointer(inner) => {
                // Handle strings specifically if inner is char
                let is_string = matches!(inner.as_ref(), Some(b) if **b == CType::Char);

                let ptr_val = if is_string {
                    if let LuaValue::String(s) = val {
                        let cstr = CString::new(s.as_bytes().to_vec())
                            .map_err(|_| LuaError::external("Null byte in string"))?;
                        let p = cstr.as_ptr() as usize;
                        cstrings.push(cstr);
                        p
                    } else {
                        0
                    }
                } else {
                    match val {
                        LuaValue::LightUserData(ud) => ud.0 as usize,
                        LuaValue::Integer(i) => *i as usize,
                        LuaValue::UserData(ud) => {
                            if let Ok(cbox) = ud.borrow::<CBox>() {
                                cbox.ptr() as usize
                            } else if let Ok(cb) = ud.borrow::<FfiCallback>() {
                                cb.as_ptr() as usize
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    }
                };
                *(slot_ptr as *mut usize) = ptr_val;
            }
            _ => {
                // Structs/Arrays by value not fully supported here, passing as pointer/sized?
                // Fallback to usize 0
                *(slot_ptr as *mut usize) = 0;
            }
        }
    }

    Ok(slot_ptr as *mut c_void)
}

fn c_to_lua(lua: &Lua, ctype: &CType, raw: u64) -> LuaResult<LuaValue> {
    // Same logic as before roughly
    match ctype {
        CType::Void => Ok(LuaValue::Nil),
        CType::Bool => Ok(LuaValue::Boolean(raw != 0)),
        CType::Char => Ok(LuaValue::Integer(raw as i8 as i64)),
        CType::UChar => Ok(LuaValue::Integer(raw as u8 as i64)),
        CType::Short => Ok(LuaValue::Integer(raw as i16 as i64)),
        CType::UShort => Ok(LuaValue::Integer(raw as u16 as i64)),
        CType::Int | CType::Enum(_) => Ok(LuaValue::Integer(raw as i32 as i64)),
        CType::UInt => Ok(LuaValue::Integer(raw as u32 as i64)),
        CType::Long => Ok(LuaValue::Integer(raw as i64)),
        CType::ULong => Ok(LuaValue::Integer(raw as i64)),
        CType::Float => {
            let f = f32::from_bits(raw as u32);
            Ok(LuaValue::Number(f as f64))
        }
        CType::Double => {
            let d = f64::from_bits(raw);
            Ok(LuaValue::Number(d))
        }
        CType::Pointer(inner) => {
            if raw == 0 {
                Ok(LuaValue::Nil)
            } else if matches!(inner.as_ref(), Some(t) if **t == CType::Char) {
                // char* -> string
                let cstr = unsafe { std::ffi::CStr::from_ptr(raw as *const i8) };
                let s = lua.create_string(cstr.to_bytes())?;
                Ok(LuaValue::String(s))
            } else {
                Ok(LuaValue::LightUserData(LuaLightUserData(
                    raw as *mut c_void,
                )))
            }
        }
        _ => Ok(LuaValue::Integer(raw as i64)),
    }
}

/// Get/set C errno
pub fn ffi_errno(new_errno: Option<i32>) -> LuaResult<i32> {
    #[cfg(windows)]
    {
        unsafe extern "C" {
            fn _errno() -> *mut i32;
        }
        unsafe {
            let ptr = _errno();
            let old = *ptr;
            if let Some(new) = new_errno {
                *ptr = new;
            }
            Ok(old)
        }
    }
    #[cfg(not(windows))]
    {
        unsafe extern "C" {
            fn __errno_location() -> *mut i32;
        }
        unsafe {
            let ptr = __errno_location();
            let old = *ptr;
            if let Some(new) = new_errno {
                *ptr = new;
            }
            Ok(old)
        }
    }
}
