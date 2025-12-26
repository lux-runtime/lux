-- tests/api/test_signal.luau
-- Tests for @lux/signal

local Signal = require("@lux/signal")

print("Testing @lux/signal...")

-- 1. Create Signal
local sig = Signal.new()
assert(type(sig) == "userdata", "Signal.new() returns userdata")

-- 2. Connect
local count = 0
local conn = sig:Connect(function(val)
	count = count + (val or 1)
end)

assert(conn, "Connect returns connection")
assert(conn.Connected, "Connection is active")

-- 3. Fire
sig:Fire(2)
assert(count == 2, "Fire should trigger callback")

-- 4. Disconnect
conn:Disconnect()
assert(not conn.Connected, "Disconnect updates state")
sig:Fire(1)
assert(count == 2, "Disconnected should not trigger")

-- 5. ConnectOnce (Method name is 'Once')
local oneShot = 0
sig:Once(function()
	oneShot = oneShot + 1
end)

sig:Fire()
sig:Fire()
assert(oneShot == 1, "Once runs only once")

print("Signal Tests Passed!")
