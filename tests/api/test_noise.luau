-- Test Noise
print("[TEST] Noise")

local noise = require("@lux/noise")

-- Set seed for reproducibility
noise.setSeed(12345)

-- Perlin 2D
local p1 = noise.perlin2(0, 0)
assert(type(p1) == "number", "perlin2 should return number")
assert(p1 >= -1 and p1 <= 1, "perlin2 should be in [-1, 1]")

-- Different positions should give different values
local p2 = noise.perlin2(1.5, 2.5)
assert(type(p2) == "number", "perlin2 at different pos failed")

-- Perlin 3D
local p3 = noise.perlin3(0.5, 0.5, 0.5)
assert(type(p3) == "number", "perlin3 should return number")
assert(p3 >= -1 and p3 <= 1, "perlin3 should be in [-1, 1]")

-- Simplex 2D
local s1 = noise.simplex2(0, 0)
assert(type(s1) == "number", "simplex2 should return number")

-- Simplex 3D
local s2 = noise.simplex3(1, 2, 3)
assert(type(s2) == "number", "simplex3 should return number")

-- FBM 2D
local fbm1 = noise.fbm2(0.5, 0.5, 4) -- 4 octaves
assert(type(fbm1) == "number", "fbm2 should return number")

-- FBM 3D with custom params
local fbm2 = noise.fbm3(0.5, 0.5, 0.5, 6, 2.0, 0.5) -- 6 octaves, lacunarity 2.0, gain 0.5
assert(type(fbm2) == "number", "fbm3 should return number")

-- Continuity test - nearby values should be similar
local a = noise.perlin2(0.5, 0.5)
local b = noise.perlin2(0.51, 0.51)
local diff = math.abs(a - b)
assert(diff < 0.5, "Noise should be continuous, diff: " .. diff)

print("[PASS] Noise")
