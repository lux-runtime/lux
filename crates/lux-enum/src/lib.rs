#![allow(clippy::cargo_common_metadata)]

//! Enum types for Lux: KeyCode (cross-platform), MouseButton, EasingStyle, EasingDirection
//! KeyCode values are platform-specific:
//! - Windows: VK_* codes (user32.dll)
//! - Linux: evdev KEY_* codes
//! - macOS: Carbon kVK_* codes

use lux_utils::TableBuilder;
use mlua::prelude::*;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

// Platform-specific key codes
#[cfg(target_os = "windows")]
mod keycodes {
    pub const A: i32 = 0x41;
    pub const B: i32 = 0x42;
    pub const C: i32 = 0x43;
    pub const D: i32 = 0x44;
    pub const E: i32 = 0x45;
    pub const F: i32 = 0x46;
    pub const G: i32 = 0x47;
    pub const H: i32 = 0x48;
    pub const I: i32 = 0x49;
    pub const J: i32 = 0x4A;
    pub const K: i32 = 0x4B;
    pub const L: i32 = 0x4C;
    pub const M: i32 = 0x4D;
    pub const N: i32 = 0x4E;
    pub const O: i32 = 0x4F;
    pub const P: i32 = 0x50;
    pub const Q: i32 = 0x51;
    pub const R: i32 = 0x52;
    pub const S: i32 = 0x53;
    pub const T: i32 = 0x54;
    pub const U: i32 = 0x55;
    pub const V: i32 = 0x56;
    pub const W: i32 = 0x57;
    pub const X: i32 = 0x58;
    pub const Y: i32 = 0x59;
    pub const Z: i32 = 0x5A;
    pub const ZERO: i32 = 0x30;
    pub const ONE: i32 = 0x31;
    pub const TWO: i32 = 0x32;
    pub const THREE: i32 = 0x33;
    pub const FOUR: i32 = 0x34;
    pub const FIVE: i32 = 0x35;
    pub const SIX: i32 = 0x36;
    pub const SEVEN: i32 = 0x37;
    pub const EIGHT: i32 = 0x38;
    pub const NINE: i32 = 0x39;
    pub const F1: i32 = 0x70;
    pub const F2: i32 = 0x71;
    pub const F3: i32 = 0x72;
    pub const F4: i32 = 0x73;
    pub const F5: i32 = 0x74;
    pub const F6: i32 = 0x75;
    pub const F7: i32 = 0x76;
    pub const F8: i32 = 0x77;
    pub const F9: i32 = 0x78;
    pub const F10: i32 = 0x79;
    pub const F11: i32 = 0x7A;
    pub const F12: i32 = 0x7B;
    pub const ESCAPE: i32 = 0x1B;
    pub const TAB: i32 = 0x09;
    pub const CAPS_LOCK: i32 = 0x14;
    pub const LEFT_SHIFT: i32 = 0xA0;
    pub const RIGHT_SHIFT: i32 = 0xA1;
    pub const LEFT_CONTROL: i32 = 0xA2;
    pub const RIGHT_CONTROL: i32 = 0xA3;
    pub const LEFT_ALT: i32 = 0xA4;
    pub const RIGHT_ALT: i32 = 0xA5;
    pub const LEFT_SUPER: i32 = 0x5B;
    pub const RIGHT_SUPER: i32 = 0x5C;
    pub const MENU: i32 = 0x5D;
    pub const SPACE: i32 = 0x20;
    pub const RETURN: i32 = 0x0D;
    pub const BACKSPACE: i32 = 0x08;
    pub const DELETE: i32 = 0x2E;
    pub const INSERT: i32 = 0x2D;
    pub const HOME: i32 = 0x24;
    pub const END: i32 = 0x23;
    pub const PAGE_UP: i32 = 0x21;
    pub const PAGE_DOWN: i32 = 0x22;
    pub const UP: i32 = 0x26;
    pub const DOWN: i32 = 0x28;
    pub const LEFT: i32 = 0x25;
    pub const RIGHT: i32 = 0x27;
    pub const NUMPAD0: i32 = 0x60;
    pub const NUMPAD1: i32 = 0x61;
    pub const NUMPAD2: i32 = 0x62;
    pub const NUMPAD3: i32 = 0x63;
    pub const NUMPAD4: i32 = 0x64;
    pub const NUMPAD5: i32 = 0x65;
    pub const NUMPAD6: i32 = 0x66;
    pub const NUMPAD7: i32 = 0x67;
    pub const NUMPAD8: i32 = 0x68;
    pub const NUMPAD9: i32 = 0x69;
    pub const NUM_LOCK: i32 = 0x90;
    pub const SEMICOLON: i32 = 0xBA;
    pub const EQUALS: i32 = 0xBB;
    pub const COMMA: i32 = 0xBC;
    pub const MINUS: i32 = 0xBD;
    pub const PERIOD: i32 = 0xBE;
    pub const SLASH: i32 = 0xBF;
    pub const GRAVE: i32 = 0xC0;
    pub const LEFT_BRACKET: i32 = 0xDB;
    pub const BACKSLASH: i32 = 0xDC;
    pub const RIGHT_BRACKET: i32 = 0xDD;
    pub const APOSTROPHE: i32 = 0xDE;
}

