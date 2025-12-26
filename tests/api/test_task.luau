-- tests/api/test_task.luau
-- Tests for global task library

print("Testing Global task...")

assert(type(task) == "table", "global 'task' should exist")

local output = {}

-- 1. Spawn
task.spawn(function()
	table.insert(output, "spawned")
end)

-- 2. Defer
task.defer(function()
	table.insert(output, "deferred")
end)

-- Wait a bit to let tasks run
task.wait(0.1)

assert(output[1] == "spawned", "spawn should run first")
-- deferred runs at end of cycle
assert(output[2] == "deferred" or output[#output] == "deferred", "deferred should run")

-- 3. Delay
local start = os.clock()
task.delay(0.2, function()
	table.insert(output, "delayed")
end)

task.wait(0.3)
assert(output[#output] == "delayed", "delay task should have run")
local elapsed = os.clock() - start
assert(elapsed >= 0.2, "delay should wait at least specified time")

print("Task Tests Passed!")
