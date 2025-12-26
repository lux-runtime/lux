#![allow(unsafe_op_in_unsafe_fn)]

//! Lux FFI Module
//!
//! Re-implements LuaJIT FFI for Lux using modular architecture.
#[allow(dead_code)]
use libloading::Library;
use mlua::prelude::*;
use std::sync::Arc;

pub mod batch;
pub mod call;
pub mod callback;
pub mod memory;
pub mod parser;
pub mod registry;
pub mod types;

use types::CType;

/// The FFI Module Entry Point
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    // ffi.cdef(decl)
    exports.set(
        "cdef",
        lua.create_function(|_, decl: String| {
            parser::parse_cdef(&decl).map_err(LuaError::external)?;
            Ok(())
        })?,
    )?;

    // ffi.new(type, init...) - Implemented in memory.rs
    exports.set("new", lua.create_function(memory::ffi_new)?)?;

    // ffi.cast(type, val) - Implemented in memory.rs
    exports.set(
        "cast",
        lua.create_function(|lua, (type_name, value): (String, LuaValue)| {
            memory::ffi_cast(lua, type_name, value)
        })?,
    )?;

    // ffi.typeof(type) - Implemented in memory.rs
    exports.set("typeof", lua.create_function(memory::ffi_typeof)?)?;

    // ffi.sizeof(type)
    exports.set(
        "sizeof",
        lua.create_function(|_, type_name: String| {
            if let Some(ctype) = CType::parse(&type_name) {
                Ok(ctype.size())
            } else if let Some(def) = registry::Registry::get().get_struct(&type_name) {
                Ok(def.size)
            } else {
                Err(LuaError::external(format!("Unknown type: {}", type_name)))
            }
        })?,
    )?;

    // ffi.alignof(type)
    exports.set(
        "alignof",
        lua.create_function(|_, type_name: String| {
            if let Some(ctype) = CType::parse(&type_name) {
                Ok(ctype.align())
            } else if let Some(def) = registry::Registry::get().get_struct(&type_name) {
                Ok(def.align)
            } else {
                Err(LuaError::external(format!("Unknown type: {}", type_name)))
            }
        })?,
    )?;

    // ffi.offsetof(type, field)
    exports.set(
        "offsetof",
        lua.create_function(|_, (type_name, field): (String, String)| {
            memory::ffi_offsetof(&type_name, &field)
        })?,
    )?;

    // ffi.addressof(cdata, field) - Special ext
    exports.set("addressof", lua.create_function(memory::ffi_addressof)?)?;

    // ffi.string(ptr, len)
    exports.set("string", lua.create_function(memory::ffi_string)?)?;

    // ffi.copy(dst, src, len)
    exports.set("copy", lua.create_function(memory::ffi_copy)?)?;

    // ffi.fill(dst, len, val)
    exports.set("fill", lua.create_function(memory::ffi_fill)?)?;

    // ffi.istype(type, val)
    exports.set(
        "istype",
        lua.create_function(|_, (type_name, val): (String, LuaValue)| {
            memory::ffi_istype(&type_name, val)
        })?,
    )?;

    // ffi.metatype(type, mt)
    exports.set(
        "metatype",
        lua.create_function(|_, (type_name, mt): (String, LuaTable)| {
            memory::ffi_metatype(&type_name, mt)
        })?,
    )?;

    // ffi.gc(cdata, finalizer)
    exports.set("gc", lua.create_function(memory::ffi_gc)?)?;

    // ffi.load(name)
    exports.set(
        "load",
        lua.create_function(move |_lua, name: String| {
            let load_name = if cfg!(windows) && !name.ends_with(".dll") {
                format!("{}.dll", name)
            } else if cfg!(target_os = "linux") && !name.contains(".so") {
                format!("lib{}.so", name)
            } else {
                name.clone()
            };

            let lib = unsafe { Library::new(&load_name) }.map_err(|e| {
                LuaError::external(format!("Failed to load library '{}': {}", load_name, e))
            })?;

            Ok(SmartLibrary {
                lib: Arc::new(lib),
                name,
            })
        })?,
    )?;

    // ffi.callback(sig, func)
    exports.set(
        "callback",
        lua.create_function(|lua, (sig, func): (String, LuaFunction)| {
            callback::create_callback(lua, &sig, func)
        })?,
    )?;

    // ffi.batch(func, input_ptr, output_ptr, count) - Batch processing
    exports.set(
        "batch",
        lua.create_function(|lua, args: (LuaValue, LuaValue, LuaValue, usize)| {
            batch::ffi_batch(&lua, args)
        })?,
    )?;

    // ffi.batch2(func, input1_ptr, input2_ptr, output_ptr, count) - Two-arg batch
    exports.set(
        "batch2",
        lua.create_function(
            |lua, args: (LuaValue, LuaValue, LuaValue, LuaValue, usize)| {
                batch::ffi_batch2(&lua, args)
            },
        )?,
    )?;

    // ffi.C
    let default_lib_name = if cfg!(windows) {
        "msvcrt.dll"
    } else {
        "libc.so.6"
    };
    if let Ok(lib) = unsafe { Library::new(default_lib_name) } {
        exports.set(
            "C",
            SmartLibrary {
                lib: Arc::new(lib),
                name: "C".to_string(),
            },
        )?;
    }

    // OS and Arch info
    exports.set("os", std::env::consts::OS)?;
    exports.set("arch", std::env::consts::ARCH)?;

    Ok(exports)
}

/// Helper for Smart Library wrapper
#[derive(Clone)]
pub struct SmartLibrary {
    lib: Arc<Library>,
    name: String,
}

impl LuaUserData for SmartLibrary {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, func_name: String| {
            // 1. Check if function declared in registry
            if let Some(sig) = registry::Registry::get().get_func(&func_name) {
                // Resolve symbol once
                let func_ptr = unsafe {
                    let sym: libloading::Symbol<*const std::ffi::c_void> =
                        this.lib.get(func_name.as_bytes()).map_err(|e| {
                            LuaError::external(format!("Symbol '{}' not found: {}", func_name, e))
                        })?;
                    *sym as usize
                };

                // Create CachedFunction with pre-prepared CIF
                let cached =
                    call::CachedFunction::new(func_ptr, sig).map_err(LuaError::external)?;

                return Ok(LuaValue::UserData(lua.create_userdata(cached)?));
            }

            // 2. Constants / Global Variables? (TODO)

            Ok(LuaValue::Nil)
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("ffi.load('{}')", this.name))
        });
    }
}

pub fn typedefs() -> String {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau")).to_string()
}
