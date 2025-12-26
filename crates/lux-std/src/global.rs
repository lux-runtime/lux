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
    // Roblox data types
    Color3,
    Vector2,
    Vector3,
    Vector2int16,
    UDim,
    UDim2,
    Rect,
    NumberRange,
    TweenInfo,
    DateTime,
    // Task and script
    Task,
    Script,
    // Utility
    Bit32,
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
        Self::Vector2int16,
        Self::UDim,
        Self::UDim2,
        Self::Rect,
        Self::NumberRange,
        Self::TweenInfo,
        Self::DateTime,
        Self::Task,
        Self::Script,
        Self::Bit32,
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
            Self::Vector2int16 => "Vector2int16",
            Self::UDim => "UDim",
            Self::UDim2 => "UDim2",
            Self::Rect => "Rect",
            Self::NumberRange => "NumberRange",
            Self::TweenInfo => "TweenInfo",
            Self::DateTime => "DateTime",
            Self::Task => "task",
            Self::Script => "script",
            Self::Bit32 => "bit32",
        }
    }

    #[rustfmt::skip]
    #[allow(unreachable_patterns)]
    pub fn create(&self, lua: Lua) -> LuaResult<LuaValue> {
        use crate::globals::roblox_types;
        
        let res = match self {
            Self::GTable => crate::globals::g_table::create(lua),
            Self::Print => crate::globals::print::create(lua),
            Self::Require => crate::globals::require::create(lua),
            Self::Version => crate::globals::version::create(lua),
            Self::Warn => crate::globals::warn::create(lua),
            Self::Color3 => roblox_types::create_color3(lua),
            Self::Vector2 => roblox_types::create_vector2(lua),
            Self::Vector3 => roblox_types::create_vector3(lua),
            Self::Vector2int16 => roblox_types::create_vector2int16(lua),
            Self::UDim => roblox_types::create_udim(lua),
            Self::UDim2 => roblox_types::create_udim2(lua),
            Self::Rect => roblox_types::create_rect(lua),
            Self::NumberRange => roblox_types::create_number_range(lua),
            Self::TweenInfo => roblox_types::create_tween_info(lua),
            Self::DateTime => crate::globals::datetime::create(lua),
            Self::Task => crate::globals::task::create(lua),
            Self::Script => crate::globals::script::create(lua),
            Self::Bit32 => roblox_types::create_bit32(lua),
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
            "vector2int16" => Self::Vector2int16,
            "udim" => Self::UDim,
            "udim2" => Self::UDim2,
            "rect" => Self::Rect,
            "numberrange" => Self::NumberRange,
            "tweeninfo" => Self::TweenInfo,
            "datetime" => Self::DateTime,
            "task" => Self::Task,
            "script" => Self::Script,
            "bit32" => Self::Bit32,
            _ => return Err(format!("Unknown global '{low}'")),
        })
    }
}
