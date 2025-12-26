//! Roblox-compatible data types for Lux
//!
//! Provides: Color3, Vector2, UDim, UDim2, Rect

use lux_utils::TableBuilder;
use mlua::prelude::*;

// ============================================================================
// Color3
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color3 {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            f64::from(r) / 255.0,
            f64::from(g) / 255.0,
            f64::from(b) / 255.0,
        )
    }

    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let h = h.clamp(0.0, 1.0);
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        if s == 0.0 {
            return Self::new(v, v, v);
        }

        let h = h * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

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

        let (r, g, b) = if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b)
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            (r, g, b)
        } else {
            return None;
        };

        Some(Self::from_rgb(r, g, b))
    }

    pub fn lerp(&self, goal: &Color3, alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        Self::new(
            self.r + (goal.r - self.r) * alpha,
            self.g + (goal.g - self.g) * alpha,
            self.b + (goal.b - self.b) * alpha,
        )
    }

    pub fn to_hsv(&self) -> (f64, f64, f64) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let v = max;
        let s = if max == 0.0 { 0.0 } else { delta / max };

        let h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            ((self.g - self.b) / delta) % 6.0
        } else if max == self.g {
            (self.b - self.r) / delta + 2.0
        } else {
            (self.r - self.g) / delta + 4.0
        };

        let h = (h / 6.0).rem_euclid(1.0);
        (h, s, v)
    }

    pub fn to_hex(&self) -> String {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        format!("{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl std::ops::Add for Color3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl std::ops::Sub for Color3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl std::ops::Mul<f64> for Color3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl LuaUserData for Color3 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("R", |_, this| Ok(this.r));
        fields.add_field_method_get("G", |_, this| Ok(this.g));
        fields.add_field_method_get("B", |_, this| Ok(this.b));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "Lerp",
            |_, this, (goal, alpha): (LuaUserDataRef<Color3>, f64)| Ok(this.lerp(&goal, alpha)),
        );
        methods.add_method("ToHSV", |_, this, ()| {
            let (h, s, v) = this.to_hsv();
            Ok((h, s, v))
        });
        methods.add_method("ToHex", |_, this, ()| Ok(this.to_hex()));
        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<Color3>| Ok(*this == *other),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Color3({}, {}, {})", this.r, this.g, this.b))
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |_, this, other: LuaUserDataRef<Color3>| Ok(*this + *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |_, this, other: LuaUserDataRef<Color3>| Ok(*this - *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Mul,
            |_, this, scalar: f64| Ok(*this * scalar),
        );
    }
}

// ============================================================================
// Vector2
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn unit(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::ZERO
        } else {
            Self::new(self.x / mag, self.y / mag)
        }
    }

    pub fn lerp(&self, goal: &Vector2, alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        Self::new(
            self.x + (goal.x - self.x) * alpha,
            self.y + (goal.y - self.y) * alpha,
        )
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<f64> for Vector2 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl LuaUserData for Vector2 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
        fields.add_field_method_get("Magnitude", |_, this| Ok(this.magnitude()));
        fields.add_field_method_get("Unit", |lua, this| lua.create_userdata(this.unit()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "Lerp",
            |lua, this, (goal, alpha): (LuaUserDataRef<Vector2>, f64)| {
                lua.create_userdata(this.lerp(&goal, alpha))
            },
        );
        methods.add_method("Dot", |_, this, other: LuaUserDataRef<Vector2>| {
            Ok(this.dot(&other))
        });
        methods.add_method("Cross", |_, this, other: LuaUserDataRef<Vector2>| {
            Ok(this.cross(&other))
        });
        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<Vector2>| Ok(*this == *other),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Vector2({}, {})", this.x, this.y))
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |lua, this, other: LuaUserDataRef<Vector2>| lua.create_userdata(*this + *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |lua, this, other: LuaUserDataRef<Vector2>| lua.create_userdata(*this - *other),
        );
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, this, scalar: f64| {
            lua.create_userdata(*this * scalar)
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, this, scalar: f64| {
            lua.create_userdata(*this / scalar)
        });
        methods.add_meta_method(LuaMetaMethod::Unm, |lua, this, ()| {
            lua.create_userdata(-*this)
        });
    }
}

// ============================================================================
// UDim
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim {
    pub scale: f64,
    pub offset: f64,
}

impl UDim {
    pub fn new(scale: f64, offset: f64) -> Self {
        Self { scale, offset }
    }

    pub fn lerp(&self, goal: &UDim, alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        Self::new(
            self.scale + (goal.scale - self.scale) * alpha,
            self.offset + (goal.offset - self.offset) * alpha,
        )
    }
}

