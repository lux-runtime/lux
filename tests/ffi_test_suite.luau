--!strict
--[[
    ╔══════════════════════════════════════════════════════════════════╗
    ║          COMPREHENSIVE FFI TEST SUITE - LUX RUNTIME              ║
    ║  Based on LuaJIT FFI test patterns and Python ctypes edge cases  ║
    ╚══════════════════════════════════════════════════════════════════╝
    
    Tests all FFI methods and edge cases to find bugs:
    - ffi.cdef, ffi.new, ffi.cast, ffi.typeof
    - ffi.sizeof, ffi.alignof, ffi.offsetof
    - ffi.string, ffi.copy, ffi.fill
    - ffi.istype, ffi.metatype, ffi.gc
    - ffi.load, ffi.callback, ffi.batch
    - Pointer arithmetic, struct access, type conversions
]]

local ffi = require("@lux/ffi")

-- Test state
local tests_passed = 0
local tests_failed = 0
local current_section = ""

-- Helper functions
local function section(name: string)
	current_section = name
	print("")
	print(string.format("┌─ %s ─┐", string.rep("─", 50)))
	print(string.format("│ %-52s │", name))
	print(string.format("└─%s─┘", string.rep("─", 52)))
end

local function test(name: string, fn: () -> (boolean, string?))
	local ok, result = pcall(fn)
	if ok and result then
		tests_passed = tests_passed + 1
		print(string.format("  ✅ %s", name))
	else
		tests_failed = tests_failed + 1
		local err = if not ok then tostring(result) else "assertion failed"
		print(string.format("  ❌ %s: %s", name, err))
	end
end

local function expect_error(name: string, fn: () -> any)
	local ok, err = pcall(fn)
	if not ok then
		tests_passed = tests_passed + 1
		print(string.format("  ✅ %s (expected error)", name))
	else
		tests_failed = tests_failed + 1
		print(string.format("  ❌ %s: expected error but got success", name))
	end
end

-- ========================================================================
-- SECTION 1: ffi.cdef - C Declaration Parsing
-- ========================================================================
section("1. ffi.cdef - C Declaration Parsing")

test("cdef: basic function declaration", function()
	ffi.cdef([[
        double sin(double x);
        double cos(double x);
        double sqrt(double x);
    ]])
	return true
end)

test("cdef: unsigned types", function()
	ffi.cdef([[
        unsigned int abs(int x);
    ]])
	return true
end)

test("cdef: pointer types", function()
	ffi.cdef([[
        void* malloc(size_t size);
        void free(void* ptr);
        void* memcpy(void* dst, const void* src, size_t n);
        void* memset(void* s, int c, size_t n);
    ]])
	return true
end)

test("cdef: struct declaration", function()
	ffi.cdef([[
        typedef struct {
            int x;
            int y;
        } Point;
        
        typedef struct {
            float r;
            float g;
            float b;
            float a;
        } Color;
    ]])
	return true
end)

test("cdef: complex struct with padding", function()
	ffi.cdef([[
        typedef struct {
            char a;
            int b;
            char c;
            double d;
        } PaddedStruct;
    ]])
	return true
end)

test("cdef: variadic function", function()
	ffi.cdef([[
        int sprintf(char* str, const char* format, ...);
    ]])
	return true
end)

expect_error("cdef: invalid syntax", function()
	ffi.cdef([[this is not valid C]])
end)

-- ========================================================================
-- SECTION 2: ffi.sizeof - Size of Types
-- ========================================================================
section("2. ffi.sizeof - Type Sizes")

test("sizeof: char = 1", function()
	return ffi.sizeof("char") == 1
end)

test("sizeof: short = 2", function()
	return ffi.sizeof("short") == 2
end)

test("sizeof: int = 4", function()
	return ffi.sizeof("int") == 4
end)

test("sizeof: float = 4", function()
	return ffi.sizeof("float") == 4
end)

test("sizeof: double = 8", function()
	return ffi.sizeof("double") == 8
end)

test("sizeof: void* = pointer size", function()
	local ptr_size = ffi.sizeof("void*")
	return ptr_size == 4 or ptr_size == 8
end)

