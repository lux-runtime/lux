use std::str::FromStr;

use mlua::prelude::*;

/// A standard global provided by Lux.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LuxStandardGlobal {
    GTable,
    Print,
    Require,
    Version,
    Warn,
    // Types from external crates
    Color3,
    Vector2,
    Vector3,
    UDim,
    UDim2,
    Rect,
    NumberRange,
    DateTime,
    Task,
    Enum,
}

impl LuxStandardGlobal {
    pub const ALL: &'static [Self] = &[
        Self::GTable,
        Self::Print,
        Self::Require,
        Self::Version,
        Self::Warn,
        Self::Color3,
        Self::Vector2,
        Self::Vector3,
        Self::UDim,
        Self::UDim2,
        Self::Rect,
        Self::NumberRange,
        Self::DateTime,
        Self::Task,
        Self::Enum,
    ];

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::GTable => "_G",
            Self::Print => "print",
            Self::Require => "require",
            Self::Version => "_VERSION",
            Self::Warn => "warn",
            Self::Color3 => "Color3",
            Self::Vector2 => "Vector2",
            Self::Vector3 => "Vector3",
            Self::UDim => "UDim",
            Self::UDim2 => "UDim2",
            Self::Rect => "Rect",
            Self::NumberRange => "NumberRange",
            Self::DateTime => "DateTime",
            Self::Task => "task",
            Self::Enum => "Enum",
        }
    }

    #[rustfmt::skip]
    #[allow(unreachable_patterns)]
    pub fn create(&self, lua: Lua) -> LuaResult<LuaValue> {
        let res = match self {
            Self::GTable => crate::globals::g_table::create(lua),
            Self::Print => crate::globals::print::create(lua),
            Self::Require => crate::globals::require::create(lua),
            Self::Version => crate::globals::version::create(lua),
            Self::Warn => crate::globals::warn::create(lua),
            // External crates
            Self::Color3 => lux_color::create(lua),
            Self::Vector2 => lux_vector::create_vector2(lua),
            Self::Vector3 => lux_vector::create_vector3(lua),
            Self::UDim => lux_udim::create_udim(lua),
            Self::UDim2 => lux_udim::create_udim2(lua),
            Self::Rect => lux_udim::create_rect(lua),
            Self::NumberRange => lux_udim::create_number_range(lua),
            Self::DateTime => lux_datetime::create(lua),
            Self::Task => lux_task::create(lua),
            Self::Enum => lux_enum::create(lua),
        };
        res.map_err(|e| e.context(format!("Failed to create global '{}'", self.name())))
    }
}

impl FromStr for LuxStandardGlobal {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let low = s.trim().to_ascii_lowercase();
        Ok(match low.as_str() {
            "_g" => Self::GTable,
            "print" => Self::Print,
            "require" => Self::Require,
            "_version" => Self::Version,
            "warn" => Self::Warn,
            "color3" => Self::Color3,
            "vector2" => Self::Vector2,
            "vector3" => Self::Vector3,
            "udim" => Self::UDim,
            "udim2" => Self::UDim2,
            "rect" => Self::Rect,
            "numberrange" => Self::NumberRange,
            "datetime" => Self::DateTime,
            "task" => Self::Task,
            "enum" => Self::Enum,
            _ => return Err(format!("Unknown global '{low}'")),
        })
    }
}