impl std::ops::Add for UDim {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.scale + other.scale, self.offset + other.offset)
    }
}

impl std::ops::Sub for UDim {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.scale - other.scale, self.offset - other.offset)
    }
}

impl std::ops::Neg for UDim {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.scale, -self.offset)
    }
}

impl LuaUserData for UDim {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Scale", |_, this| Ok(this.scale));
        fields.add_field_method_get("Offset", |_, this| Ok(this.offset));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "Lerp",
            |lua, this, (goal, alpha): (LuaUserDataRef<UDim>, f64)| {
                lua.create_userdata(this.lerp(&goal, alpha))
            },
        );
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaUserDataRef<UDim>| {
            Ok(*this == *other)
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("UDim({}, {})", this.scale, this.offset))
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |lua, this, other: LuaUserDataRef<UDim>| lua.create_userdata(*this + *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |lua, this, other: LuaUserDataRef<UDim>| lua.create_userdata(*this - *other),
        );
        methods.add_meta_method(LuaMetaMethod::Unm, |lua, this, ()| {
            lua.create_userdata(-*this)
        });
    }
}

// ============================================================================
// UDim2
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    pub fn new(x_scale: f64, x_offset: f64, y_scale: f64, y_offset: f64) -> Self {
        Self {
            x: UDim::new(x_scale, x_offset),
            y: UDim::new(y_scale, y_offset),
        }
    }

    pub fn from_scale(x_scale: f64, y_scale: f64) -> Self {
        Self::new(x_scale, 0.0, y_scale, 0.0)
    }

    pub fn from_offset(x_offset: f64, y_offset: f64) -> Self {
        Self::new(0.0, x_offset, 0.0, y_offset)
    }

    pub fn lerp(&self, goal: &UDim2, alpha: f64) -> Self {
        Self {
            x: self.x.lerp(&goal.x, alpha),
            y: self.y.lerp(&goal.y, alpha),
        }
    }
}

impl std::ops::Add for UDim2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for UDim2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Neg for UDim2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl LuaUserData for UDim2 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |lua, this| lua.create_userdata(this.x));
        fields.add_field_method_get("Y", |lua, this| lua.create_userdata(this.y));
        fields.add_field_method_get("Width", |lua, this| lua.create_userdata(this.x));
        fields.add_field_method_get("Height", |lua, this| lua.create_userdata(this.y));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "Lerp",
            |lua, this, (goal, alpha): (LuaUserDataRef<UDim2>, f64)| {
                lua.create_userdata(this.lerp(&goal, alpha))
            },
        );
        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<UDim2>| Ok(*this == *other),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!(
                "UDim2({}, {}, {}, {})",
                this.x.scale, this.x.offset, this.y.scale, this.y.offset
            ))
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |lua, this, other: LuaUserDataRef<UDim2>| lua.create_userdata(*this + *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |lua, this, other: LuaUserDataRef<UDim2>| lua.create_userdata(*this - *other),
        );
        methods.add_meta_method(LuaMetaMethod::Unm, |lua, this, ()| {
            lua.create_userdata(-*this)
        });
    }
}

// ============================================================================
// Rect
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vector2,
    pub max: Vector2,
}

impl Rect {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min: Vector2::new(min_x.min(max_x), min_y.min(max_y)),
            max: Vector2::new(min_x.max(max_x), min_y.max(max_y)),
        }
    }

    pub fn from_vectors(min: Vector2, max: Vector2) -> Self {
        Self::new(min.x, min.y, max.x, max.y)
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }
}

impl LuaUserData for Rect {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Min", |lua, this| lua.create_userdata(this.min));
        fields.add_field_method_get("Max", |lua, this| lua.create_userdata(this.max));
        fields.add_field_method_get("Width", |_, this| Ok(this.width()));
        fields.add_field_method_get("Height", |_, this| Ok(this.height()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaUserDataRef<Rect>| {
            Ok(*this == *other)
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!(
                "Rect({}, {}, {}, {})",
                this.min.x, this.min.y, this.max.x, this.max.y
            ))
        });
    }
}

// ============================================================================
// NumberRange
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberRange {
    pub min: f64,
    pub max: f64,
}

impl NumberRange {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }
}

impl LuaUserData for NumberRange {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Min", |_, this| Ok(this.min));
        fields.add_field_method_get("Max", |_, this| Ok(this.max));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<NumberRange>| Ok(*this == *other),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("NumberRange({}, {})", this.min, this.max))
        });
    }
}