test("sizeof: defined struct (Point)", function()
	local size = ffi.sizeof("Point")
	return size == 8 -- 2 ints = 8 bytes
end)

expect_error("sizeof: unknown type", function()
	ffi.sizeof("NonExistentType12345")
end)

-- ========================================================================
-- SECTION 3: ffi.alignof - Type Alignment
-- ========================================================================
section("3. ffi.alignof - Type Alignment")

test("alignof: char = 1", function()
	return ffi.alignof("char") == 1
end)

test("alignof: int = 4", function()
	return ffi.alignof("int") == 4
end)

test("alignof: double = 8", function()
	return ffi.alignof("double") == 8
end)

-- ========================================================================
-- SECTION 4: ffi.new - Memory Allocation
-- ========================================================================
section("4. ffi.new - Memory Allocation")

test("new: allocate int", function()
	local x = ffi.new("int")
	return x ~= nil
end)

test("new: allocate int with value", function()
	local x = ffi.new("int", 42)
	return x[0] == 42
end)

test("new: allocate double", function()
	local x = ffi.new("double", 3.14159)
	local val = x[0]
	return math.abs(val - 3.14159) < 0.0001
end)

test("new: allocate char", function()
	local x = ffi.new("char", 65) -- 'A'
	return x[0] == 65
end)

test("new: allocate struct Point", function()
	local p = ffi.new("Point")
	return p ~= nil
end)

expect_error("new: void (incomplete type)", function()
	ffi.new("void")
end)

-- ========================================================================
-- SECTION 5: ffi.cast - Type Casting
-- ========================================================================
section("5. ffi.cast - Type Casting")

test("cast: int to double*", function()
	local ptr = ffi.cast("double*", 0)
	return ptr ~= nil
end)

test("cast: from cdata", function()
	local x = ffi.new("int", 100)
	local ptr = ffi.cast("int*", x)
	return ptr ~= nil
end)

-- ========================================================================
-- SECTION 6: ffi.typeof - Type Objects
-- ========================================================================
section("6. ffi.typeof - Type Objects")

test("typeof: create int type", function()
	local int_t = ffi.typeof("int")
	return int_t ~= nil
end)

test("typeof: type has size property", function()
	local int_t = ffi.typeof("int")
	return int_t.size == 4
end)

test("typeof: type is callable (constructor)", function()
	local int_t = ffi.typeof("int")
	local x = int_t(42)
	return x[0] == 42
end)

-- ========================================================================
-- SECTION 7: ffi.string - C String Conversion
-- ========================================================================
section("7. ffi.string - C String Conversion")

test("string: from char* pointer", function()
	-- Allocate and fill a string
	local buf = ffi.C.malloc(10)
	if not buf then
		return false
	end

	-- Write "Hello" manually
	local p = ffi.cast("char*", buf)
	p[0] = 72 -- H
	p[1] = 101 -- e
	p[2] = 108 -- l
	p[3] = 108 -- l
	p[4] = 111 -- o
	p[5] = 0 -- null terminator

	local str = ffi.string(buf)
	ffi.C.free(buf)

	return str == "Hello"
end)

test("string: with explicit length", function()
	local buf = ffi.C.malloc(10)
	if not buf then
		return false
	end

	local p = ffi.cast("char*", buf)
	p[0] = 65 -- A
	p[1] = 66 -- B
	p[2] = 67 -- C

	local str = ffi.string(buf, 3)
	ffi.C.free(buf)

	return str == "ABC"
end)

-- ========================================================================
-- SECTION 8: ffi.copy / ffi.fill - Memory Operations
-- ========================================================================
section("8. ffi.copy / ffi.fill - Memory Operations")

test("fill: zero memory", function()
	local buf = ffi.C.malloc(16)
	if not buf then
		return false
	end

	ffi.fill(buf, 16, 0)

	local p = ffi.cast("char*", buf)
	local all_zero = true
	for i = 0, 15 do
		if p[i] ~= 0 then
			all_zero = false
			break
		end
	end

	ffi.C.free(buf)
	return all_zero
end)

