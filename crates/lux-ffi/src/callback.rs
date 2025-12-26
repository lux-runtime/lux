//! FFI Callbacks with libffi closures
//!
//! Uses libffi::low API for dynamic closures with proper type conversions.

use crate::memory::CBox;
use crate::types::{CType, CallConv};
use libffi::low::{
    CodePtr, closure_alloc, closure_free, ffi_cif, ffi_closure, ffi_type, prep_cif,
    prep_closure_mut,
};
use mlua::prelude::*;
use std::ffi::c_void;
use std::ptr::{self, addr_of_mut};

// ABI Handling
#[cfg(not(target_os = "windows"))]
use libffi::raw::ffi_abi_FFI_DEFAULT_ABI as FFI_DEFAULT_ABI;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use libffi::raw::ffi_abi_FFI_MS_CDECL as FFI_CDECL;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use libffi::raw::ffi_abi_FFI_STDCALL as FFI_STDCALL;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
use libffi::raw::ffi_abi_FFI_WIN64 as FFI_WIN64;

fn get_abi(conv: CallConv) -> libffi::raw::ffi_abi {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    match conv {
        CallConv::Stdcall => FFI_STDCALL,
        CallConv::C => FFI_CDECL,
        _ => FFI_CDECL,
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    match conv {
        _ => FFI_WIN64,
    }

    #[cfg(not(target_os = "windows"))]
    FFI_DEFAULT_ABI
}

// ============================================================================
// Type Conversion: CType -> ffi_type
// ============================================================================

/// Convert CType to libffi ffi_type pointer
fn ctype_to_ffi_type(ctype: &CType) -> *mut ffi_type {
    match ctype {
        CType::Void => addr_of_mut!(libffi::low::types::void),
        CType::Bool | CType::Char | CType::Int8 => addr_of_mut!(libffi::low::types::sint8),
        CType::UChar | CType::UInt8 => addr_of_mut!(libffi::low::types::uint8),
        CType::Short | CType::Int16 => addr_of_mut!(libffi::low::types::sint16),
        CType::UShort | CType::UInt16 | CType::WChar => addr_of_mut!(libffi::low::types::uint16),
        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            addr_of_mut!(libffi::low::types::sint32)
        }
        CType::UInt | CType::UInt32 => addr_of_mut!(libffi::low::types::uint32),
        CType::Long | CType::LongLong | CType::Int64 => addr_of_mut!(libffi::low::types::sint64),
        CType::ULong | CType::ULongLong | CType::UInt64 => addr_of_mut!(libffi::low::types::uint64),
        CType::Float => addr_of_mut!(libffi::low::types::float),
        CType::Double => addr_of_mut!(libffi::low::types::double),
        // All pointer types use pointer
        CType::Pointer(_)
        | CType::Struct(_)
        | CType::Union(_)
        | CType::Array(_, _)
        | CType::Function(_)
        | CType::GUID => addr_of_mut!(libffi::low::types::pointer),
    }
}

// ============================================================================
// Callback Data - Stored with each callback
// ============================================================================

/// Userdata stored with each callback
struct CallbackData {
    func_key: LuaRegistryKey,
    lua: Lua,
    arg_types: Vec<CType>,
    ret_type: CType,
}

// ============================================================================
// Callback Trampoline - Called by C code
// ============================================================================

/// The callback trampoline - signature must match libffi's expectation
unsafe extern "C" fn callback_trampoline(
    _cif: &ffi_cif,
    result: &mut c_void,
    args: *const *const c_void,
    userdata: &mut c_void,
) {
    // All operations here are inside an unsafe block since this is an unsafe fn
    let data = unsafe { &*(userdata as *const c_void as *const CallbackData) };
    let lua = &data.lua;

    // println!("[FFI] In trampoline, calling Lua...");

    // Get Lua function from registry
    let func: LuaFunction = match lua.registry_value(&data.func_key) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[FFI CALLBACK ERROR] Failed to get Lua function: {}", e);
            return;
        }
    };

    // Convert C args to Lua values
    let mut lua_args = Vec::with_capacity(data.arg_types.len());
    for (i, arg_type) in data.arg_types.iter().enumerate() {
        let arg_ptr = unsafe { *args.add(i) };
        let lua_val = unsafe { c_arg_to_lua(lua, arg_type, arg_ptr) };
        lua_args.push(lua_val);
    }

    // Call Lua function
    let call_result = func.call::<LuaMultiValue>(LuaMultiValue::from_iter(lua_args));

    // Convert result back to C
    let ret_ptr = result as *mut c_void;
    match call_result {
        Ok(values) => {
            let first = values.into_iter().next().unwrap_or(LuaValue::Nil);
            unsafe { lua_to_c_result(&data.ret_type, &first, ret_ptr) };
        }
        Err(e) => {
            eprintln!("[FFI CALLBACK ERROR] Lua function error: {}", e);
            // Set default return value on error
            unsafe { lua_to_c_result(&data.ret_type, &LuaValue::Integer(0), ret_ptr) };
        }
    }
}

