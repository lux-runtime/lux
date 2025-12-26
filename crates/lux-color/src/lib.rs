#![allow(clippy::cargo_common_metadata)]

//! Color3 type for Lux - RGB, HSV, Hex support

use lux_utils::TableBuilder;
use mlua::prelude::*;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Color3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color3 {
    #[inline]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            f64::from(r) / 255.0,
            f64::from(g) / 255.0,
            f64::from(b) / 255.0,
        )
    }

    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let (h, s, v) = (h.clamp(0.0, 1.0), s.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        if s == 0.0 {
            return Self::new(v, v, v);
        }
        let h = h * 6.0;
        let i = h.floor();
        let f = h - i;
        let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
        let (r, g, b) = match i as i32 % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        Self::new(r, g, b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let (r, g, b) = match hex.len() {
            6 => (
                u8::from_str_radix(&hex[0..2], 16).ok()?,
                u8::from_str_radix(&hex[2..4], 16).ok()?,
                u8::from_str_radix(&hex[4..6], 16).ok()?,
            ),
            3 => (
                u8::from_str_radix(&hex[0..1], 16).ok()? * 17,
                u8::from_str_radix(&hex[1..2], 16).ok()? * 17,
                u8::from_str_radix(&hex[2..3], 16).ok()? * 17,
            ),
            _ => return None,
        };
        Some(Self::from_rgb(r, g, b))
    }

    #[inline]
    pub fn lerp(&self, goal: &Self, a: f64) -> Self {
        let a = a.clamp(0.0, 1.0);
        Self::new(
            self.r + (goal.r - self.r) * a,
            self.g + (goal.g - self.g) * a,
            self.b + (goal.b - self.b) * a,
        )
    }

    pub fn to_hsv(&self) -> (f64, f64, f64) {
        let (max, min) = (
            self.r.max(self.g).max(self.b),
            self.r.min(self.g).min(self.b),
        );
        let d = max - min;
        let v = max;
        let s = if max == 0.0 { 0.0 } else { d / max };
        let h = if d == 0.0 {
            0.0
        } else if max == self.r {
            ((self.g - self.b) / d) % 6.0
        } else if max == self.g {
            (self.b - self.r) / d + 2.0
        } else {
            (self.r - self.g) / d + 4.0
        };
        ((h / 6.0).rem_euclid(1.0), s, v)
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8
        )
    }
}

impl LuaUserData for Color3 {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("R", |_, t| Ok(t.r));
        f.add_field_method_get("G", |_, t| Ok(t.g));
        f.add_field_method_get("B", |_, t| Ok(t.b));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_method("Lerp", |_, t, (g, a): (LuaUserDataRef<Self>, f64)| {
            Ok(t.lerp(&g, a))
        });
        m.add_method("ToHSV", |_, t, ()| {
            let (h, s, v) = t.to_hsv();
            Ok((h, s, v))
        });
        m.add_method("ToHex", |_, t, ()| Ok(t.to_hex()));
        m.add_meta_method(LuaMetaMethod::Eq, |_, t, o: LuaUserDataRef<Self>| {
            Ok(*t == *o)
        });
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!("{}, {}, {}", t.r, t.g, t.b))
        });
    }
}

pub fn create(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, (r, g, b): (f64, f64, f64)| {
            lua.create_userdata(Color3::new(r, g, b))
        })?
        .with_function("fromRGB", |lua, (r, g, b): (u8, u8, u8)| {
            lua.create_userdata(Color3::from_rgb(r, g, b))
        })?
        .with_function("fromHSV", |lua, (h, s, v): (f64, f64, f64)| {
            lua.create_userdata(Color3::from_hsv(h, s, v))
        })?
        .with_function("fromHex", |lua, hex: String| {
            Color3::from_hex(&hex)
                .map(|c| lua.create_userdata(c))
                .transpose()?
                .ok_or_else(|| LuaError::external("Invalid hex"))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}