#[cfg(target_os = "linux")]
mod keycodes {
    pub const A: i32 = 30;
    pub const B: i32 = 48;
    pub const C: i32 = 46;
    pub const D: i32 = 32;
    pub const E: i32 = 18;
    pub const F: i32 = 33;
    pub const G: i32 = 34;
    pub const H: i32 = 35;
    pub const I: i32 = 23;
    pub const J: i32 = 36;
    pub const K: i32 = 37;
    pub const L: i32 = 38;
    pub const M: i32 = 50;
    pub const N: i32 = 49;
    pub const O: i32 = 24;
    pub const P: i32 = 25;
    pub const Q: i32 = 16;
    pub const R: i32 = 19;
    pub const S: i32 = 31;
    pub const T: i32 = 20;
    pub const U: i32 = 22;
    pub const V: i32 = 47;
    pub const W: i32 = 17;
    pub const X: i32 = 45;
    pub const Y: i32 = 21;
    pub const Z: i32 = 44;
    pub const ZERO: i32 = 11;
    pub const ONE: i32 = 2;
    pub const TWO: i32 = 3;
    pub const THREE: i32 = 4;
    pub const FOUR: i32 = 5;
    pub const FIVE: i32 = 6;
    pub const SIX: i32 = 7;
    pub const SEVEN: i32 = 8;
    pub const EIGHT: i32 = 9;
    pub const NINE: i32 = 10;
    pub const F1: i32 = 59;
    pub const F2: i32 = 60;
    pub const F3: i32 = 61;
    pub const F4: i32 = 62;
    pub const F5: i32 = 63;
    pub const F6: i32 = 64;
    pub const F7: i32 = 65;
    pub const F8: i32 = 66;
    pub const F9: i32 = 67;
    pub const F10: i32 = 68;
    pub const F11: i32 = 87;
    pub const F12: i32 = 88;
    pub const ESCAPE: i32 = 1;
    pub const TAB: i32 = 15;
    pub const CAPS_LOCK: i32 = 58;
    pub const LEFT_SHIFT: i32 = 42;
    pub const RIGHT_SHIFT: i32 = 54;
    pub const LEFT_CONTROL: i32 = 29;
    pub const RIGHT_CONTROL: i32 = 97;
    pub const LEFT_ALT: i32 = 56;
    pub const RIGHT_ALT: i32 = 100;
    pub const LEFT_SUPER: i32 = 125;
    pub const RIGHT_SUPER: i32 = 126;
    pub const MENU: i32 = 127;
    pub const SPACE: i32 = 57;
    pub const RETURN: i32 = 28;
    pub const BACKSPACE: i32 = 14;
    pub const DELETE: i32 = 111;
    pub const INSERT: i32 = 110;
    pub const HOME: i32 = 102;
    pub const END: i32 = 107;
    pub const PAGE_UP: i32 = 104;
    pub const PAGE_DOWN: i32 = 109;
    pub const UP: i32 = 103;
    pub const DOWN: i32 = 108;
    pub const LEFT: i32 = 105;
    pub const RIGHT: i32 = 106;
    pub const NUMPAD0: i32 = 82;
    pub const NUMPAD1: i32 = 79;
    pub const NUMPAD2: i32 = 80;
    pub const NUMPAD3: i32 = 81;
    pub const NUMPAD4: i32 = 75;
    pub const NUMPAD5: i32 = 76;
    pub const NUMPAD6: i32 = 77;
    pub const NUMPAD7: i32 = 71;
    pub const NUMPAD8: i32 = 72;
    pub const NUMPAD9: i32 = 73;
    pub const NUM_LOCK: i32 = 69;
    pub const SEMICOLON: i32 = 39;
    pub const EQUALS: i32 = 13;
    pub const COMMA: i32 = 51;
    pub const MINUS: i32 = 12;
    pub const PERIOD: i32 = 52;
    pub const SLASH: i32 = 53;
    pub const GRAVE: i32 = 41;
    pub const LEFT_BRACKET: i32 = 26;
    pub const BACKSLASH: i32 = 43;
    pub const RIGHT_BRACKET: i32 = 27;
    pub const APOSTROPHE: i32 = 40;
}