// ============================================================================
// C -> Lua Conversion
// ============================================================================

unsafe fn c_arg_to_lua(lua: &Lua, ctype: &CType, ptr: *const c_void) -> LuaValue {
    if ptr.is_null() {
        return LuaValue::Nil;
    }

    match ctype {
        CType::Void => LuaValue::Nil,

        CType::Bool => LuaValue::Boolean(unsafe { *(ptr as *const i8) } != 0),

        CType::Char | CType::Int8 => LuaValue::Integer(i64::from(unsafe { *(ptr as *const i8) })),
        CType::UChar | CType::UInt8 => LuaValue::Integer(i64::from(unsafe { *(ptr as *const u8) })),

        CType::Short | CType::Int16 => {
            LuaValue::Integer(i64::from(unsafe { *(ptr as *const i16) }))
        }
        CType::UShort | CType::UInt16 | CType::WChar => {
            LuaValue::Integer(i64::from(unsafe { *(ptr as *const u16) }))
        }

        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            LuaValue::Integer(i64::from(unsafe { *(ptr as *const i32) }))
        }
        CType::UInt | CType::UInt32 => {
            LuaValue::Integer(i64::from(unsafe { *(ptr as *const u32) }))
        }

        CType::Long | CType::LongLong | CType::Int64 => {
            LuaValue::Integer(unsafe { *(ptr as *const i64) })
        }
        CType::ULong | CType::ULongLong | CType::UInt64 => {
            LuaValue::Number(unsafe { *(ptr as *const u64) } as f64)
        }

        CType::Float => LuaValue::Number(f64::from(unsafe { *(ptr as *const f32) })),
        CType::Double => LuaValue::Number(unsafe { *(ptr as *const f64) }),

        CType::Pointer(inner) => {
            // Special handling for char* (strings)
            if let Some(inner_type) = inner {
                if **inner_type == CType::Char {
                    let cptr = unsafe { *(ptr as *const *const std::ffi::c_char) };
                    if cptr.is_null() {
                        return LuaValue::Nil;
                    }
                    match unsafe { std::ffi::CStr::from_ptr(cptr) }.to_str() {
                        Ok(s) => {
                            return lua
                                .create_string(s)
                                .map(LuaValue::String)
                                .unwrap_or(LuaValue::Nil);
                        }
                        Err(_) => return LuaValue::Nil,
                    }
                }
            }
            // Generic pointer - return as LightUserData
            LuaValue::LightUserData(LuaLightUserData(unsafe { *(ptr as *const *mut c_void) }))
        }

        // Struct/Union/Array/Function/GUID pointers
        CType::Struct(_)
        | CType::Union(_)
        | CType::Array(_, _)
        | CType::Function(_)
        | CType::GUID => {
            LuaValue::LightUserData(LuaLightUserData(unsafe { *(ptr as *const *mut c_void) }))
        }
    }
}

// ============================================================================
// Lua -> C Conversion
// ============================================================================

