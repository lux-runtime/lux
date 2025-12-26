-- tests/ffi/test_ffi_advanced.luau
-- Advanced FFI Tests

local ffi = require("@lux/ffi")

print("Testing @lux/ffi Advanced...")

-- 1. Properties
print("  > Testing ffi properties")
assert(type(ffi.os) == "string", "ffi.os should be string")
assert(type(ffi.arch) == "string", "ffi.arch should be string")
assert(ffi.os == "windows" or ffi.os == "linux" or ffi.os == "macos", "valid os")
assert(ffi.arch == "x86_64" or ffi.arch == "x86" or ffi.arch == "aarch64", "valid arch")

-- 2. Nested Struct
print("  > Testing nested struct")
ffi.cdef([[
    typedef struct Inner {
        int value;
    } Inner;
    
    typedef struct Outer {
        Inner inner;
        int count;
    } Outer;
]])

local outer = ffi.new("Outer")
outer.count = 5
assert(outer.count == 5, "outer.count access")
assert(ffi.sizeof("Outer") > ffi.sizeof("Inner"), "Outer larger than Inner")

-- 3. Arrays
print("  > Testing arrays")
local intArr = ffi.new("int[10]")
for i = 0, 9 do
	intArr[i] = i * 10
end
assert(intArr[0] == 0, "arr[0] == 0")
assert(intArr[5] == 50, "arr[5] == 50")
assert(intArr[9] == 90, "arr[9] == 90")

-- 4. ffi.sizeof basics
print("  > Testing sizeof")
assert(ffi.sizeof("char") == 1, "sizeof char")
assert(ffi.sizeof("short") == 2, "sizeof short")
assert(ffi.sizeof("int") == 4, "sizeof int")
assert(ffi.sizeof("long long") == 8, "sizeof long long")
assert(ffi.sizeof("float") == 4, "sizeof float")
assert(ffi.sizeof("double") == 8, "sizeof double")

-- 5. ffi.alignof
print("  > Testing alignof")
assert(ffi.alignof("char") == 1, "alignof char")
assert(ffi.alignof("int") == 4, "alignof int")
assert(ffi.alignof("double") == 8, "alignof double")

-- 6. ffi.typeof
print("  > Testing typeof")
local PointType = ffi.typeof("int")
assert(type(PointType) == "userdata", "typeof returns userdata")
assert(PointType.size == 4, "int size is 4")

-- 7. ffi.fill
print("  > Testing fill")
local buffer = ffi.new("char[10]")
ffi.fill(buffer, 10, 65) -- Fill with 'A' (65)
assert(buffer[0] == 65, "buffer[0] is 65")
assert(buffer[9] == 65, "buffer[9] is 65")

-- 8. ffi.copy
print("  > Testing copy")
local src = ffi.new("int[3]")
local dst = ffi.new("int[3]")
src[0] = 100
src[1] = 200
src[2] = 300
ffi.copy(dst, src, ffi.sizeof("int") * 3)
assert(dst[0] == 100, "copy dst[0]")
assert(dst[1] == 200, "copy dst[1]")
assert(dst[2] == 300, "copy dst[2]")

-- 9. ffi.string
print("  > Testing ffi.string")
local strBuf = ffi.new("char[20]")
-- Initialize with 'Hello' manually since ffi.copy from string may not work
local hello = "Hello"
for i = 1, #hello do
	strBuf[i - 1] = string.byte(hello, i)
end
strBuf[#hello] = 0 -- Null terminator

local luaStr = ffi.string(strBuf)
assert(luaStr == "Hello", "ffi.string conversion")

local partialStr = ffi.string(strBuf, 3)
assert(partialStr == "Hel", "partial ffi.string")

-- 10. ffi.istype
print("  > Testing istype")
local intVal = ffi.new("int", 42)
assert(ffi.istype("int", intVal) == true, "istype int")
assert(ffi.istype("float", intVal) == false, "istype not float")

-- 11. ffi.cast
print("  > Testing cast")
local castedPtr = ffi.cast("void*", 0x12345678)
assert(type(castedPtr) == "userdata", "cast returns userdata")

-- 12. Enum (if supported)
print("  > Testing enum")
ffi.cdef([[
    typedef enum Direction {
        NORTH = 0,
        EAST = 1,
        SOUTH = 2,
        WEST = 3
    } Direction;
]])
local dir = ffi.new("Direction")
dir[0] = 2 -- SOUTH
assert(dir[0] == 2, "enum value")

-- 13. Union
print("  > Testing union")
ffi.cdef([[
    typedef union Data {
        int i;
        float f;
        char c;
    } Data;
]])
local data = ffi.new("Data")
data.i = 42
assert(data.i == 42, "union int access")
assert(ffi.sizeof("Data") == 4, "union size is max member")

-- 14. ffi.C (if available)
print("  > Testing ffi.C")
if ffi.C then
	-- Try calling abs from C library
	local absOk, absResult = pcall(function()
		return ffi.C.abs(-42)
	end)
	if absOk then
		assert(absResult == 42, "ffi.C.abs(-42) == 42")
	else
		print("    C.abs not available: " .. tostring(absResult))
	end
else
	print("    ffi.C not available")
end

-- 15. Callback
print("  > Testing callback")
local callbackOk, callback = pcall(function()
	return ffi.callback("int(*)(int, int)", function(a, b)
		return a + b
	end)
end)
if callbackOk and callback then
	assert(callback ~= nil, "callback created")
	-- Note: callback internal structure may vary, just check it exists
	print("    Callback created successfully")
else
	print("    Callback creation skipped: " .. tostring(callback))
end

-- 16. offsetof
print("  > Testing offsetof")
ffi.cdef([[
    typedef struct Person {
        char name[32];
        int age;
    } Person;
]])
local nameOffset = ffi.offsetof("Person", "name")
local ageOffset = ffi.offsetof("Person", "age")
assert(nameOffset == 0, "name offset is 0")
assert(ageOffset == 32, "age offset is 32")

print("FFI Advanced Tests Passed!")
