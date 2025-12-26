#![allow(clippy::cargo_common_metadata)]

//! High-performance Vector2 and Vector3 types for Lux
//! Optimized for FFI compatibility with #[repr(C)]

use lux_utils::TableBuilder;
use mlua::prelude::*;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

// ============================================================================
// Vector2
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::ZERO
        } else {
            Self::new(self.x / mag, self.y / mag)
        }
    }

    #[inline]
    pub fn lerp(&self, goal: &Self, alpha: f64) -> Self {
        let a = alpha.clamp(0.0, 1.0);
        Self::new(
            self.x + (goal.x - self.x) * a,
            self.y + (goal.y - self.y) * a,
        )
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y)
    }
}
impl std::ops::Sub for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y)
    }
}
impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s)
    }
}
impl std::ops::Div<f64> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, s: f64) -> Self {
        Self::new(self.x / s, self.y / s)
    }
}
impl std::ops::Neg for Vector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl LuaUserData for Vector2 {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("X", |_, t| Ok(t.x));
        f.add_field_method_get("Y", |_, t| Ok(t.y));
        f.add_field_method_get("Magnitude", |_, t| Ok(t.magnitude()));
        f.add_field_method_get("Unit", |lua, t| lua.create_userdata(t.unit()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_method("Lerp", |lua, t, (g, a): (LuaUserDataRef<Self>, f64)| {
            lua.create_userdata(t.lerp(&g, a))
        });
        m.add_method("Dot", |_, t, o: LuaUserDataRef<Self>| Ok(t.dot(&o)));
        m.add_method("Cross", |_, t, o: LuaUserDataRef<Self>| Ok(t.cross(&o)));
        m.add_meta_method(LuaMetaMethod::Eq, |_, t, o: LuaUserDataRef<Self>| {
            Ok(*t == *o)
        });
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!("{}, {}", t.x, t.y))
        });
        m.add_meta_method(LuaMetaMethod::Add, |lua, t, o: LuaUserDataRef<Self>| {
            lua.create_userdata(*t + *o)
        });
        m.add_meta_method(LuaMetaMethod::Sub, |lua, t, o: LuaUserDataRef<Self>| {
            lua.create_userdata(*t - *o)
        });
        m.add_meta_method(LuaMetaMethod::Mul, |lua, t, s: f64| {
            lua.create_userdata(*t * s)
        });
        m.add_meta_method(LuaMetaMethod::Div, |lua, t, s: f64| {
            lua.create_userdata(*t / s)
        });
        m.add_meta_method(LuaMetaMethod::Unm, |lua, t, ()| lua.create_userdata(-*t));
    }
}

// ============================================================================
// Vector3
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::ZERO
        } else {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    #[inline]
    pub fn lerp(&self, goal: &Self, alpha: f64) -> Self {
        let a = alpha.clamp(0.0, 1.0);
        Self::new(
            self.x + (goal.x - self.x) * a,
            self.y + (goal.y - self.y) * a,
            self.z + (goal.z - self.z) * a,
        )
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl std::ops::Sub for Vector3 {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
impl std::ops::Div<f64> for Vector3 {
    type Output = Self;
    #[inline]
    fn div(self, s: f64) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}
impl std::ops::Neg for Vector3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl LuaUserData for Vector3 {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("X", |_, t| Ok(t.x));
        f.add_field_method_get("Y", |_, t| Ok(t.y));
        f.add_field_method_get("Z", |_, t| Ok(t.z));
        f.add_field_method_get("Magnitude", |_, t| Ok(t.magnitude()));
        f.add_field_method_get("Unit", |lua, t| lua.create_userdata(t.unit()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_method("Lerp", |lua, t, (g, a): (LuaUserDataRef<Self>, f64)| {
            lua.create_userdata(t.lerp(&g, a))
        });
        m.add_method("Dot", |_, t, o: LuaUserDataRef<Self>| Ok(t.dot(&o)));
        m.add_method("Cross", |lua, t, o: LuaUserDataRef<Self>| {
            lua.create_userdata(t.cross(&o))
        });
        m.add_meta_method(LuaMetaMethod::Eq, |_, t, o: LuaUserDataRef<Self>| {
            Ok(*t == *o)
        });
        m.add_meta_method(LuaMetaMethod::ToString, |_, t, ()| {
            Ok(format!("{}, {}, {}", t.x, t.y, t.z))
        });
        m.add_meta_method(LuaMetaMethod::Add, |lua, t, o: LuaUserDataRef<Self>| {
            lua.create_userdata(*t + *o)
        });
        m.add_meta_method(LuaMetaMethod::Sub, |lua, t, o: LuaUserDataRef<Self>| {
            lua.create_userdata(*t - *o)
        });
        m.add_meta_method(LuaMetaMethod::Mul, |lua, t, s: f64| {
            lua.create_userdata(*t * s)
        });
        m.add_meta_method(LuaMetaMethod::Div, |lua, t, s: f64| {
            lua.create_userdata(*t / s)
        });
        m.add_meta_method(LuaMetaMethod::Unm, |lua, t, ()| lua.create_userdata(-*t));
    }
}

// ============================================================================
// Constructors
// ============================================================================

pub fn create_vector2(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua.clone())?
        .with_function("new", |lua, (x, y): (f64, f64)| {
            lua.create_userdata(Vector2::new(x, y))
        })?
        .with_value("zero", lua.create_userdata(Vector2::ZERO)?)?
        .with_value("one", lua.create_userdata(Vector2::ONE)?)?
        .build_readonly()
        .map(LuaValue::Table)
}

pub fn create_vector3(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua.clone())?
        .with_function("new", |lua, (x, y, z): (f64, f64, f64)| {
            lua.create_userdata(Vector3::new(x, y, z))
        })?
        .with_value("zero", lua.create_userdata(Vector3::ZERO)?)?
        .with_value("one", lua.create_userdata(Vector3::ONE)?)?
        .build_readonly()
        .map(LuaValue::Table)
}
