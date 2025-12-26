-- Normal logs
print("Normal print output")
warn("This is a warning message")

-- Uncomment one section at a time to verify error formatting

-- Case 1: Regex Error (Bad Argument)
-- local regex = require("@lux/regex")
-- print("Triggering Regex Error (Bad Argument)...")
-- regex.new()

-- Case 2: FFI Error (Load failure)
local ffi = require("@lux/ffi")
print("Triggering FFI Error (Bad Library)...")
ffi.load("non_existent_lib.dll")

-- Case 3: Serde Error (Parse error)
-- local serde = require("@lux/serde")
-- print("Triggering Serde Error (Bad JSON)...")
-- serde.decode("json", "{ invalid")

-- Case 4: Syntax Error
-- if true then