// ============================================================================
// Vector3
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

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

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::ZERO
        } else {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    pub fn lerp(&self, goal: &Vector3, alpha: f64) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        Self::new(
            self.x + (goal.x - self.x) * alpha,
            self.y + (goal.y - self.y) * alpha,
            self.z + (goal.z - self.z) * alpha,
        )
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl LuaUserData for Vector3 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
        fields.add_field_method_get("Z", |_, this| Ok(this.z));
        fields.add_field_method_get("Magnitude", |_, this| Ok(this.magnitude()));
        fields.add_field_method_get("Unit", |lua, this| lua.create_userdata(this.unit()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "Lerp",
            |lua, this, (goal, alpha): (LuaUserDataRef<Vector3>, f64)| {
                lua.create_userdata(this.lerp(&goal, alpha))
            },
        );
        methods.add_method("Dot", |_, this, other: LuaUserDataRef<Vector3>| {
            Ok(this.dot(&other))
        });
        methods.add_method("Cross", |lua, this, other: LuaUserDataRef<Vector3>| {
            lua.create_userdata(this.cross(&other))
        });
        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<Vector3>| Ok(*this == *other),
        );
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Vector3({}, {}, {})", this.x, this.y, this.z))
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |lua, this, other: LuaUserDataRef<Vector3>| lua.create_userdata(*this + *other),
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |lua, this, other: LuaUserDataRef<Vector3>| lua.create_userdata(*this - *other),
        );
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, this, scalar: f64| {
            lua.create_userdata(*this * scalar)
        });
        methods.add_meta_method(LuaMetaMethod::Div, |lua, this, scalar: f64| {
            lua.create_userdata(*this / scalar)
        });
        methods.add_meta_method(LuaMetaMethod::Unm, |lua, this, ()| {
            lua.create_userdata(-*this)
        });
    }
}

// ============================================================================
// Module creators
// ============================================================================

pub fn create_color3(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua.clone())?
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
                .ok_or_else(|| LuaError::external("Invalid hex color"))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}

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

pub fn create_udim(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, (scale, offset): (f64, f64)| {
            lua.create_userdata(UDim::new(scale, offset))
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
    TableBuilder::new(lua.clone())?
        .with_function("new", |lua, args: LuaMultiValue| {
            let args: Vec<LuaValue> = args.into_vec();
            match args.len() {
                4 => {
                    let min_x = args[0].as_number().unwrap_or(0.0);
                    let min_y = args[1].as_number().unwrap_or(0.0);
                    let max_x = args[2].as_number().unwrap_or(0.0);
                    let max_y = args[3].as_number().unwrap_or(0.0);
                    lua.create_userdata(Rect::new(min_x, min_y, max_x, max_y))
                }
                2 => {
                    let min = args[0]
                        .as_userdata()
                        .and_then(|ud| ud.borrow::<Vector2>().ok())
                        .map(|v| *v)
                        .unwrap_or(Vector2::ZERO);
                    let max = args[1]
                        .as_userdata()
                        .and_then(|ud| ud.borrow::<Vector2>().ok())
                        .map(|v| *v)
                        .unwrap_or(Vector2::ZERO);
                    lua.create_userdata(Rect::from_vectors(min, max))
                }
                _ => Err(LuaError::external(
                    "Rect.new expects 2 Vector2s or 4 numbers",
                )),
            }
        })?
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

// ============================================================================
// Vector2int16
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

impl Vector2int16 {
    #[allow(dead_code)]
    pub const ZERO: Self = Self { x: 0, y: 0 };

    #[inline]
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl LuaUserData for Vector2int16 {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("{}, {}", this.x, this.y))
        });
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaUserDataRef<Self>| {
            Ok(this.x == other.x && this.y == other.y)
        });
        methods.add_meta_method(
            LuaMetaMethod::Add,
            |lua, this, other: LuaUserDataRef<Self>| {
                lua.create_userdata(Self::new(
                    this.x.saturating_add(other.x),
                    this.y.saturating_add(other.y),
                ))
            },
        );
        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |lua, this, other: LuaUserDataRef<Self>| {
                lua.create_userdata(Self::new(
                    this.x.saturating_sub(other.x),
                    this.y.saturating_sub(other.y),
                ))
            },
        );
        methods.add_meta_method(LuaMetaMethod::Mul, |lua, this, scalar: i16| {
            lua.create_userdata(Self::new(
                this.x.saturating_mul(scalar),
                this.y.saturating_mul(scalar),
            ))
        });
    }
}