unsafe fn lua_to_c_result(ctype: &CType, val: &LuaValue, ret_ptr: *mut c_void) {
    if ret_ptr.is_null() {
        return;
    }

    match ctype {
        CType::Void => {}

        CType::Bool => {
            let b = match val {
                LuaValue::Boolean(b) => *b,
                LuaValue::Nil => false,
                LuaValue::Integer(i) => *i != 0,
                LuaValue::Number(n) => *n != 0.0,
                _ => true, // Any other value is truthy
            };
            *(ret_ptr as *mut i8) = if b { 1 } else { 0 };
        }

        CType::Char | CType::Int8 => {
            *(ret_ptr as *mut i8) = lua_to_i64(val) as i8;
        }
        CType::UChar | CType::UInt8 => {
            *(ret_ptr as *mut u8) = lua_to_i64(val) as u8;
        }
        CType::Short | CType::Int16 => {
            *(ret_ptr as *mut i16) = lua_to_i64(val) as i16;
        }
        CType::UShort | CType::UInt16 | CType::WChar => {
            *(ret_ptr as *mut u16) = lua_to_i64(val) as u16;
        }
        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            *(ret_ptr as *mut i32) = lua_to_i64(val) as i32;
        }
        CType::UInt | CType::UInt32 => {
            *(ret_ptr as *mut u32) = lua_to_i64(val) as u32;
        }
        CType::Long | CType::LongLong | CType::Int64 => {
            // LRESULT is typically long (i64 on 64-bit)
            *(ret_ptr as *mut i64) = lua_to_i64(val);
        }
        CType::ULong | CType::ULongLong | CType::UInt64 => {
            *(ret_ptr as *mut u64) = lua_to_u64(val);
        }
        CType::Float => {
            *(ret_ptr as *mut f32) = lua_to_f64(val) as f32;
        }
        CType::Double => {
            *(ret_ptr as *mut f64) = lua_to_f64(val);
        }

        CType::Pointer(_)
        | CType::Struct(_)
        | CType::Union(_)
        | CType::Array(_, _)
        | CType::Function(_)
        | CType::GUID => {
            // For pointers, we need to handle various Lua types robustly
            let ptr_val: *mut c_void = match val {
                LuaValue::LightUserData(ud) => ud.0,
                LuaValue::Integer(i) => *i as *mut c_void,
                LuaValue::Number(n) => *n as i64 as *mut c_void,
                LuaValue::Nil => ptr::null_mut(),
                LuaValue::Boolean(false) => ptr::null_mut(),
                LuaValue::Boolean(true) => 1 as *mut c_void,
                LuaValue::UserData(ud) => {
                    if let Ok(cbox) = ud.borrow::<CBox>() {
                        cbox.as_ptr()
                    } else if let Ok(cb) = ud.borrow::<crate::callback::FfiCallback>() {
                        cb.ptr() as *mut c_void
                    } else {
                        ptr::null_mut()
                    }
                }
                LuaValue::String(s) => {
                    // String -> char* (dangerous, string must stay alive!)
                    s.as_bytes().as_ptr() as *mut c_void
                }
                _ => ptr::null_mut(),
            };
            *(ret_ptr as *mut *mut c_void) = ptr_val;
        }
    }
}

