-- Test UDim types
print("[TEST] UDim types")

-- UDim
local udim = UDim.new(0.5, 100)
assert(udim.Scale == 0.5, "UDim.Scale failed")
assert(udim.Offset == 100, "UDim.Offset failed")

-- UDim2
local udim2 = UDim2.new(0.5, 10, 0.25, 20)
assert(udim2.X.Scale == 0.5, "UDim2.X.Scale failed")
assert(udim2.X.Offset == 10, "UDim2.X.Offset failed")
assert(udim2.Y.Scale == 0.25, "UDim2.Y.Scale failed")
assert(udim2.Y.Offset == 20, "UDim2.Y.Offset failed")

-- UDim2.fromScale
local scaled = UDim2.fromScale(0.5, 0.5)
assert(scaled.X.Scale == 0.5, "UDim2.fromScale X failed")
assert(scaled.X.Offset == 0, "UDim2.fromScale X offset should be 0")

-- UDim2.fromOffset
local offset = UDim2.fromOffset(100, 200)
assert(offset.X.Offset == 100, "UDim2.fromOffset X failed")
assert(offset.Y.Offset == 200, "UDim2.fromOffset Y failed")
assert(offset.X.Scale == 0, "UDim2.fromOffset scale should be 0")

-- Rect
local rect = Rect.new(10, 20, 110, 220)
assert(rect.Width == 100, "Rect.Width failed")
assert(rect.Height == 200, "Rect.Height failed")

-- Rect with inverted coords (should auto-correct)
local rect2 = Rect.new(100, 100, 0, 0)
assert(rect2.Width == 100, "Rect inverted Width failed")

-- NumberRange
local range = NumberRange.new(0, 100)
assert(range.Min == 0, "NumberRange.Min failed")
assert(range.Max == 100, "NumberRange.Max failed")

-- NumberRange with inverted (should auto-correct)
local range2 = NumberRange.new(100, 0)
assert(range2.Min == 0, "NumberRange inverted Min failed")
assert(range2.Max == 100, "NumberRange inverted Max failed")

print("[PASS] UDim types")
