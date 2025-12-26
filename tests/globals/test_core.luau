-- tests/globals/init.luau
-- Tests for Lux Core Global Environment

print("Testing Globals...")

-- 1. Check _VERSION
assert(type(_VERSION) == "string", "_VERSION should be a string")
print("  _VERSION: " .. _VERSION)

-- 2. Check _G
assert(type(_G) == "table", "_G should be a table")
-- Note: In sandboxed Luau environments, _G._G may not be set

-- 3. Check Basic Functions
assert(type(print) == "function", "print should be a function")
assert(type(error) == "function", "error should be a function")
assert(type(assert) == "function", "assert should be a function")
assert(type(type) == "function", "type should be a function")
assert(type(tostring) == "function", "tostring should be a function")
assert(type(tonumber) == "function", "tonumber should be a function")

-- 4. Check Require
assert(type(require) == "function", "require should be a function")

-- 5. Check Lux Global (if applicable, based on previous code)
-- If lux is registered as a global or package
local has_lux_pkg, lux = pcall(require, "@lux")
if has_lux_pkg then
	print("  @lux package found")
end

-- 6. Test Error Handling Fundamentals
local status, err = pcall(function()
	error("test error")
end)
assert(status == false, "pcall should catch error")
assert(string.find(err, "test error"), "error message should be preserved")

print("Globals Tests Passed!")