fn lua_to_i64(val: &LuaValue) -> i64 {
    match val {
        LuaValue::Integer(i) => *i,
        LuaValue::Number(n) => *n as i64,
        LuaValue::Boolean(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        LuaValue::Nil => 0,
        _ => 0,
    }
}

fn lua_to_u64(val: &LuaValue) -> u64 {
    match val {
        LuaValue::Integer(i) => *i as u64,
        LuaValue::Number(n) => *n as u64,
        LuaValue::Boolean(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        LuaValue::Nil => 0,
        _ => 0,
    }
}

fn lua_to_f64(val: &LuaValue) -> f64 {
    match val {
        LuaValue::Number(n) => *n,
        LuaValue::Integer(i) => *i as f64,
        LuaValue::Boolean(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        LuaValue::Nil => 0.0,
        _ => 0.0,
    }
}

// ============================================================================
// FfiCallback - The main callback struct
// ============================================================================

/// A callback that can be passed to C functions.
pub struct FfiCallback {
    closure: *mut ffi_closure,
    code_ptr: CodePtr,
    _cif: Box<ffi_cif>,
    _arg_types_ffi: Vec<*mut ffi_type>,
    _data: Box<CallbackData>,
    ret_type: CType,
    arg_count: usize,
}

unsafe impl Send for FfiCallback {}
unsafe impl Sync for FfiCallback {}

impl FfiCallback {
    /// Create a new callback from a Lua function.
    pub fn new(
        lua: &Lua,
        func: LuaFunction,
        ret_type: CType,
        arg_types: Vec<CType>,
        conv: CallConv,
    ) -> LuaResult<Self> {
        let func_key = lua.create_registry_value(func)?;

        let arg_types_ffi: Vec<*mut ffi_type> =
            arg_types.iter().map(|t| ctype_to_ffi_type(t)).collect();

        let ret_type_ffi = ctype_to_ffi_type(&ret_type);
        let ret_type_for_data = ret_type.clone();

        let mut cif = Box::new(unsafe { std::mem::zeroed::<ffi_cif>() });

        // Select ABI based on convention and platform
        let abi = get_abi(conv);

        let status = unsafe {
            prep_cif(
                cif.as_mut(),
                abi,
                arg_types_ffi.len(),
                ret_type_ffi,
                if arg_types_ffi.is_empty() {
                    ptr::null_mut()
                } else {
                    arg_types_ffi.as_ptr() as *mut _
                },
            )
        };

        if status.is_err() {
            eprintln!("[FFI ERROR] Failed to prepare CIF for callback");
            return Err(LuaError::external("Failed to prepare callback CIF"));
        }

        let (closure, code_ptr) = closure_alloc();

        if closure.is_null() {
            eprintln!("[FFI ERROR] Failed to allocate closure");
            return Err(LuaError::external("Failed to allocate closure"));
        }

        let data = Box::new(CallbackData {
            func_key,
            lua: lua.clone(),
            arg_types: arg_types.clone(),
            ret_type: ret_type_for_data,
        });

        let arg_count = arg_types.len();

        let status = unsafe {
            prep_closure_mut(
                closure,
                cif.as_mut(),
                callback_trampoline,
                data.as_ref() as *const CallbackData as *mut c_void,
                code_ptr,
            )
        };

        if status.is_err() {
            unsafe { closure_free(closure) };
            eprintln!("[FFI ERROR] Failed to prepare closure");
            return Err(LuaError::external("Failed to prepare closure"));
        }

        Ok(Self {
            closure,
            code_ptr,
            _cif: cif,
            _arg_types_ffi: arg_types_ffi,
            _data: data,
            ret_type,
            arg_count,
        })
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.code_ptr.as_ptr() as *mut c_void
    }

    pub fn ptr(&self) -> usize {
        self.code_ptr.as_ptr() as usize
    }
}

impl Drop for FfiCallback {
    fn drop(&mut self) {
        if !self.closure.is_null() {
            unsafe { closure_free(self.closure) };
        }
    }
}

impl LuaUserData for FfiCallback {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("_ptr", |_, this| Ok(this.ptr()));
        fields.add_field_method_get("address", |_, this| Ok(this.ptr()));
        fields.add_field_method_get("ptr", |_, this| Ok(LuaLightUserData(this.as_ptr())));
        fields.add_field_method_get("retType", |lua, this| this.ret_type.clone().into_lua(lua));
        fields.add_field_method_get("argCount", |_, this| Ok(this.arg_count));
        fields.add_field_method_get("isValid", |_, this| Ok(!this.closure.is_null()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getPtr", |_, this, ()| Ok(LuaLightUserData(this.as_ptr())));
        methods.add_method("isValid", |_, this, ()| Ok(!this.closure.is_null()));
        methods.add_method("free", |_, _, ()| Ok(()));
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Create a callback from a Lua function with signature string
pub fn create_callback(lua: &Lua, sig_str: &str, func: LuaFunction) -> LuaResult<LuaAnyUserData> {
    let (ret_type, arg_types, conv) = parse_callback_signature(sig_str)?;
    let cb = FfiCallback::new(lua, func, ret_type, arg_types, conv)?;
    lua.create_userdata(cb)
}

/// Parse callback signature: "int(int, int)" -> (CType, Vec<CType>, CallConv)
fn parse_callback_signature(sig: &str) -> LuaResult<(CType, Vec<CType>, CallConv)> {
    // Reuse parser.rs logic if possible, or simple local parsing
    // But we need to handle "stdcall" etc in signature string
    // "int (__stdcall *)(int)" or similar?
    // Or just "int(int)" and rely on default?
    // If the user wants stdcall callback, they usually write "int (__stdcall *)(int)" or "int __stdcall(int)"

    // Simplification based on parser.rs logic:
    // parse_func_decl matches "ret match ( args )"
    // Let's wrap it in a dummy name to use parser.rs's robust logic if exposed?
    // parse_func_decl is not public? I made it private.
    // I should make it public or duplicate logic.
    // I made `parse_func_decl` private in `parser.rs`.
    // I will implement simple parsing here checking for __stdcall prefix.

    let sig = sig.trim();

    // Check registry first
    if let Some(func_sig) = crate::registry::Registry::get().get_func(sig) {
        let args = func_sig.args.into_iter().map(|(_, t)| t).collect();
        return Ok((func_sig.ret, args, func_sig.conv));
    }

    // Simple inline parsing
    let paren_start = sig
        .find('(')
        .ok_or_else(|| LuaError::external(format!("Invalid signature: {}", sig)))?;
    let paren_end = sig
        .rfind(')')
        .ok_or_else(|| LuaError::external(format!("Invalid signature: {}", sig)))?;

    let before_paren = sig[..paren_start].trim();

    let (conv, ret_str) = if before_paren.contains("__stdcall") {
        (CallConv::Stdcall, before_paren.replace("__stdcall", ""))
    } else if before_paren.contains("WINAPI") {
        (CallConv::Stdcall, before_paren.replace("WINAPI", ""))
    } else {
        (CallConv::C, before_paren.to_string())
    };

    let ret = CType::parse(&ret_str).unwrap_or(CType::Int);

    let args_str = &sig[paren_start + 1..paren_end];
    let mut args = Vec::new();

    if !args_str.is_empty() && args_str != "void" {
        for arg in args_str.split(',') {
            let arg = arg.trim();
            if let Some(ctype) = CType::parse(arg) {
                args.push(ctype);
            }
        }
    }

    Ok((ret, args, conv))
}