#[cfg(target_os = "macos")]
mod keycodes {
    pub const A: i32 = 0x00;
    pub const B: i32 = 0x0B;
    pub const C: i32 = 0x08;
    pub const D: i32 = 0x02;
    pub const E: i32 = 0x0E;
    pub const F: i32 = 0x03;
    pub const G: i32 = 0x05;
    pub const H: i32 = 0x04;
    pub const I: i32 = 0x22;
    pub const J: i32 = 0x26;
    pub const K: i32 = 0x28;
    pub const L: i32 = 0x25;
    pub const M: i32 = 0x2E;
    pub const N: i32 = 0x2D;
    pub const O: i32 = 0x1F;
    pub const P: i32 = 0x23;
    pub const Q: i32 = 0x0C;
    pub const R: i32 = 0x0F;
    pub const S: i32 = 0x01;
    pub const T: i32 = 0x11;
    pub const U: i32 = 0x20;
    pub const V: i32 = 0x09;
    pub const W: i32 = 0x0D;
    pub const X: i32 = 0x07;
    pub const Y: i32 = 0x10;
    pub const Z: i32 = 0x06;
    pub const ZERO: i32 = 0x1D;
    pub const ONE: i32 = 0x12;
    pub const TWO: i32 = 0x13;
    pub const THREE: i32 = 0x14;
    pub const FOUR: i32 = 0x15;
    pub const FIVE: i32 = 0x17;
    pub const SIX: i32 = 0x16;
    pub const SEVEN: i32 = 0x1A;
    pub const EIGHT: i32 = 0x1C;
    pub const NINE: i32 = 0x19;
    pub const F1: i32 = 0x7A;
    pub const F2: i32 = 0x78;
    pub const F3: i32 = 0x63;
    pub const F4: i32 = 0x76;
    pub const F5: i32 = 0x60;
    pub const F6: i32 = 0x61;
    pub const F7: i32 = 0x62;
    pub const F8: i32 = 0x64;
    pub const F9: i32 = 0x65;
    pub const F10: i32 = 0x6D;
    pub const F11: i32 = 0x67;
    pub const F12: i32 = 0x6F;
    pub const ESCAPE: i32 = 0x35;
    pub const TAB: i32 = 0x30;
    pub const CAPS_LOCK: i32 = 0x39;
    pub const LEFT_SHIFT: i32 = 0x38;
    pub const RIGHT_SHIFT: i32 = 0x3C;
    pub const LEFT_CONTROL: i32 = 0x3B;
    pub const RIGHT_CONTROL: i32 = 0x3E;
    pub const LEFT_ALT: i32 = 0x3A;
    pub const RIGHT_ALT: i32 = 0x3D;
    pub const LEFT_SUPER: i32 = 0x37;
    pub const RIGHT_SUPER: i32 = 0x36;
    pub const MENU: i32 = 0x6E;
    pub const SPACE: i32 = 0x31;
    pub const RETURN: i32 = 0x24;
    pub const BACKSPACE: i32 = 0x33;
    pub const DELETE: i32 = 0x75;
    pub const INSERT: i32 = 0x72;
    pub const HOME: i32 = 0x73;
    pub const END: i32 = 0x77;
    pub const PAGE_UP: i32 = 0x74;
    pub const PAGE_DOWN: i32 = 0x79;
    pub const UP: i32 = 0x7E;
    pub const DOWN: i32 = 0x7D;
    pub const LEFT: i32 = 0x7B;
    pub const RIGHT: i32 = 0x7C;
    pub const NUMPAD0: i32 = 0x52;
    pub const NUMPAD1: i32 = 0x53;
    pub const NUMPAD2: i32 = 0x54;
    pub const NUMPAD3: i32 = 0x55;
    pub const NUMPAD4: i32 = 0x56;
    pub const NUMPAD5: i32 = 0x57;
    pub const NUMPAD6: i32 = 0x58;
    pub const NUMPAD7: i32 = 0x59;
    pub const NUMPAD8: i32 = 0x5B;
    pub const NUMPAD9: i32 = 0x5C;
    pub const NUM_LOCK: i32 = 0x47;
    pub const SEMICOLON: i32 = 0x29;
    pub const EQUALS: i32 = 0x18;
    pub const COMMA: i32 = 0x2B;
    pub const MINUS: i32 = 0x1B;
    pub const PERIOD: i32 = 0x2F;
    pub const SLASH: i32 = 0x2C;
    pub const GRAVE: i32 = 0x32;
    pub const LEFT_BRACKET: i32 = 0x21;
    pub const BACKSLASH: i32 = 0x2A;
    pub const RIGHT_BRACKET: i32 = 0x1E;
    pub const APOSTROPHE: i32 = 0x27;
}

