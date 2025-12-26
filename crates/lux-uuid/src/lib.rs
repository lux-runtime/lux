#![allow(clippy::cargo_common_metadata)]

//! UUID v4 and v7 generation for Lux

use lux_utils::TableBuilder;
use mlua::prelude::*;
use uuid::Uuid;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

/// Generate UUID v4 (random)
fn uuid_v4(_: &Lua, _: ()) -> LuaResult<String> {
    Ok(Uuid::new_v4().to_string())
}

/// Generate UUID v7 (timestamp-based, sortable)
fn uuid_v7(_: &Lua, _: ()) -> LuaResult<String> {
    Ok(Uuid::now_v7().to_string())
}

/// Validate UUID format
fn uuid_is_valid(_: &Lua, s: String) -> LuaResult<bool> {
    Ok(Uuid::parse_str(&s).is_ok())
}

/// Parse UUID to 16 bytes buffer
fn uuid_parse(lua: &Lua, s: String) -> LuaResult<LuaValue> {
    match Uuid::parse_str(&s) {
        Ok(uuid) => {
            let buf = lua.create_buffer(uuid.as_bytes().to_vec())?;
            Ok(LuaValue::Buffer(buf))
        }
        Err(_) => Ok(LuaValue::Nil),
    }
}

/// Format 16 bytes buffer to UUID string
fn uuid_format(_: &Lua, buf: mlua::Buffer) -> LuaResult<String> {
    let bytes = buf.to_vec();
    if bytes.len() != 16 {
        return Err(LuaError::external("Buffer must be 16 bytes"));
    }
    let mut arr = [0u8; 16];
    arr.copy_from_slice(&bytes);
    Ok(Uuid::from_bytes(arr).to_string())
}

/// Create the uuid module
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("v4", uuid_v4)?
        .with_function("v7", uuid_v7)?
        .with_function("isValid", uuid_is_valid)?
        .with_function("parse", uuid_parse)?
        .with_function("format", uuid_format)?
        .with_value("nil", "00000000-0000-0000-0000-000000000000")?
        .build_readonly()
}
