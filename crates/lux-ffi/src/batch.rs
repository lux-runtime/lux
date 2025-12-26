//! FFI Batch Processing
//!
//! Provides batch processing of arrays through FFI functions
//! for zero per-element Lua allocation overhead.
//!
//! Note: This module uses raw pointers passed from Lua.
//! The caller is responsible for ensuring buffer validity.

#![allow(dead_code)]

use crate::call::CachedFunction;
use mlua::prelude::*;

/// Batch call a function on an array of doubles using raw pointers
///
/// Usage: ffi.batch(func, input_ptr, output_ptr, count)
/// - func: CachedFunction (from ffi.C.sin, etc.)
/// - input_ptr: raw pointer to input double array (from ffi.cast or CData)
/// - output_ptr: raw pointer to output double array
/// - count: number of elements to process
pub fn ffi_batch(
    _lua: &Lua,
    (func, input_ptr, output_ptr, count): (LuaValue, LuaValue, LuaValue, usize),
) -> LuaResult<usize> {
    // Get the cached function
    let func_ud = func
        .as_userdata()
        .ok_or_else(|| LuaError::external("Expected CachedFunction"))?;

    let cached = func_ud
        .borrow::<CachedFunction>()
        .map_err(|_| LuaError::external("Expected CachedFunction userdata"))?;

    // Get input pointer
    let input = match &input_ptr {
        LuaValue::LightUserData(ud) => ud.0 as *const f64,
        LuaValue::Integer(i) => *i as *const f64,
        _ => return Err(LuaError::external("Expected input pointer")),
    };

    // Get output pointer
    let output = match &output_ptr {
        LuaValue::LightUserData(ud) => ud.0 as *mut f64,
        LuaValue::Integer(i) => *i as *mut f64,
        _ => return Err(LuaError::external("Expected output pointer")),
    };

    if input.is_null() || output.is_null() {
        return Err(LuaError::external("Null pointer"));
    }

    unsafe {
        let func_ptr = cached.fn_ptr;
        let func: extern "C" fn(f64) -> f64 = std::mem::transmute(func_ptr);

        for i in 0..count {
            let val = *input.add(i);
            let result = func(val);
            *output.add(i) = result;
        }
    }

    Ok(count)
}

/// Batch call a two-argument function using raw pointers
pub fn ffi_batch2(
    _lua: &Lua,
    (func, input1_ptr, input2_ptr, output_ptr, count): (
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        usize,
    ),
) -> LuaResult<usize> {
    let func_ud = func
        .as_userdata()
        .ok_or_else(|| LuaError::external("Expected CachedFunction"))?;

    let cached = func_ud
        .borrow::<CachedFunction>()
        .map_err(|_| LuaError::external("Expected CachedFunction userdata"))?;

    let input1 = match &input1_ptr {
        LuaValue::LightUserData(ud) => ud.0 as *const f64,
        LuaValue::Integer(i) => *i as *const f64,
        _ => return Err(LuaError::external("Expected input1 pointer")),
    };

    let input2 = match &input2_ptr {
        LuaValue::LightUserData(ud) => ud.0 as *const f64,
        LuaValue::Integer(i) => *i as *const f64,
        _ => return Err(LuaError::external("Expected input2 pointer")),
    };

    let output = match &output_ptr {
        LuaValue::LightUserData(ud) => ud.0 as *mut f64,
        LuaValue::Integer(i) => *i as *mut f64,
        _ => return Err(LuaError::external("Expected output pointer")),
    };

    if input1.is_null() || input2.is_null() || output.is_null() {
        return Err(LuaError::external("Null pointer"));
    }

    unsafe {
        let func_ptr = cached.fn_ptr;
        let func: extern "C" fn(f64, f64) -> f64 = std::mem::transmute(func_ptr);

        for i in 0..count {
            let a = *input1.add(i);
            let b = *input2.add(i);
            let result = func(a, b);
            *output.add(i) = result;
        }
    }

    Ok(count)
}