use keycodes::*;

/// Creates Enum.KeyCode - Platform-specific key codes for FFI
pub fn create_keycode(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_value("A", A)?
        .with_value("B", B)?
        .with_value("C", C)?
        .with_value("D", D)?
        .with_value("E", E)?
        .with_value("F", F)?
        .with_value("G", G)?
        .with_value("H", H)?
        .with_value("I", I)?
        .with_value("J", J)?
        .with_value("K", K)?
        .with_value("L", L)?
        .with_value("M", M)?
        .with_value("N", N)?
        .with_value("O", O)?
        .with_value("P", P)?
        .with_value("Q", Q)?
        .with_value("R", R)?
        .with_value("S", S)?
        .with_value("T", T)?
        .with_value("U", U)?
        .with_value("V", V)?
        .with_value("W", W)?
        .with_value("X", X)?
        .with_value("Y", Y)?
        .with_value("Z", Z)?
        .with_value("Zero", ZERO)?
        .with_value("One", ONE)?
        .with_value("Two", TWO)?
        .with_value("Three", THREE)?
        .with_value("Four", FOUR)?
        .with_value("Five", FIVE)?
        .with_value("Six", SIX)?
        .with_value("Seven", SEVEN)?
        .with_value("Eight", EIGHT)?
        .with_value("Nine", NINE)?
        .with_value("F1", F1)?
        .with_value("F2", F2)?
        .with_value("F3", F3)?
        .with_value("F4", F4)?
        .with_value("F5", F5)?
        .with_value("F6", F6)?
        .with_value("F7", F7)?
        .with_value("F8", F8)?
        .with_value("F9", F9)?
        .with_value("F10", F10)?
        .with_value("F11", F11)?
        .with_value("F12", F12)?
        .with_value("Escape", ESCAPE)?
        .with_value("Tab", TAB)?
        .with_value("CapsLock", CAPS_LOCK)?
        .with_value("LeftShift", LEFT_SHIFT)?
        .with_value("RightShift", RIGHT_SHIFT)?
        .with_value("LeftControl", LEFT_CONTROL)?
        .with_value("RightControl", RIGHT_CONTROL)?
        .with_value("LeftAlt", LEFT_ALT)?
        .with_value("RightAlt", RIGHT_ALT)?
        .with_value("LeftSuper", LEFT_SUPER)?
        .with_value("RightSuper", RIGHT_SUPER)?
        .with_value("Menu", MENU)?
        .with_value("Space", SPACE)?
        .with_value("Return", RETURN)?
        .with_value("Backspace", BACKSPACE)?
        .with_value("Delete", DELETE)?
        .with_value("Insert", INSERT)?
        .with_value("Home", HOME)?
        .with_value("End", END)?
        .with_value("PageUp", PAGE_UP)?
        .with_value("PageDown", PAGE_DOWN)?
        .with_value("Up", UP)?
        .with_value("Down", DOWN)?
        .with_value("Left", LEFT)?
        .with_value("Right", RIGHT)?
        .with_value("Numpad0", NUMPAD0)?
        .with_value("Numpad1", NUMPAD1)?
        .with_value("Numpad2", NUMPAD2)?
        .with_value("Numpad3", NUMPAD3)?
        .with_value("Numpad4", NUMPAD4)?
        .with_value("Numpad5", NUMPAD5)?
        .with_value("Numpad6", NUMPAD6)?
        .with_value("Numpad7", NUMPAD7)?
        .with_value("Numpad8", NUMPAD8)?
        .with_value("Numpad9", NUMPAD9)?
        .with_value("NumLock", NUM_LOCK)?
        .with_value("Semicolon", SEMICOLON)?
        .with_value("Equals", EQUALS)?
        .with_value("Comma", COMMA)?
        .with_value("Minus", MINUS)?
        .with_value("Period", PERIOD)?
        .with_value("Slash", SLASH)?
        .with_value("Grave", GRAVE)?
        .with_value("LeftBracket", LEFT_BRACKET)?
        .with_value("Backslash", BACKSLASH)?
        .with_value("RightBracket", RIGHT_BRACKET)?
        .with_value("Apostrophe", APOSTROPHE)?
        .build_readonly()
        .map(LuaValue::Table)
}

