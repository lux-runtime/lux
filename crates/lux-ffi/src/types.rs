//! C Type System
//!
//! Defines all C type representations and conversions.

/// C type representation
#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    Void,
    Bool,
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,  // Explicit 64-bit signed
    ULongLong, // Explicit 64-bit unsigned
    Int8,      // Sized types
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    WChar, // Windows wide char (usually 16-bit)
    Pointer(Option<Box<CType>>),
    Array(Box<CType>, usize),
    Struct(String),
    Union(String),
    Enum(String),
    Function(Box<FuncType>),
    GUID,    // 128-bit UUID for COM
    HRESULT, // COM result type (32-bit signed)
}

impl CType {
    /// Size in bytes
    pub fn size(&self) -> usize {
        match self {
            CType::Void => 0,
            CType::Bool | CType::Char | CType::UChar | CType::Int8 | CType::UInt8 => 1,
            CType::Short | CType::UShort | CType::Int16 | CType::UInt16 | CType::WChar => 2,
            CType::Int
            | CType::UInt
            | CType::Int32
            | CType::UInt32
            | CType::Float
            | CType::HRESULT => 4,
            CType::Long
            | CType::ULong
            | CType::LongLong
            | CType::ULongLong
            | CType::Int64
            | CType::UInt64
            | CType::Double => 8,
            CType::GUID => 16,
            CType::Pointer(_) | CType::Function(_) => std::mem::size_of::<usize>(),
            CType::Array(elem, count) => elem.size() * count,
            CType::Struct(name) | CType::Union(name) => crate::registry::Registry::get()
                .struct_size(name)
                .unwrap_or(0),
            CType::Enum(_) => 4,
        }
    }

    /// Alignment in bytes
    pub fn align(&self) -> usize {
        match self {
            CType::Void => 1,
            CType::Bool | CType::Char | CType::UChar | CType::Int8 | CType::UInt8 => 1,
            CType::Short | CType::UShort | CType::Int16 | CType::UInt16 | CType::WChar => 2,
            CType::Int
            | CType::UInt
            | CType::Int32
            | CType::UInt32
            | CType::Float
            | CType::Enum(_)
            | CType::HRESULT => 4,
            CType::Long
            | CType::ULong
            | CType::LongLong
            | CType::ULongLong
            | CType::Int64
            | CType::UInt64
            | CType::Double => 8,
            CType::GUID => 4, // GUID is typically 4-byte aligned
            CType::Pointer(_) | CType::Function(_) => std::mem::size_of::<usize>(),
            CType::Array(elem, _) => elem.align(),
            CType::Struct(name) | CType::Union(name) => crate::registry::Registry::get()
                .struct_align(name)
                .unwrap_or(1),
        }
    }

