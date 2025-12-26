-- Test Color3
print("[TEST] Color3")

-- Basic constructor
local red = Color3.new(1, 0, 0)
assert(red.R == 1, "Color3.R failed")
assert(red.G == 0, "Color3.G failed")
assert(red.B == 0, "Color3.B failed")

-- fromRGB
local white = Color3.fromRGB(255, 255, 255)
assert(white.R == 1, "Color3.fromRGB R failed")
assert(white.G == 1, "Color3.fromRGB G failed")
assert(white.B == 1, "Color3.fromRGB B failed")

local half = Color3.fromRGB(128, 128, 128)
assert(half.R > 0.49 and half.R < 0.51, "Color3.fromRGB half failed")

-- fromHex
local hexColor = Color3.fromHex("#FF0000")
assert(hexColor.R == 1, "Color3.fromHex R failed")
assert(hexColor.G == 0, "Color3.fromHex G failed")

local hexShort = Color3.fromHex("FFF")
assert(hexShort.R == 1, "Color3.fromHex short failed")

-- fromHSV
local hsvRed = Color3.fromHSV(0, 1, 1)
assert(hsvRed.R == 1, "Color3.fromHSV R failed")
assert(hsvRed.G == 0, "Color3.fromHSV G failed")

-- Lerp
local black = Color3.new(0, 0, 0)
local lerped = black:Lerp(white, 0.5)
assert(lerped.R == 0.5, "Color3.Lerp failed")

-- ToHSV
local h, s, v = red:ToHSV()
assert(h == 0, "Color3.ToHSV H failed")
assert(s == 1, "Color3.ToHSV S failed")
assert(v == 1, "Color3.ToHSV V failed")

-- ToHex
local hex = red:ToHex()
assert(hex == "FF0000", "Color3.ToHex failed, got: " .. hex)

print("[PASS] Color3")
