#![allow(clippy::cargo_common_metadata)]

//! Fast Base64 encoding/decoding for Lux

use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE},
};
use lux_utils::TableBuilder;
use mlua::prelude::*;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

/// Encode data to base64
fn encode(_: &Lua, data: LuaValue) -> LuaResult<String> {
    let bytes = match data {
        LuaValue::String(s) => s.as_bytes().to_vec(),
        LuaValue::Buffer(b) => b.to_vec(),
        _ => return Err(LuaError::external("Expected string or buffer")),
    };
    Ok(STANDARD.encode(&bytes))
}

/// Decode base64 to buffer
fn decode(lua: &Lua, encoded: String) -> LuaResult<mlua::Buffer> {
    let bytes = STANDARD
        .decode(&encoded)
        .map_err(|e| LuaError::external(format!("Invalid base64: {}", e)))?;
    lua.create_buffer(bytes)
}

/// Encode to URL-safe base64
fn encode_url(_: &Lua, data: LuaValue) -> LuaResult<String> {
    let bytes = match data {
        LuaValue::String(s) => s.as_bytes().to_vec(),
        LuaValue::Buffer(b) => b.to_vec(),
        _ => return Err(LuaError::external("Expected string or buffer")),
    };
    Ok(URL_SAFE.encode(&bytes))
}

/// Decode URL-safe base64 to buffer
fn decode_url(lua: &Lua, encoded: String) -> LuaResult<mlua::Buffer> {
    let bytes = URL_SAFE
        .decode(&encoded)
        .map_err(|e| LuaError::external(format!("Invalid base64: {}", e)))?;
    lua.create_buffer(bytes)
}

/// Create the base64 module
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("encode", encode)?
        .with_function("decode", decode)?
        .with_function("encodeUrl", encode_url)?
        .with_function("decodeUrl", decode_url)?
        .build_readonly()
}
