//! FFI Memory Management
//!
//! Handles allocation, pointers, and C data types.

use crate::callback::FfiCallback;
use crate::registry::Registry;
use crate::types::CType;
use mlua::prelude::*;
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::{CStr, c_void};
use std::ptr;

/// Common trait for C data wrappers
pub trait CData {
    fn ptr(&self) -> *mut c_void;
    fn type_name(&self) -> String;
}

/// Owned C memory (allocated by ffi.new)
pub struct CBox {
    ptr: *mut c_void,
    size: usize,
    pub ctype: CType,
    owned: bool, // If true, we free on drop
}

impl CBox {
    pub fn new(ctype: CType) -> Self {
        let size = ctype.size().max(1); // logical size 0 -> 1 byte
        let align = ctype.align().max(1);

        unsafe {
            let layout = Layout::from_size_align(size, align).unwrap();
            let ptr = alloc(layout).cast();
            ptr::write_bytes(ptr, 0, size); // Zero initialize by default

            Self {
                ptr,
                size,
                ctype,
                owned: true,
            }
        }
    }

    pub fn from_raw(ptr: *mut c_void, ctype: CType, owned: bool) -> Self {
        Self {
            ptr,
            size: ctype.size(),
            ctype,
            owned,
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for CBox {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            let size = self.size.max(1);
            let align = self.ctype.align().max(1);
            unsafe {
                let layout = Layout::from_size_align(size, align).unwrap();
                dealloc(self.ptr.cast(), layout);
            }
        }
    }
}

impl CData for CBox {
    fn ptr(&self) -> *mut c_void {
        self.ptr
    }
    fn type_name(&self) -> String {
        format!("{:?}", self.ctype)
    }
}

impl LuaUserData for CBox {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("ptr", |_, this| Ok(LuaLightUserData(this.ptr)));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Pointer arithmetic and dereference
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            // Handle array indexing [0] etc
            let idx = if let LuaValue::Integer(i) = key {
                Some(i as isize)
            } else if let LuaValue::Number(n) = key {
                Some(n as isize)
            } else {
                None
            };

            if let Some(idx) = idx {
                let (target_type, stride) = if let CType::Array(elem, _) = &this.ctype {
                    (elem.as_ref(), elem.size())
                } else if let CType::Pointer(inner) = &this.ctype {
                    // If pointer to T, stride is size of T
                    if let Some(t) = inner.as_ref() {
                        (t.as_ref(), t.size())
                    } else {
                        // Void pointer or similar
                        return Ok(LuaValue::Nil);
                    }
                } else {
                    // Treat as pointer to self (size of self) - akin to &x[idx]
                    (&this.ctype, this.ctype.size())
                };

                let offset = (idx as isize) * (stride as isize);
                let ptr = unsafe { (this.ptr as *mut u8).offset(offset) as *mut c_void };

                // Return reference/value depending on type
                return unsafe { c_to_lua_at_ptr(lua, target_type, ptr) };
            } else if let LuaValue::String(s) = key {
                // Handle struct field access
                let field_name = s.to_str().map_err(LuaError::external)?;

                // Helper to resolve struct name handling pointers
                let target_type = if let CType::Pointer(inner) = &this.ctype {
                    // Check if inner is struct/union
                    inner.as_ref().map(|t| t.as_ref())
                } else {
                    Some(&this.ctype)
                };

                if let Some(match_type) = target_type {
                    if let CType::Struct(struct_name) = match_type {
                        let reg = Registry::get();
                        if let Some(def) = reg.get_struct(struct_name) {
                            if let Some(field) = def.field(&field_name) {
                                let offset = field.offset as isize;
                                let ptr = unsafe { this.ptr.offset(offset) };
                                return unsafe { c_to_lua_at_ptr(lua, &field.ctype, ptr) };
                            }
                        }
                    } else if let CType::Union(union_name) = match_type {
                        // Union logic (same as struct but all offsets 0 usually, or defined in def)
                        // Our StructDef handles unions too with offsets
                        let reg = Registry::get();
                        if let Some(def) = reg.get_struct(union_name) {
                            if let Some(field) = def.field(&field_name) {
                                let offset = field.offset as isize;
                                let ptr = unsafe { this.ptr.offset(offset) };
                                return unsafe { c_to_lua_at_ptr(lua, &field.ctype, ptr) };
                            }
                        }
                    }
                }
            }

