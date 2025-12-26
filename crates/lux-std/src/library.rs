use std::str::FromStr;

use mlua::prelude::*;

/// A standard library provided by Lux (accessed via @lux/).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[rustfmt::skip]
pub enum LuxStandardLibrary {
    #[cfg(feature = "fs")]         Fs,
    #[cfg(feature = "luau")]       Luau,
    #[cfg(feature = "process")]    Process,
    #[cfg(feature = "regex")]      Regex,
    #[cfg(feature = "serde")]      Serde,
    #[cfg(feature = "stdio")]      Stdio,
    #[cfg(feature = "ffi")]        Ffi,
    #[cfg(feature = "signal")]     Signal,
}

impl LuxStandardLibrary {
    #[rustfmt::skip]
    pub const ALL: &'static [Self] = &[
        #[cfg(feature = "fs")]         Self::Fs,
        #[cfg(feature = "luau")]       Self::Luau,
        #[cfg(feature = "process")]    Self::Process,
        #[cfg(feature = "regex")]      Self::Regex,
        #[cfg(feature = "serde")]      Self::Serde,
        #[cfg(feature = "stdio")]      Self::Stdio,
        #[cfg(feature = "ffi")]        Self::Ffi,
        #[cfg(feature = "signal")]     Self::Signal,
    ];

    #[must_use]
    #[rustfmt::skip]
    #[allow(unreachable_patterns)]
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "fs")]         Self::Fs         => "fs",
            #[cfg(feature = "luau")]       Self::Luau       => "luau",
            #[cfg(feature = "process")]    Self::Process    => "process",
            #[cfg(feature = "regex")]      Self::Regex      => "regex",
            #[cfg(feature = "serde")]      Self::Serde      => "serde",
            #[cfg(feature = "stdio")]      Self::Stdio      => "stdio",
            #[cfg(feature = "ffi")]        Self::Ffi        => "ffi",
            #[cfg(feature = "signal")]     Self::Signal     => "signal",
            _ => unreachable!(),
        }
    }

    #[must_use]
    #[rustfmt::skip]
    #[allow(unreachable_patterns)]
    pub fn typedefs(&self) -> String {
    	match self {
            #[cfg(feature = "fs")]         Self::Fs         => lux_fs::typedefs(),
            #[cfg(feature = "luau")]       Self::Luau       => lux_luau::typedefs(),
            #[cfg(feature = "process")]    Self::Process    => lux_process::typedefs(),
            #[cfg(feature = "regex")]      Self::Regex      => lux_regex::typedefs(),
            #[cfg(feature = "serde")]      Self::Serde      => lux_serde::typedefs(),
            #[cfg(feature = "stdio")]      Self::Stdio      => lux_stdio::typedefs(),
            #[cfg(feature = "ffi")]        Self::Ffi        => lux_ffi::typedefs(),
            #[cfg(feature = "signal")]     Self::Signal     => lux_signal::typedefs(),
            _ => unreachable!(),
        }
    }

    #[rustfmt::skip]
    #[allow(unreachable_patterns)]
    pub fn module(&self, lua: Lua) -> LuaResult<LuaTable> {
        let res: LuaResult<LuaTable> = match self {
            #[cfg(feature = "fs")]         Self::Fs         => lux_fs::module(lua),
            #[cfg(feature = "luau")]       Self::Luau       => lux_luau::module(lua),
            #[cfg(feature = "process")]    Self::Process    => lux_process::module(lua),
            #[cfg(feature = "regex")]      Self::Regex      => lux_regex::module(lua),
            #[cfg(feature = "serde")]      Self::Serde      => lux_serde::module(lua),
            #[cfg(feature = "stdio")]      Self::Stdio      => lux_stdio::module(lua),
            #[cfg(feature = "ffi")]        Self::Ffi        => lux_ffi::module(lua),
            #[cfg(feature = "signal")]     Self::Signal     => lux_signal::module(lua),
            _ => unreachable!(),
        };
        res.map_err(|e| e.context(format!("Failed to create library '{}'", self.name())))
    }
}

impl FromStr for LuxStandardLibrary {
    type Err = String;
    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let low = s.trim().to_ascii_lowercase();
        Ok(match low.as_str() {
            #[cfg(feature = "fs")]         "fs"         => Self::Fs,
            #[cfg(feature = "luau")]       "luau"       => Self::Luau,
            #[cfg(feature = "process")]    "process"    => Self::Process,
            #[cfg(feature = "regex")]      "regex"      => Self::Regex,
            #[cfg(feature = "serde")]      "serde"      => Self::Serde,
            #[cfg(feature = "stdio")]      "stdio"      => Self::Stdio,
            #[cfg(feature = "ffi")]        "ffi"        => Self::Ffi,
            #[cfg(feature = "signal")]     "signal"     => Self::Signal,
            _ => return Err(format!("Unknown library '{low}'")),
        })
    }
}
