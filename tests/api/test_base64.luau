-- Test Base64
print("[TEST] Base64")

local base64 = require("@lux/base64")

-- Encode string
local encoded = base64.encode("Hello World")
assert(type(encoded) == "string", "base64.encode should return string")
assert(encoded == "SGVsbG8gV29ybGQ=", "base64.encode failed: " .. encoded)

-- Decode
local decoded = base64.decode(encoded)
assert(type(decoded) == "buffer", "base64.decode should return buffer")
assert(buffer.tostring(decoded) == "Hello World", "base64.decode failed")

-- URL-safe encode
local urlEncoded = base64.encodeUrl("path/to/file?query=1")
assert(type(urlEncoded) == "string", "base64.encodeUrl should return string")
-- URL-safe should not contain + or /
assert(not string.find(urlEncoded, "+"), "URL-safe should not contain +")
assert(not string.find(urlEncoded, "/"), "URL-safe should not contain /")

-- URL-safe decode
local urlDecoded = base64.decodeUrl(urlEncoded)
assert(buffer.tostring(urlDecoded) == "path/to/file?query=1", "base64.decodeUrl failed")

-- Empty string
local emptyEncoded = base64.encode("")
assert(emptyEncoded == "", "base64.encode empty failed")

-- Binary data via buffer
local binData = buffer.create(4)
buffer.writeu8(binData, 0, 0xFF)
buffer.writeu8(binData, 1, 0x00)
buffer.writeu8(binData, 2, 0xAB)
buffer.writeu8(binData, 3, 0xCD)

local binEncoded = base64.encode(binData)
local binDecoded = base64.decode(binEncoded)
assert(buffer.readu8(binDecoded, 0) == 0xFF, "Binary decode failed")
assert(buffer.readu8(binDecoded, 2) == 0xAB, "Binary decode failed at pos 2")

print("[PASS] Base64")
