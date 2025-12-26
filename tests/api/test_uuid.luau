-- Test UUID
print("[TEST] UUID")

local uuid = require("@lux/uuid")

-- v4 (random)
local id1 = uuid.v4()
assert(#id1 == 36, "UUID v4 length failed: " .. #id1)
assert(
	string.match(id1, "^%x%x%x%x%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%x%x%x%x%x%x%x%x$"),
	"UUID v4 format failed"
)

-- v7 (timestamp-based)
local id2 = uuid.v7()
assert(#id2 == 36, "UUID v7 length failed")

-- Uniqueness
local id3 = uuid.v4()
assert(id1 ~= id3, "UUID v4 should be unique")

-- isValid
assert(uuid.isValid(id1), "UUID isValid should be true")
assert(uuid.isValid("550e8400-e29b-41d4-a716-446655440000"), "UUID isValid standard failed")
assert(not uuid.isValid("not-a-uuid"), "UUID isValid should be false for invalid")
assert(not uuid.isValid(""), "UUID isValid should be false for empty")

-- nil constant
assert(uuid["nil"] == "00000000-0000-0000-0000-000000000000", "UUID nil constant failed")

-- parse and format
local buf = uuid.parse(id1)
if buf then
	assert(#buffer.tostring(buf) == 16, "UUID parse should return 16 bytes")
	local formatted = uuid.format(buf)
	assert(formatted == id1, "UUID round-trip failed")
end

print("[PASS] UUID")