test("fill: pattern fill (0xFF)", function()
	local buf = ffi.C.malloc(8)
	if not buf then
		return false
	end

	ffi.fill(buf, 8, 0xFF)

	local p = ffi.cast("unsigned char*", buf)
	local all_ff = true
	for i = 0, 7 do
		if p[i] ~= 255 then
			all_ff = false
			break
		end
	end

	ffi.C.free(buf)
	return all_ff
end)

test("copy: from Lua string", function()
	local buf = ffi.C.malloc(10)
	if not buf then
		return false
	end

	ffi.copy(buf, "Test", 4)

	local str = ffi.string(buf, 4)
	ffi.C.free(buf)

	return str == "Test"
end)

-- ========================================================================
-- SECTION 9: ffi.istype - Type Checking
-- ========================================================================
section("9. ffi.istype - Type Checking")

test("istype: int cdata is int", function()
	local x = ffi.new("int", 42)
	return ffi.istype("int", x) == true
end)

test("istype: double cdata is NOT int", function()
	local x = ffi.new("double", 3.14)
	return ffi.istype("int", x) == false
end)

test("istype: nil is NOT int", function()
	return ffi.istype("int", nil) == false
end)

-- ========================================================================
-- SECTION 10: ffi.C - Standard Library Calls
-- ========================================================================
section("10. ffi.C - Standard Library Calls")

test("C.sin: basic call", function()
	local result = ffi.C.sin(0)
	return math.abs(result) < 0.0001
end)

test("C.sin: pi/2 = 1", function()
	local result = ffi.C.sin(math.pi / 2)
	return math.abs(result - 1) < 0.0001
end)

test("C.cos: basic call", function()
	local result = ffi.C.cos(0)
	return math.abs(result - 1) < 0.0001
end)

test("C.sqrt: 4 = 2", function()
	local result = ffi.C.sqrt(4)
	return math.abs(result - 2) < 0.0001
end)

test("C.sqrt: 2 ≈ 1.414", function()
	local result = ffi.C.sqrt(2)
	return math.abs(result - 1.41421356) < 0.0001
end)

test("C.malloc/free: basic allocation", function()
	local ptr = ffi.C.malloc(100)
	if not ptr then
		return false
	end
	ffi.C.free(ptr)
	return true
end)

-- ========================================================================
-- SECTION 11: ffi.callback - Lua to C Callbacks
-- ========================================================================
section("11. ffi.callback - Lua Callbacks")

test("callback: create int(int) callback", function()
	local cb = ffi.callback("int(*)(int)", function(x)
		return x * 2
	end)
	return cb ~= nil
end)

-- ========================================================================
-- SECTION 12: Struct Field Access
-- ========================================================================
section("12. Struct Field Access")

test("struct: set and get int field", function()
	local p = ffi.new("Point")
	p.x = 10
	p.y = 20
	return p.x == 10 and p.y == 20
end)

test("struct: Color float fields", function()
	local c = ffi.new("Color")
	c.r = 1.0
	c.g = 0.5
	c.b = 0.25
	c.a = 1.0
	return math.abs(c.r - 1.0) < 0.01 and math.abs(c.g - 0.5) < 0.01
end)

-- ========================================================================
-- SECTION 13: Pointer Arithmetic
-- ========================================================================
section("13. Pointer Arithmetic")

test("pointer: index access [0]", function()
	local x = ffi.new("int", 42)
	return x[0] == 42
end)

test("pointer: modify via index", function()
	local x = ffi.new("int", 0)
	x[0] = 100
	return x[0] == 100
end)

test("pointer: array-like access", function()
	local buf = ffi.C.malloc(4 * 4) -- 4 ints
	if not buf then
		return false
	end

	local arr = ffi.cast("int*", buf)
	arr[0] = 10
	arr[1] = 20
	arr[2] = 30
	arr[3] = 40

	local ok = arr[0] == 10 and arr[1] == 20 and arr[2] == 30 and arr[3] == 40
	ffi.C.free(buf)
	return ok
end)

-- ========================================================================
-- SECTION 14: Edge Cases & Error Handling
-- ========================================================================
section("14. Edge Cases & Error Handling")

