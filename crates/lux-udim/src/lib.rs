#![allow(clippy::cargo_common_metadata)]

//! UDim, UDim2, Rect, NumberRange types for Lux

use lux_utils::TableBuilder;
use mlua::prelude::*;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

// ============================================================================
// UDim
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct UDim {
    pub scale: f64,
    pub offset: f64,
}

impl UDim {
    #[inline]
    pub const fn new(scale: f64, offset: f64) -> Self {
        Self { scale, offset }
    }
}

impl LuaUserData for UDim {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("Scale", |_, t| Ok(t.scale));
        f.add_field_method_get("Offset", |_, t| Ok(t.offset));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!("{}, {}", t.scale, t.offset))
        });
        m.add_meta_method(LuaMetaMethod::Eq, |_, t, o: LuaUserDataRef<Self>| {
            Ok(*t == *o)
        });
    }
}

// ============================================================================
// UDim2
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    #[inline]
    pub const fn new(xs: f64, xo: f64, ys: f64, yo: f64) -> Self {
        Self {
            x: UDim::new(xs, xo),
            y: UDim::new(ys, yo),
        }
    }
    #[inline]
    pub const fn from_scale(xs: f64, ys: f64) -> Self {
        Self::new(xs, 0.0, ys, 0.0)
    }
    #[inline]
    pub const fn from_offset(xo: f64, yo: f64) -> Self {
        Self::new(0.0, xo, 0.0, yo)
    }
}

impl LuaUserData for UDim2 {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("X", |lua, t| lua.create_userdata(t.x));
        f.add_field_method_get("Y", |lua, t| lua.create_userdata(t.y));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!(
                "{{{}, {}, {}, {}}}",
                t.x.scale, t.x.offset, t.y.scale, t.y.offset
            ))
        });
        m.add_meta_method(LuaMetaMethod::Eq, |_, t, o: LuaUserDataRef<Self>| {
            Ok(*t == *o)
        });
    }
}

// ============================================================================
// Rect (uses inline Vector2-like)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Rect {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Rect {
    #[inline]
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x: min_x.min(max_x),
            min_y: min_y.min(max_y),
            max_x: min_x.max(max_x),
            max_y: min_y.max(max_y),
        }
    }
    #[inline]
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }
    #[inline]
    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
}

impl LuaUserData for Rect {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("Width", |_, t| Ok(t.width()));
        f.add_field_method_get("Height", |_, t| Ok(t.height()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!(
                "Rect({}, {}, {}, {})",
                t.min_x, t.min_y, t.max_x, t.max_y
            ))
        });
    }
}

// ============================================================================
// NumberRange
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct NumberRange {
    pub min: f64,
    pub max: f64,
}

impl NumberRange {
    #[inline]
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }
}

impl LuaUserData for NumberRange {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("Min", |_, t| Ok(t.min));
        f.add_field_method_get("Max", |_, t| Ok(t.max));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!("NumberRange({}, {})", t.min, t.max))
        });
    }
}

// ============================================================================
// Constructors
// ============================================================================

pub fn create_udim(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, (s, o): (f64, f64)| {
            lua.create_userdata(UDim::new(s, o))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}

pub fn create_udim2(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, (xs, xo, ys, yo): (f64, f64, f64, f64)| {
            lua.create_userdata(UDim2::new(xs, xo, ys, yo))
        })?
        .with_function("fromScale", |lua, (xs, ys): (f64, f64)| {
            lua.create_userdata(UDim2::from_scale(xs, ys))
        })?
        .with_function("fromOffset", |lua, (xo, yo): (f64, f64)| {
            lua.create_userdata(UDim2::from_offset(xo, yo))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}

pub fn create_rect(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function(
            "new",
            |lua, (min_x, min_y, max_x, max_y): (f64, f64, f64, f64)| {
                lua.create_userdata(Rect::new(min_x, min_y, max_x, max_y))
            },
        )?
        .build_readonly()
        .map(LuaValue::Table)
}

pub fn create_number_range(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, (min, max): (f64, f64)| {
            lua.create_userdata(NumberRange::new(min, max))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}