/// Creates Enum.MouseButton
pub fn create_mouse_button(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_value("Left", 0)?
        .with_value("Right", 1)?
        .with_value("Middle", 2)?
        .with_value("Button4", 3)?
        .with_value("Button5", 4)?
        .build_readonly()
        .map(LuaValue::Table)
}

/// Creates Enum.EasingStyle
pub fn create_easing_style(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_value("Linear", 0)?
        .with_value("Quad", 1)?
        .with_value("Cubic", 2)?
        .with_value("Quart", 3)?
        .with_value("Quint", 4)?
        .with_value("Sine", 5)?
        .with_value("Expo", 6)?
        .with_value("Circ", 7)?
        .with_value("Elastic", 8)?
        .with_value("Back", 9)?
        .with_value("Bounce", 10)?
        .build_readonly()
        .map(LuaValue::Table)
}

/// Creates Enum.EasingDirection
pub fn create_easing_direction(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_value("In", 0)?
        .with_value("Out", 1)?
        .with_value("InOut", 2)?
        .build_readonly()
        .map(LuaValue::Table)
}

/// Creates Enum.SortOrder
pub fn create_sort_order(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_value("LayoutOrder", 0)?
        .with_value("Name", 1)?
        .build_readonly()
        .map(LuaValue::Table)
}

/// Creates the main Enum global
pub fn create(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua.clone())?
        .with_value("KeyCode", create_keycode(lua.clone())?)?
        .with_value("MouseButton", create_mouse_button(lua.clone())?)?
        .with_value("EasingStyle", create_easing_style(lua.clone())?)?
        .with_value("EasingDirection", create_easing_direction(lua.clone())?)?
        .with_value("SortOrder", create_sort_order(lua)?)?
        .build_readonly()
        .map(LuaValue::Table)
}