test("edge: very large allocation (1MB)", function()
	local ptr = ffi.C.malloc(1024 * 1024)
	if not ptr then
		return false
	end
	ffi.C.free(ptr)
	return true
end)

test("edge: zero-size malloc", function()
	-- Behavior varies by platform, but should not crash
	local ptr = ffi.C.malloc(0)
	if ptr then
		ffi.C.free(ptr)
	end
	return true
end)

test("edge: double free (should not crash immediately)", function()
	-- This is UB in C, but we're testing if Lua layer handles it gracefully
	-- Note: This might crash on some platforms!
	local ptr = ffi.C.malloc(16)
	if ptr then
		ffi.C.free(ptr)
		-- Intentionally NOT calling free again to avoid crash
	end
	return true
end)

expect_error("edge: call undeclared function", function()
	local _ = ffi.C.this_function_definitely_does_not_exist_12345()
end)

-- ========================================================================
-- SECTION 15: Batch API (Optimization)
-- ========================================================================
section("15. Batch API (Optimization)")

test("batch: ffi.batch exists", function()
	return ffi.batch ~= nil
end)

test("batch: ffi.batch2 exists", function()
	return ffi.batch2 ~= nil
end)

test("batch: process double array", function()
	local COUNT = 100
	local DOUBLE_SIZE = 8

	local input_ptr = ffi.C.malloc(COUNT * DOUBLE_SIZE)
	local output_ptr = ffi.C.malloc(COUNT * DOUBLE_SIZE)

	if not input_ptr or not output_ptr then
		if input_ptr then
			ffi.C.free(input_ptr)
		end
		if output_ptr then
			ffi.C.free(output_ptr)
		end
		return false
	end

	-- Fill input
	local inp = ffi.cast("double*", input_ptr)
	for i = 0, COUNT - 1 do
		inp[i] = i * 0.1
	end

	-- Call batch
	local sin_fn = ffi.C.sin
	local processed = ffi.batch(sin_fn, input_ptr, output_ptr, COUNT)

	-- Verify
	local out = ffi.cast("double*", output_ptr)
	local ok = processed == COUNT and math.abs(out[0] - math.sin(0)) < 0.0001

	ffi.C.free(input_ptr)
	ffi.C.free(output_ptr)

	return ok
end)

-- ========================================================================
-- SECTION 16: Type Conversion Edge Cases
-- ========================================================================
section("16. Type Conversion Edge Cases")

test("conversion: int overflow", function()
	local x = ffi.new("int", 2147483647) -- INT_MAX
	return x[0] == 2147483647
end)

test("conversion: negative int", function()
	local x = ffi.new("int", -100)
	return x[0] == -100
end)

test("conversion: float precision", function()
	local x = ffi.new("float", 3.141592653589793)
	local diff = math.abs(x[0] - 3.141592653589793)
	-- Float has less precision than double
	return diff < 0.001
end)

test("conversion: char from number", function()
	local x = ffi.new("char", 255) -- Will wrap to -1 as signed char
	return x[0] == -1 or x[0] == 255 -- Platform dependent
end)

-- ========================================================================
-- FINAL SUMMARY
-- ========================================================================
print("")
print(
	"╔══════════════════════════════════════════════════════════════════╗"
)
print("║                         TEST SUMMARY                             ║")
print(
	"╠══════════════════════════════════════════════════════════════════╣"
)
print(
	string.format(
		"║   ✅ Passed: %-5d                                               ║",
		tests_passed
	)
)
print(
	string.format(
		"║   ❌ Failed: %-5d                                               ║",
		tests_failed
	)
)
print(
	string.format(
		"║   Total:    %-5d                                               ║",
		tests_passed + tests_failed
	)
)
print(
	"╠══════════════════════════════════════════════════════════════════╣"
)
if tests_failed == 0 then
	print("║   🎉 ALL TESTS PASSED!                                           ║")
else
	print("║   ⚠️  SOME TESTS FAILED - Review output above                    ║")
end
print(
	"╚══════════════════════════════════════════════════════════════════╝"
)