    /// Parse a C type string
    pub fn parse(s: &str) -> Option<Self> {
        // Clean the string of variable names if mixed in, but this is type parsing.
        // We assume 's' is just the type part.
        // However, we should strip common C keywords that CType doesn't care about here
        // but might be passed in.
        let mut s = s.trim();

        // Strip common modifiers
        s = s.strip_prefix("const ").unwrap_or(s).trim();
        s = s.strip_prefix("volatile ").unwrap_or(s).trim();
        s = s.strip_prefix("static ").unwrap_or(s).trim();
        s = s.strip_prefix("extern ").unwrap_or(s).trim();
        // Win32 calling conventions might be attached to function types, usually they are handled in parser.rs
        // properly, but sometimes might leak here.

        if s.is_empty() {
            return None;
        }

        // Handle arrays: "int[4]" or "int [4]"
        if let Some(bracket) = s.find('[') {
            let base = s[..bracket].trim();
            let size_str = s[bracket + 1..].trim_end_matches(']');
            let size = size_str.parse().ok()?;
            return Some(CType::Array(Box::new(CType::parse(base)?), size));
        }

        // Handle pointers
        if s.ends_with('*') {
            let base = s[..s.len() - 1].trim();
            if base == "void" || base.is_empty() {
                return Some(CType::Pointer(None));
            }
            return Some(CType::Pointer(Some(Box::new(CType::parse(base)?))));
        }

        // Primitives
        match s {
            "void" => Some(CType::Void),
            "bool" | "_Bool" | "BOOL" => Some(CType::Int), // Win32 BOOL is int (4 bytes)
            "char" | "int8_t" | "signed char" => {
                // Explicit signed char
                if s == "char" {
                    // Check platform default char signedness
                    // std::ffi::c_char is i8 or u8 depending on platform
                    // (ARM is usually unsigned, x86 signed)
                    if std::any::TypeId::of::<std::ffi::c_char>() == std::any::TypeId::of::<u8>() {
                        Some(CType::UChar)
                    } else {
                        Some(CType::Char)
                    }
                } else {
                    Some(CType::Char)
                }
            }
            "unsigned char" | "uint8_t" | "BYTE" | "byte" => Some(CType::UChar),
            "short" | "int16_t" | "SHORT" => Some(CType::Short),
            "unsigned short" | "uint16_t" | "USHORT" | "WORD" => Some(CType::UShort),
            "int" | "int32_t" | "signed" | "INT" | "HRESULT" => Some(CType::Int),
            "unsigned" | "unsigned int" | "uint32_t" | "UINT" | "DWORD" => Some(CType::UInt),
            "long" | "long int" => {
                // In C, 'long' depends on platform.
                // Windows (MSVC): long is 32-bit even on 64-bit.
                // Linux/Mac: long is 64-bit on 64-bit (LP64), 32-bit on 32-bit.
                if cfg!(windows) {
                    Some(CType::Int)
                } else {
                    #[cfg(target_pointer_width = "64")]
                    {
                        Some(CType::Long)
                    }
                    #[cfg(target_pointer_width = "32")]
                    {
                        Some(CType::Int)
                    }
                }
            }
            "unsigned long" => {
                if cfg!(windows) {
                    Some(CType::UInt)
                } else {
                    Some(CType::ULong)
                }
            }
            "long long" | "int64_t" | "LONGLONG" => Some(CType::Long),
            "unsigned long long" | "uint64_t" | "ULONGLONG" => Some(CType::ULong),

            "intptr_t" | "ptrdiff_t" | "ssize_t" => {
                #[cfg(target_pointer_width = "64")]
                {
                    Some(CType::Long)
                }
                #[cfg(target_pointer_width = "32")]
                {
                    Some(CType::Int)
                }
            }

            "uintptr_t" | "size_t" | "ULONG_PTR" | "DWORD_PTR" | "SIZE_T" => {
                #[cfg(target_pointer_width = "64")]
                {
                    Some(CType::ULong)
                }
                #[cfg(target_pointer_width = "32")]
                {
                    Some(CType::UInt)
                }
            }
            "float" | "FLOAT" => Some(CType::Float),
            "double" | "DOUBLE" => Some(CType::Double),

            // String types -> char pointer
            "char*" | "const char*" | "LPCSTR" | "LPSTR" | "PCSTR" | "PSTR" => {
                Some(CType::Pointer(Some(Box::new(CType::Char))))
            }
            "wchar_t*" | "const wchar_t*" | "LPCWSTR" | "LPWSTR" | "PCWSTR" | "PWSTR" => {
                Some(CType::Pointer(Some(Box::new(CType::UShort))))
            }

            // Win32 handle types -> pointer
            "HWND" | "HANDLE" | "HINSTANCE" | "HMODULE" | "HDC" | "HBRUSH" | "HPEN" | "HICON"
            | "HCURSOR" | "HMENU" | "HFONT" | "HBITMAP" | "HRGN" | "HGDIOBJ" | "LPVOID"
            | "PVOID" | "LPCVOID" => Some(CType::Pointer(None)),

            // Win32 parameter types
            "WPARAM" | "LPARAM" | "LRESULT" => {
                #[cfg(target_pointer_width = "64")]
                {
                    Some(CType::Long)
                }
                #[cfg(target_pointer_width = "32")]
                {
                    Some(CType::Int)
                }
            }
            "ATOM" => Some(CType::UShort),

            // Named types - lookup in registry
            _ => {
                let reg = crate::registry::Registry::get();

                // Check for struct prefix
                if let Some(name) = s.strip_prefix("struct ") {
                    return Some(CType::Struct(name.trim().to_string()));
                }
                if let Some(name) = s.strip_prefix("union ") {
                    return Some(CType::Union(name.trim().to_string()));
                }
                if let Some(name) = s.strip_prefix("enum ") {
                    return Some(CType::Enum(name.trim().to_string()));
                }

                if reg.has_struct(s) {
                    Some(CType::Struct(s.to_string()))
                } else if reg.has_enum(s) {
                    Some(CType::Enum(s.to_string()))
                } else if let Some(alias) = reg.get_typedef(s) {
                    Some(alias)
                } else {
                    // Start assume pointer for LP*/P* Win32 types if not resolved
                    if (s.starts_with("LP") || s.starts_with("P"))
                        && s.len() > 2
                        && s.chars()
                            .nth(if s.starts_with("LP") { 2 } else { 1 })
                            .map(|c| c.is_uppercase())
                            .unwrap_or(false)
                    {
                        Some(CType::Pointer(None))
                    } else {
                        // Forward declaration / Lazy Struct support
                        // If we don't know the type, and it's not a primitive, assume it is a struct.
                        // This allows `typedef struct Foo Foo;` or just `Foo* p;` where `Foo` is defined later.
                        // We must be careful not to swallow typos, but in C, unknown usage is usually a struct tag or typedef.
                        // If we return Some(Struct(s)), it works for pointers.
                        Some(CType::Struct(s.to_string()))
                    }
                }
            }
        }
    }