            Ok(LuaValue::Nil)
        });

        methods.add_meta_method(
            LuaMetaMethod::NewIndex,
            |_, this, (key, value): (LuaValue, LuaValue)| {
                let idx = if let LuaValue::Integer(i) = key {
                    Some(i as isize)
                } else if let LuaValue::Number(n) = key {
                    Some(n as isize)
                } else {
                    None
                };

                if let Some(idx) = idx {
                    let (target_type, stride) = if let CType::Array(elem, _) = &this.ctype {
                        (elem.as_ref(), elem.size())
                    } else if let CType::Pointer(inner) = &this.ctype {
                        if let Some(t) = inner.as_ref() {
                            (t.as_ref(), t.size())
                        } else {
                            return Err(LuaError::external("Cannot index void pointer"));
                        }
                    } else {
                        (&this.ctype, this.ctype.size())
                    };

                    let offset = (idx as isize) * (stride as isize);
                    let ptr = unsafe { (this.ptr as *mut u8).offset(offset) as *mut c_void };
                    // Set value
                    return unsafe { lua_to_c_at_ptr(target_type, ptr, value) }
                        .map_err(LuaError::external);
                } else if let LuaValue::String(s) = key {
                    // Handle struct field assignment
                    let field_name = s.to_str().map_err(LuaError::external)?;

                    let target_type = if let CType::Pointer(inner) = &this.ctype {
                        inner.as_ref().map(|t| t.as_ref())
                    } else {
                        Some(&this.ctype)
                    };

                    if let Some(match_type) = target_type {
                        let reg = Registry::get();
                        // Handle Struct and Union
                        let struct_name = match match_type {
                            CType::Struct(n) => Some(n),
                            CType::Union(n) => Some(n),
                            _ => None,
                        };

                        if let Some(name) = struct_name {
                            if let Some(def) = reg.get_struct(name) {
                                if let Some(field) = def.field(&field_name) {
                                    let offset = field.offset as isize;
                                    let ptr = unsafe { this.ptr.offset(offset) };
                                    return unsafe { lua_to_c_at_ptr(&field.ctype, ptr, value) }
                                        .map_err(LuaError::external);
                                }
                            }
                        }
                    }
                }
                Ok(())
            },
        );

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("cdata<{:?}>: {:p}", this.ctype, this.ptr))
        });

        methods.add_meta_method(LuaMetaMethod::Call, |lua, this, args: LuaMultiValue| {
            // Check if it is a function pointer or function

            let func_ptr = match &this.ctype {
                CType::Pointer(inner) => {
                    // Fix Option dereferencing
                    if let Some(inner_type) = inner.as_ref() {
                        if let CType::Function(_) = inner_type.as_ref() {
                            unsafe { *(this.ptr as *const usize) }
                        } else {
                            return Err(LuaError::external(
                                "Attempt to call non-function pointer cdata",
                            ));
                        }
                    } else {
                        return Err(LuaError::external("Attempt to call void pointer cdata"));
                    }
                }
                CType::Function(_) => {
                    this.ptr as usize // The cdata ptr IS the function address (unlikely for CBox but possible)
                }
                _ => return Err(LuaError::external("Attempt to call non-function cdata")),
            };

            if func_ptr == 0 {
                return Err(LuaError::external("Attempt to call null function pointer"));
            }

            // We need to pass the function address + arguments to ffi_call
            // But ffi_call takes a LuaUserData (CLib) usually?
            // No, call::ffi_call takes (lua, args). It expects the first arg to be the function name or something?
            // Actually call::ffi_call logic (lines 148+ in call.rs) does:
            // let func_name = ...
            // let lib = ...

            // We need a lower level "call_fn_ptr" function in call.rs that takes (ptr, sig, args).
            // Let's look at call::ffi_call again. It resolves symbol then calls internal invoke?

            // Re-use call::ffi_call_ptr if it exists, or create one.
            // For now, let's assume we can export a helper from call.rs.

            unsafe { crate::call::ffi_call_ptr(lua.clone(), func_ptr, &this.ctype, args) }
        });
    }
}