pub fn create_vector2int16(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua.clone())?
        .with_function("new", |lua, (x, y): (i16, i16)| {
            lua.create_userdata(Vector2int16::new(x, y))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}

// ============================================================================
// TweenInfo
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct TweenInfo {
    pub time: f64,
    pub easing_style: u8, // 0=Linear, 1=Quad, 2=Cubic, 3=Sine, 4=Bounce, 5=Elastic
    pub easing_direction: u8, // 0=In, 1=Out, 2=InOut
    pub repeat_count: i32,
    pub reverses: bool,
    pub delay_time: f64,
}

impl Default for TweenInfo {
    fn default() -> Self {
        Self {
            time: 1.0,
            easing_style: 0,
            easing_direction: 1,
            repeat_count: 0,
            reverses: false,
            delay_time: 0.0,
        }
    }
}

impl TweenInfo {
    pub fn new(time: f64, style: u8, dir: u8, repeat: i32, reverses: bool, delay: f64) -> Self {
        Self {
            time,
            easing_style: style,
            easing_direction: dir,
            repeat_count: repeat,
            reverses,
            delay_time: delay,
        }
    }
}

impl LuaUserData for TweenInfo {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Time", |_, this| Ok(this.time));
        fields.add_field_method_get("EasingStyle", |_, this| Ok(this.easing_style));
        fields.add_field_method_get("EasingDirection", |_, this| Ok(this.easing_direction));
        fields.add_field_method_get("RepeatCount", |_, this| Ok(this.repeat_count));
        fields.add_field_method_get("Reverses", |_, this| Ok(this.reverses));
        fields.add_field_method_get("DelayTime", |_, this| Ok(this.delay_time));
    }
}

pub fn create_tween_info(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, args: LuaMultiValue| {
            let args: Vec<LuaValue> = args.into_vec();
            let time = args.first().and_then(|v| v.as_number()).unwrap_or(1.0);
            let style = args.get(1).and_then(|v| v.as_integer()).unwrap_or(0) as u8;
            let dir = args.get(2).and_then(|v| v.as_integer()).unwrap_or(1) as u8;
            let repeat = args.get(3).and_then(|v| v.as_integer()).unwrap_or(0) as i32;
            let reverses = args.get(4).and_then(|v| v.as_boolean()).unwrap_or(false);
            let delay = args.get(5).and_then(|v| v.as_number()).unwrap_or(0.0);
            lua.create_userdata(TweenInfo::new(time, style, dir, repeat, reverses, delay))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}

// ============================================================================
// bit32 - Bitwise operations
// ============================================================================

pub fn create_bit32(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("band", |_, args: LuaMultiValue| {
            let mut result = u32::MAX;
            for v in args.into_vec() {
                if let Some(n) = v.as_integer() {
                    result &= n as u32;
                }
            }
            Ok(result)
        })?
        .with_function("bor", |_, args: LuaMultiValue| {
            let mut result = 0u32;
            for v in args.into_vec() {
                if let Some(n) = v.as_integer() {
                    result |= n as u32;
                }
            }
            Ok(result)
        })?
        .with_function("bxor", |_, args: LuaMultiValue| {
            let mut result = 0u32;
            for v in args.into_vec() {
                if let Some(n) = v.as_integer() {
                    result ^= n as u32;
                }
            }
            Ok(result)
        })?
        .with_function("bnot", |_, n: u32| Ok(!n))?
        .with_function("lshift", |_, (n, disp): (u32, u32)| Ok(n << disp.min(31)))?
        .with_function("rshift", |_, (n, disp): (u32, u32)| Ok(n >> disp.min(31)))?
        .with_function("arshift", |_, (n, disp): (i32, u32)| {
            Ok((n >> disp.min(31)) as u32)
        })?
        .with_function(
            "extract",
            |_, (n, field, width): (u32, u32, Option<u32>)| {
                let width = width.unwrap_or(1).min(32 - field);
                let mask = (1u32 << width) - 1;
                Ok((n >> field) & mask)
            },
        )?
        .with_function(
            "replace",
            |_, (n, v, field, width): (u32, u32, u32, Option<u32>)| {
                let width = width.unwrap_or(1).min(32 - field);
                let mask = (1u32 << width) - 1;
                Ok((n & !(mask << field)) | ((v & mask) << field))
            },
        )?
        .with_function("countlz", |_, n: u32| Ok(n.leading_zeros()))?
        .with_function("countrz", |_, n: u32| Ok(n.trailing_zeros()))?
        .build_readonly()
        .map(LuaValue::Table)
}