    /// Convert to libffi type
    pub fn to_ffi_type(&self) -> libffi::middle::Type {
        use libffi::middle::Type;
        match self {
            CType::Void => Type::void(),
            CType::Bool | CType::Char | CType::Int8 => Type::i8(),
            CType::UChar | CType::UInt8 => Type::u8(),
            CType::Short | CType::Int16 => Type::i16(),
            CType::UShort | CType::UInt16 | CType::WChar => Type::u16(),
            CType::Int | CType::Int32 | CType::Enum(_) | CType::HRESULT => Type::i32(),
            CType::UInt | CType::UInt32 => Type::u32(),
            CType::Long | CType::LongLong | CType::Int64 => Type::i64(),
            CType::ULong | CType::ULongLong | CType::UInt64 => Type::u64(),
            CType::Float => Type::f32(),
            CType::Double => Type::f64(),
            CType::Pointer(_) | CType::Function(_) => Type::pointer(),
            CType::Array(_, _) => Type::pointer(),
            CType::Struct(_) | CType::Union(_) => Type::pointer(),
            CType::GUID => Type::pointer(), // GUID passed by pointer typically
        }
    }
}

/// Function type
#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub ret: CType,
    pub args: Vec<CType>,
    pub variadic: bool,
    pub conv: CallConv,
}

/// Calling convention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallConv {
    #[default]
    C,
    Stdcall,
    Fastcall,
    Win64,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ctype: CType,
    pub offset: usize,
    pub bits: Option<(usize, usize)>,
}

/// Struct/Union definition
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub size: usize,
    pub align: usize,
    pub is_union: bool,
    pub is_packed: bool,
}

impl StructDef {
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn field_offset(&self, name: &str) -> Option<usize> {
        self.field(name).map(|f| f.offset)
    }
}

/// Function signature
#[derive(Debug, Clone)]
pub struct FuncSig {
    pub name: String,
    pub ret: CType,
    pub args: Vec<(String, CType)>,
    pub variadic: bool,
    pub conv: CallConv,
}

impl FuncSig {
    pub fn arg_types(&self) -> Vec<&CType> {
        self.args.iter().map(|(_, t)| t).collect()
    }
}

// IntoLua implementation for CType
use mlua::prelude::*;

impl IntoLua for CType {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let name = match self {
            CType::Void => "void",
            CType::Bool => "bool",
            CType::Char => "char",
            CType::UChar => "uchar",
            CType::Short => "short",
            CType::UShort => "ushort",
            CType::Int => "int",
            CType::UInt => "uint",
            CType::Long => "long",
            CType::ULong => "ulong",
            CType::LongLong => "longlong",
            CType::ULongLong => "ulonglong",
            CType::Int8 => "int8",
            CType::UInt8 => "uint8",
            CType::Int16 => "int16",
            CType::UInt16 => "uint16",
            CType::Int32 => "int32",
            CType::UInt32 => "uint32",
            CType::Int64 => "int64",
            CType::UInt64 => "uint64",
            CType::Float => "float",
            CType::Double => "double",
            CType::WChar => "wchar",
            CType::Pointer(_) => "pointer",
            CType::Array(_, _) => "array",
            CType::Struct(_) => "struct",
            CType::Union(_) => "union",
            CType::Enum(_) => "enum",
            CType::Function(_) => "function",
            CType::GUID => "guid",
            CType::HRESULT => "hresult",
        };
        Ok(LuaValue::String(lua.create_string(name)?))
    }
}
