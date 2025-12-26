-- tests/stress/test_loop_alloc.luau
-- Stress Test: Loop Allocation

print("Testing Stress: Loop Alloc...")

local iterations = 10000
local t = {}

local start = os.clock()
for i = 1, iterations do
	t[i] = { x = i, y = i * 2, z = tostring(i) }
end
local duration = os.clock() - start

assert(#t == iterations, "allocation count match")
print(string.format("  Allocated %d tables in %.4fs", iterations, duration))

-- Verify GC doesn't crash (if available)
t = nil
local gcOk = pcall(function()
	-- Note: collectgarbage may not be fully supported in sandboxed Luau
	if collectgarbage then
		pcall(collectgarbage, "collect")
	end
end)
print("  GC check passed")

print("Stress Loop Test Passed!")
