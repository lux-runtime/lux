-- Test Vector2 and Vector3
print("[TEST] Vector types")

-- Vector2
local v1 = Vector2.new(3, 4)
assert(v1.X == 3, "Vector2.X failed")
assert(v1.Y == 4, "Vector2.Y failed")
assert(v1.Magnitude == 5, "Vector2.Magnitude failed")

local unit = v1.Unit
assert(unit.X == 0.6, "Vector2.Unit.X failed")
assert(unit.Y == 0.8, "Vector2.Unit.Y failed")

local v2 = Vector2.new(1, 2)
local lerped = v1:Lerp(v2, 0.5)
assert(lerped.X == 2, "Vector2.Lerp.X failed")
assert(lerped.Y == 3, "Vector2.Lerp.Y failed")

assert(v1:Dot(v2) == 11, "Vector2.Dot failed") -- 3*1 + 4*2 = 11
assert(v1:Cross(v2) == 2, "Vector2.Cross failed") -- 3*2 - 4*1 = 2

-- Vector2 operators
local sum = v1 + v2
assert(sum.X == 4 and sum.Y == 6, "Vector2 + failed")
local diff = v1 - v2
assert(diff.X == 2 and diff.Y == 2, "Vector2 - failed")
local scaled = v1 * 2
assert(scaled.X == 6 and scaled.Y == 8, "Vector2 * failed")
local divided = v1 / 2
assert(divided.X == 1.5 and divided.Y == 2, "Vector2 / failed")
local neg = -v1
assert(neg.X == -3 and neg.Y == -4, "Vector2 unary - failed")

-- Vector2 constants
assert(Vector2.zero.X == 0 and Vector2.zero.Y == 0, "Vector2.zero failed")
assert(Vector2.one.X == 1 and Vector2.one.Y == 1, "Vector2.one failed")

-- Vector3
local v3 = Vector3.new(1, 2, 3)
assert(v3.X == 1 and v3.Y == 2 and v3.Z == 3, "Vector3 properties failed")

local v4 = Vector3.new(4, 5, 6)
local cross = v3:Cross(v4)
-- (2*6 - 3*5, 3*4 - 1*6, 1*5 - 2*4) = (-3, 6, -3)
assert(cross.X == -3 and cross.Y == 6 and cross.Z == -3, "Vector3.Cross failed")

local dot3 = v3:Dot(v4)
assert(dot3 == 32, "Vector3.Dot failed") -- 1*4 + 2*5 + 3*6 = 32

assert(Vector3.zero.X == 0, "Vector3.zero failed")
assert(Vector3.one.Z == 1, "Vector3.one failed")

print("[PASS] Vector types")