// Helpers for reading/writing memory at ptr based on type

/// Convert C value at pointer to Lua value
/// Creates proper CBox userdata for aggregate types and pointers
unsafe fn c_to_lua_at_ptr(lua: &Lua, ctype: &CType, ptr: *mut c_void) -> LuaResult<LuaValue> {
    if ptr.is_null() {
        return Ok(LuaValue::Nil);
    }

    match ctype {
        CType::Void => Ok(LuaValue::Nil),

        CType::Bool => Ok(LuaValue::Boolean(*(ptr as *const i8) != 0)),

        CType::Char | CType::Int8 => Ok(LuaValue::Integer(*(ptr as *const i8) as i64)),
        CType::UChar | CType::UInt8 => Ok(LuaValue::Integer(*(ptr as *const u8) as i64)),

        CType::Short | CType::Int16 => Ok(LuaValue::Integer(*(ptr as *const i16) as i64)),
        CType::UShort | CType::UInt16 | CType::WChar => {
            Ok(LuaValue::Integer(*(ptr as *const u16) as i64))
        }

        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            Ok(LuaValue::Integer(*(ptr as *const i32) as i64))
        }
        CType::UInt | CType::UInt32 => Ok(LuaValue::Integer(*(ptr as *const u32) as i64)),

        CType::Long | CType::LongLong | CType::Int64 => Ok(LuaValue::Integer(*(ptr as *const i64))),
        CType::ULong | CType::ULongLong | CType::UInt64 => {
            // u64 may overflow i64, return as number for large values
            let val = *(ptr as *const u64);
            if val <= i64::MAX as u64 {
                Ok(LuaValue::Integer(val as i64))
            } else {
                Ok(LuaValue::Number(val as f64))
            }
        }

        CType::Float => Ok(LuaValue::Number(*(ptr as *const f32) as f64)),
        CType::Double => Ok(LuaValue::Number(*(ptr as *const f64))),

        CType::Pointer(_) => {
            let val = *(ptr as *const *mut c_void);
            if val.is_null() {
                return Ok(LuaValue::Nil);
            }

            // Create a non-owned CBox wrapping this pointer value
            // The CBox represents the pointer itself, not what it points to
            let cbox = CBox::from_raw(val, ctype.clone(), false);
            lua.create_userdata(cbox).map(LuaValue::UserData)
        }

        CType::Array(_elem_type, _count) => {
            // For arrays, create a CBox referencing this memory
            let cbox = CBox::from_raw(ptr, ctype.clone(), false);
            lua.create_userdata(cbox).map(LuaValue::UserData)
        }

        CType::Struct(_name) | CType::Union(_name) => {
            // Create a non-owned CBox referencing this struct/union
            let cbox = CBox::from_raw(ptr, ctype.clone(), false);
            lua.create_userdata(cbox).map(LuaValue::UserData)
        }

        CType::Function(_) => {
            // Function type - return as callable CBox
            let cbox = CBox::from_raw(ptr, ctype.clone(), false);
            lua.create_userdata(cbox).map(LuaValue::UserData)
        }

        CType::GUID => {
            // GUID is a 16-byte struct, return as CBox
            let cbox = CBox::from_raw(ptr, ctype.clone(), false);
            lua.create_userdata(cbox).map(LuaValue::UserData)
        }
    }
}

