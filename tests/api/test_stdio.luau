-- tests/api/test_stdio.luau
-- Tests for @lux/stdio

local stdio = require("@lux/stdio")

print("Testing @lux/stdio...")

-- 1. stdio.format
print("  > Testing stdio.format")
local formatted = stdio.format("hello", 123, true, { key = "value" })
assert(type(formatted) == "string", "format returns string")
assert(string.find(formatted, "hello"), "should contain hello")
assert(string.find(formatted, "123"), "should contain 123")

-- 2. stdio.color
print("  > Testing stdio.color")
local red = stdio.color("red")
assert(type(red) == "string", "color returns string")
assert(string.find(red, "\27%["), "should be ANSI escape sequence")

local green = stdio.color("green")
assert(type(green) == "string", "green color")

local blue = stdio.color("blue")
assert(type(blue) == "string", "blue color")

local reset = stdio.color("reset")
assert(type(reset) == "string", "reset color")

-- Test available colors
local colors = { "black", "red", "green", "yellow", "blue", "purple", "cyan", "white" }
for _, color in ipairs(colors) do
	local result = stdio.color(color)
	assert(type(result) == "string", "color " .. color .. " should return string")
end

-- 3. stdio.style
print("  > Testing stdio.style")
local bold = stdio.style("bold")
assert(type(bold) == "string", "bold style")
assert(string.find(bold, "\27%["), "should be ANSI sequence")

local dim = stdio.style("dim")
assert(type(dim) == "string", "dim style")

-- Test available styles
local styles = { "bold", "dim" }
for _, style in ipairs(styles) do
	local result = stdio.style(style)
	assert(type(result) == "string", "style " .. style .. " should return string")
end

-- 4. Print with colors (visual test)
print("  > Visual test: " .. stdio.color("red") .. "RED" .. stdio.color("reset") .. " text")
print("  > Visual test: " .. stdio.style("bold") .. "BOLD" .. stdio.color("reset") .. " text")

-- Note: stdin functions (readLine, readToEnd) require user input
-- and write/ewrite are tested implicitly by print

print("Stdio Tests Passed!")
