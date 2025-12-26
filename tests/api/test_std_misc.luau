-- tests/api/test_std_misc.luau
-- Tests for @lux/regex and global DateTime

local regex = require("@lux/regex")

print("Testing @lux/regex & Global DateTime...")

-- === REGEX ===
print("  > Regex")
local pattern = regex.new([[(\d+)-(\w+)]])
local text = "123-abc"
local match = pattern:find(text)

assert(match, "regex match found")
-- match is UserData (LuaMatch)

-- Test Replace (Instance method)
local replacer = regex.new([[(\d+)]])
local replaced = replacer:replace(text, "NUM")
assert(replaced == "NUM-abc", "regex replace works")

-- === DATETIME ===
print("  > DateTime")
if type(DateTime) == "table" then
	local now = DateTime.now()
	assert(type(now) == "userdata", "DateTime.now() returns userdata")

	-- Unix Timestamp
	local ts = now.UnixTimestamp
	assert(type(ts) == "number", "unix timestamp is number")

	-- ISO
	local iso = now:ToIsoDate()
	assert(type(iso) == "string", "iso date is string")
	-- check format roughly YYYY-MM-DD
	assert(string.match(iso, "%d%d%d%d%-%d%d%-%d%d"), "iso format seems correct")

	local fromIso = DateTime.fromIsoDate(iso)
	assert(fromIso ~= nil, "parsing iso date works")
	assert(math.abs(fromIso.UnixTimestamp - ts) < 2, "round trip iso works (roughly)")
else
	print("SKIP: Global DateTime not found")
end

print("Std Misc Tests Passed!")