/// Write Lua value to C memory at pointer
unsafe fn lua_to_c_at_ptr(ctype: &CType, ptr: *mut c_void, value: LuaValue) -> Result<(), String> {
    if ptr.is_null() {
        return Err("Cannot write to null pointer".to_string());
    }

    match ctype {
        CType::Void => Ok(()),

        CType::Bool => {
            let b = match &value {
                LuaValue::Boolean(b) => *b,
                LuaValue::Nil => false,
                LuaValue::Integer(i) => *i != 0,
                LuaValue::Number(n) => *n != 0.0,
                _ => true, // truthy
            };
            *(ptr as *mut i8) = if b { 1 } else { 0 };
            Ok(())
        }

        CType::Char | CType::Int8 => {
            *(ptr as *mut i8) = value.as_i32().unwrap_or(0) as i8;
            Ok(())
        }
        CType::UChar | CType::UInt8 => {
            *(ptr as *mut u8) = value.as_i32().unwrap_or(0) as u8;
            Ok(())
        }

        CType::Short | CType::Int16 => {
            *(ptr as *mut i16) = value.as_i32().unwrap_or(0) as i16;
            Ok(())
        }
        CType::UShort | CType::UInt16 | CType::WChar => {
            *(ptr as *mut u16) = value.as_i32().unwrap_or(0) as u16;
            Ok(())
        }

        CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => {
            *(ptr as *mut i32) = value.as_i32().unwrap_or(0);
            Ok(())
        }
        CType::UInt | CType::UInt32 => {
            *(ptr as *mut u32) = value.as_u32().unwrap_or(0);
            Ok(())
        }

        CType::Long | CType::LongLong | CType::Int64 => {
            let v = match &value {
                LuaValue::Integer(i) => *i,
                LuaValue::Number(n) => *n as i64,
                LuaValue::LightUserData(ud) => ud.0 as i64,
                LuaValue::UserData(ud) => {
                    if let Ok(cbox) = ud.borrow::<CBox>() {
                        cbox.ptr() as i64
                    } else if let Ok(cb) = ud.borrow::<FfiCallback>() {
                        cb.as_ptr() as i64
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            *(ptr as *mut i64) = v;
            Ok(())
        }
        CType::ULong | CType::ULongLong | CType::UInt64 => {
            let v = match &value {
                LuaValue::Integer(i) => *i as u64,
                LuaValue::Number(n) => *n as u64,
                LuaValue::LightUserData(ud) => ud.0 as u64,
                LuaValue::UserData(ud) => {
                    if let Ok(cbox) = ud.borrow::<CBox>() {
                        cbox.ptr() as u64
                    } else if let Ok(cb) = ud.borrow::<FfiCallback>() {
                        cb.as_ptr() as u64
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            *(ptr as *mut u64) = v;
            Ok(())
        }

        CType::Float => {
            *(ptr as *mut f32) = value.as_f64().unwrap_or(0.0) as f32;
            Ok(())
        }
        CType::Double => {
            *(ptr as *mut f64) = value.as_f64().unwrap_or(0.0);
            Ok(())
        }

        CType::Pointer(_) | CType::Function(_) => {
            let p = match &value {
                LuaValue::Nil => ptr::null_mut(),
                LuaValue::LightUserData(ud) => ud.0,
                LuaValue::Integer(i) => *i as *mut c_void,
                LuaValue::UserData(ud) => {
                    if let Ok(cbox) = ud.borrow::<CBox>() {
                        cbox.ptr()
                    } else if let Ok(cb) = ud.borrow::<FfiCallback>() {
                        cb.as_ptr()
                    } else {
                        ptr::null_mut()
                    }
                }
                // String -> char* requires special handling (caller must keep string alive)
                LuaValue::String(s) => s.as_bytes().as_ptr() as *mut c_void,
                _ => ptr::null_mut(),
            };
            *(ptr as *mut *mut c_void) = p;
            Ok(())
        }

        CType::Array(elem_type, count) => {
            // Array assignment from table
            if let LuaValue::Table(t) = &value {
                for i in 0..*count {
                    if let Ok(v) = t.get::<LuaValue>(i as i64 + 1) {
                        let offset = (i * elem_type.size()) as isize;
                        let elem_ptr = ptr.offset(offset);
                        lua_to_c_at_ptr(elem_type, elem_ptr, v)?;
                    }
                }
                Ok(())
            } else if let LuaValue::UserData(ud) = &value {
                // Copy from another CBox
                if let Ok(src) = ud.borrow::<CBox>() {
                    let size = ctype.size();
                    ptr::copy_nonoverlapping(src.ptr() as *const u8, ptr as *mut u8, size);
                }
                Ok(())
            } else {
                Err("Array assignment requires table or cdata".to_string())
            }
        }

        CType::Struct(_name) | CType::Union(_name) => {
            // Struct/union assignment from table or cdata
            if let LuaValue::Table(t) = &value {
                let reg = Registry::get();
                if let Some(def) = reg.get_struct(_name) {
                    for field in &def.fields {
                        if let Ok(v) = t.get::<LuaValue>(field.name.as_str()) {
                            let field_ptr = ptr.offset(field.offset as isize);
                            lua_to_c_at_ptr(&field.ctype, field_ptr, v)?;
                        }
                    }
                }
                Ok(())
            } else if let LuaValue::UserData(ud) = &value {
                // Copy from another CBox
                if let Ok(src) = ud.borrow::<CBox>() {
                    let size = ctype.size();
                    ptr::copy_nonoverlapping(src.ptr() as *const u8, ptr as *mut u8, size);
                }
                Ok(())
            } else {
                Err("Struct/union assignment requires table or cdata".to_string())
            }
        }

        CType::GUID => {
            // GUID assignment from table {Data1, Data2, Data3, Data4} or cdata
            if let LuaValue::UserData(ud) = &value {
                if let Ok(src) = ud.borrow::<CBox>() {
                    ptr::copy_nonoverlapping(src.ptr() as *const u8, ptr as *mut u8, 16);
                }
            }
            Ok(())
        }
    }
}

// Module Functions

pub fn ffi_new(lua: &Lua, args: LuaMultiValue) -> LuaResult<LuaValue> {
    let args_vec: Vec<LuaValue> = args.into_iter().collect();
    if args_vec.is_empty() {
        return Err(LuaError::external("ffi.new expects at least 1 argument"));
    }

    let type_name = match &args_vec[0] {
        LuaValue::String(s) => s.to_str()?.to_string(),
        _ => return Err(LuaError::external("ffi.new: type must be a string")),
    };

    let ctype = CType::parse(&type_name)
        .ok_or_else(|| LuaError::external(format!("Unknown type: {}", type_name)))?;

    // Allocate
    if ctype.size() == 0 {
        return Err(LuaError::external(format!(
            "cannot allocate incomplete type '{}'",
            type_name
        )));
    }
    let cbox = CBox::new(ctype.clone());

    // Initialize if init value provided
    if args_vec.len() > 1 {
        // TODO: array initialization, struct initialization
        // For now, simple primitive init
        unsafe {
            let _ = lua_to_c_at_ptr(&ctype, cbox.ptr, args_vec[1].clone());
        }
    }

    lua.create_userdata(cbox).map(LuaValue::UserData)
}

pub fn ffi_cast(lua: &Lua, ctype_str: String, value: LuaValue) -> LuaResult<LuaValue> {
    let ctype = CType::parse(&ctype_str)
        .ok_or_else(|| LuaError::external(format!("Unknown type: {}", ctype_str)))?;

    let ptr = match value {
        LuaValue::LightUserData(ud) => ud.0,
        LuaValue::Integer(i) => i as *mut c_void,
        LuaValue::UserData(ud) => {
            if let Ok(b) = ud.borrow::<CBox>() {
                b.ptr
            } else {
                ptr::null_mut()
            }
        }
        _ => ptr::null_mut(),
    };

    // Create a non-owned CBox (reference)
    let cbox = CBox::from_raw(ptr, ctype, false);
    lua.create_userdata(cbox).map(LuaValue::UserData)
}

pub fn ffi_string(lua: &Lua, args: LuaMultiValue) -> LuaResult<LuaValue> {
    let args_vec: Vec<LuaValue> = args.into_iter().collect();
    if args_vec.is_empty() {
        return Ok(LuaValue::Nil);
    }

    let ptr = match &args_vec[0] {
        LuaValue::LightUserData(ud) => ud.0,
        LuaValue::Integer(i) => *i as *mut c_void,
        LuaValue::UserData(ud) => {
            if let Ok(b) = ud.borrow::<CBox>() {
                b.ptr
            } else {
                return Ok(LuaValue::Nil);
            }
        }
        _ => return Ok(LuaValue::Nil),
    };

    if ptr.is_null() {
        return Ok(LuaValue::Nil);
    }

    let len = if args_vec.len() > 1 {
        match args_vec[1] {
            LuaValue::Integer(i) => Some(i as usize),
            _ => None,
        }
    } else {
        None
    };

    unsafe {
        if let Some(l) = len {
            let slice = std::slice::from_raw_parts(ptr as *const u8, l);
            Ok(LuaValue::String(lua.create_string(slice)?))
        } else {
            let cstr = CStr::from_ptr(ptr as *const i8);
            Ok(LuaValue::String(lua.create_string(cstr.to_bytes())?))
        }
    }
}

pub fn ffi_copy(_lua: &Lua, (dst, src, len): (LuaValue, LuaValue, Option<usize>)) -> LuaResult<()> {
    let dst_ptr = get_ptr_from_value(&dst)?;
    let src_ptr = get_ptr_from_value(&src)?;

    if dst_ptr.is_null() || src_ptr.is_null() {
        return Err(LuaError::external("ffi.copy: null pointer"));
    }

    // If len is not provided, try to assume string length if src is string?
    // But src is userdata/ptr here usually.
    // LuaJIT allows ffi.copy(dst, str) where len is str len.
    // We strictly take len for now or default?
    let count = len.unwrap_or(0);

    // Check if src is a lua string
    if let LuaValue::String(s) = src {
        unsafe {
            let bytes = s.as_bytes();
            let copy_len = len.unwrap_or(bytes.len());
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst_ptr as *mut u8, copy_len);
        }
        return Ok(());
    }

    if count == 0 {
        return Ok(());
    }

    unsafe {
        ptr::copy_nonoverlapping(src_ptr as *const u8, dst_ptr as *mut u8, count);
    }
    Ok(())
}

pub fn ffi_fill(_lua: &Lua, (dst, len, byte): (LuaValue, usize, Option<u8>)) -> LuaResult<()> {
    let dst_ptr = get_ptr_from_value(&dst)?;
    if dst_ptr.is_null() {
        return Err(LuaError::external("ffi.fill: null pointer"));
    }
    let val = byte.unwrap_or(0);
    unsafe {
        ptr::write_bytes(dst_ptr as *mut u8, val, len);
    }
    Ok(())
}

pub fn get_ptr_from_value(val: &LuaValue) -> LuaResult<*mut c_void> {
    match val {
        LuaValue::LightUserData(ud) => Ok(ud.0),
        LuaValue::Integer(i) => Ok(*i as *mut c_void),
        LuaValue::UserData(ud) => {
            if let Ok(b) = ud.borrow::<CBox>() {
                Ok(b.ptr)
            } else if let Ok(cb) = ud.borrow::<FfiCallback>() {
                Ok(cb.as_ptr())
            } else {
                Ok(ptr::null_mut())
            }
        }
        _ => Ok(ptr::null_mut()),
    }
}

pub fn ffi_gc(_lua: &Lua, (cdata, finalizer): (LuaValue, LuaValue)) -> LuaResult<LuaValue> {
    // LuaJIT allows attaching a finalizer to cdata.
    // With mlua, we can use set_user_value to store the finalizer.
    // The finalizer would need to be called in Drop, which is complex.
    // For now, store it but note: actual finalization needs CBox to check user_value on drop.
    if let LuaValue::UserData(ref ud) = cdata {
        if let LuaValue::Function(f) = finalizer {
            let _ = ud.set_user_value(f);
        } else if finalizer.is_nil() {
            let _ = ud.set_user_value(LuaNil);
        }
    }
    Ok(cdata)
}

pub fn ffi_metatype(_ctype: &str, _mt: LuaTable) -> LuaResult<()> {
    // TODO: Store metatable association for this ctype in registry
    // Would require modifying CBox creation to apply metatable
    Ok(())
}

pub fn ffi_istype(ctype_str: &str, value: LuaValue) -> LuaResult<bool> {
    if let LuaValue::UserData(ud) = value {
        if let Ok(cbox) = ud.borrow::<CBox>() {
            if let Some(expected) = CType::parse(ctype_str) {
                return Ok(cbox.ctype == expected);
            }
        }
    }
    Ok(false)
}

/// CType wrapper for ffi.typeof - allows using ctype as constructor
pub struct CTypeWrapper {
    pub ctype: CType,
    pub name: String,
}

impl LuaUserData for CTypeWrapper {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.ctype.size()));
        fields.add_field_method_get("align", |_, this| Ok(this.ctype.align()));
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Call metamethod: allows ctype(init) syntax
        methods.add_meta_method(LuaMetaMethod::Call, |lua, this, args: LuaMultiValue| {
            // Create a new CBox with this type
            let cbox = CBox::new(this.ctype.clone());

            // Initialize if init value provided
            let args_vec: Vec<LuaValue> = args.into_iter().collect();
            if !args_vec.is_empty() {
                unsafe {
                    let _ = lua_to_c_at_ptr(&this.ctype, cbox.ptr, args_vec[0].clone());
                }
            }

            lua.create_userdata(cbox).map(LuaValue::UserData)
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("ctype<{}>", this.name))
        });
    }
}

pub fn ffi_typeof(lua: &Lua, ctype_str: String) -> LuaResult<LuaValue> {
    let ctype = CType::parse(&ctype_str)
        .ok_or_else(|| LuaError::external(format!("Unknown type: {}", ctype_str)))?;

    let wrapper = CTypeWrapper {
        ctype,
        name: ctype_str,
    };

    lua.create_userdata(wrapper).map(LuaValue::UserData)
}

pub fn ffi_offsetof(ctype_str: &str, field: &str) -> LuaResult<LuaValue> {
    let reg = Registry::get();

    // Strip "struct " prefix if present
    let name = ctype_str.strip_prefix("struct ").unwrap_or(ctype_str);

    if let Some(def) = reg.get_struct(name) {
        if let Some(f) = def.field(field) {
            return Ok(LuaValue::Integer(f.offset as i64));
        }
        return Err(LuaError::external(format!(
            "Field '{}' not found in struct '{}'",
            field, name
        )));
    }

    Err(LuaError::external(format!("Struct '{}' not found", name)))
}

/// Get address of a field in a CBox struct
/// Returns a LightUserData pointer that can be passed to C functions
pub fn ffi_addressof(_lua: &Lua, (cdata, field): (LuaValue, String)) -> LuaResult<LuaValue> {
    let ud = match cdata {
        LuaValue::UserData(ud) => ud,
        _ => {
            return Err(LuaError::external(
                "ffi.addressof: first argument must be cdata",
            ));
        }
    };

    let cbox = ud
        .borrow::<CBox>()
        .map_err(|_| LuaError::external("ffi.addressof: not a cdata"))?;

    // Get struct type
    let struct_name = match &cbox.ctype {
        CType::Struct(n) => n,
        CType::Union(n) => n,
        _ => {
            return Err(LuaError::external(
                "ffi.addressof: cdata must be a struct or union",
            ));
        }
    };

    let reg = Registry::get();
    let def = reg
        .get_struct(struct_name)
        .ok_or_else(|| LuaError::external(format!("Struct '{}' not found", struct_name)))?;

    let field_def = def.field(&field).ok_or_else(|| {
        LuaError::external(format!(
            "Field '{}' not found in struct '{}'",
            field, struct_name
        ))
    })?;

    // Calculate pointer to field
    let field_ptr = unsafe { cbox.ptr.offset(field_def.offset as isize) };

    // Return as LightUserData which can be passed to C functions expecting int* etc
    Ok(LuaValue::LightUserData(LuaLightUserData(field_ptr)))
}
