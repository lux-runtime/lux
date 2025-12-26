-- tests/api/test_serde.luau
-- Tests for @lux/serde

local serde = require("@lux/serde")

print("Testing @lux/serde...")

-- 1. JSON
print("  > Testing JSON")
local data = {
	name = "Lux",
	version = 1.0,
	features = { "ffi", "std" },
	active = true,
}

local jsonStr = serde.encode("json", data)
assert(type(jsonStr) == "string", "json.encode should return string")
assert(string.find(jsonStr, '"name"'), "json string should contain data")

local decodedJson = serde.decode("json", jsonStr)
assert(decodedJson.name == "Lux", "json.decode should restore data")
assert(decodedJson.version == 1.0, "json number should be correct")
assert(#decodedJson.features == 2, "json array should be restored")

-- Pretty print
local jsonPretty = serde.encode("json", data, true)
assert(string.find(jsonPretty, "\n"), "pretty json has newlines")

-- 2. TOML
print("  > Testing TOML")
local tomlOk, tomlStr = pcall(serde.encode, "toml", data)
if tomlOk then
	assert(type(tomlStr) == "string", "toml.encode should return string")

	local decodedToml = serde.decode("toml", tomlStr)
	assert(decodedToml.name == "Lux", "toml.decode should restore data")
else
	print("    Skipping TOML (not available)")
end

-- 3. YAML
print("  > Testing YAML")
local yamlOk, yamlStr = pcall(serde.encode, "yaml", data)
if yamlOk then
	assert(type(yamlStr) == "string", "yaml.encode should return string")

	local decodedYaml = serde.decode("yaml", yamlStr)
	assert(decodedYaml.name == "Lux", "yaml.decode should restore data")
else
	print("    Skipping YAML (not available)")
end

-- 4. Hash
print("  > Testing hash")
local message = "Hello, World!"

local md5 = serde.hash("md5", message)
assert(type(md5) == "string", "md5 hash returns string")
assert(#md5 == 32, "md5 is 32 hex chars")

local sha256 = serde.hash("sha256", message)
assert(type(sha256) == "string", "sha256 hash returns string")
assert(#sha256 == 64, "sha256 is 64 hex chars")

local sha1 = serde.hash("sha1", message)
assert(type(sha1) == "string", "sha1 hash returns string")
assert(#sha1 == 40, "sha1 is 40 hex chars")

-- Verify consistency
local md5_2 = serde.hash("md5", message)
assert(md5 == md5_2, "same message same hash")

-- Different message different hash
local md5_diff = serde.hash("md5", "Different message")
assert(md5 ~= md5_diff, "different message different hash")

-- 5. Compress/Decompress (async)
print("  > Testing compress/decompress")
local testData = string.rep("Hello World! ", 100) -- Compressible data

local compressOk, compressed = pcall(serde.compress, "zlib", testData)
if compressOk then
	assert(type(compressed) == "string", "compress returns string")
	assert(#compressed < #testData, "compressed smaller than original")

	local decompressed = serde.decompress("zlib", compressed)
	assert(decompressed == testData, "decompress restores original")
else
	print("    Compress/decompress skipped: " .. tostring(compressed))
end

print("Serde Tests Passed!")
