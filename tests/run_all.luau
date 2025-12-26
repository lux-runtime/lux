-- tests/run_all.luau
-- Comprehensive Test Runner for Lux Runtime
-- Usage: lux tests/run_all.luau [optional_filter]

local fs = require("@lux/fs")
local process = require("@lux/process")
local stdio = require("@lux/stdio")

-- Utility for ANSI colors
local function style(name)
	local codes = {
		reset = "\27[0m",
		bold = "\27[1m",
		red = "\27[31m",
		green = "\27[32m",
		yellow = "\27[33m",
		blue = "\27[34m",
		magenta = "\27[35m",
		cyan = "\27[36m",
		gray = "\27[90m",
	}
	return codes[name] or ""
end

-- State
local stats = {
	total = 0,
	passed = 0,
	failed = 0,
	skipped = 0,
	suites = 0,
}

local failures = {}

-- Find Lux Executable
local function findLuxExecutable()
	local candidates = {
		"target/debug/lux.exe",
		"target/release/lux.exe",
		"lux.exe",
	}
	for _, p in ipairs(candidates) do
		if fs.isFile(p) then
			return p
		end
	end
	-- Fallback to assuming 'lux' is in PATH
	return "lux"
end

local LUX_EXE = findLuxExecutable()

-- Helper to print header
local function printHeader()
	print(
		style("bold")
			.. style("magenta")
			.. "🚀 Lux Runtime Comprehensive Test Suite"
			.. style("reset")
	)
	print(style("gray") .. "Executable: " .. LUX_EXE .. style("reset"))
	print(string.rep("-", 50))
end

-- Run a single test file using subprocess
local function runTestFile(path)
	stats.total = stats.total + 1

	local start = os.clock()
	-- Run the test file in a separate process to ensure isolation
	local result = process.exec(LUX_EXE, { "run", path }, {
		stdio = "inherit", -- Capture output
	})
	local duration = (os.clock() - start) * 1000

	if result.ok and result.code == 0 then
		stats.passed = stats.passed + 1
		print(
			string.format(
				"%s PASS %s %s(%.2fms)%s",
				style("green"),
				path,
				style("gray"),
				duration,
				style("reset")
			)
		)
		return true
	else
		stats.failed = stats.failed + 1
		print(
			string.format(
				"%s FAIL %s %s(Code: %d)%s",
				style("red"),
				path,
				style("gray"),
				result.code,
				style("reset")
			)
		)

		-- Store failure details
		table.insert(failures, {
			path = path,
			stdout = result.stdout or "",
			stderr = result.stderr or "",
			code = result.code,
		})
		return false
	end
end

-- Recursively scan and run tests
local function scanAndRun(dir, filter)
	if not fs.isDir(dir) then
		return
	end

	local entries = fs.readDir(dir)
	-- Sort entries for deterministic order
	table.sort(entries)

	for _, name in ipairs(entries) do
		local fullPath = dir .. "/" .. name

		if fs.isDir(fullPath) then
			-- Recurse
			scanAndRun(fullPath, filter)
		elseif string.match(name, "%.luau$") then
			-- Run .luau files, skipping init.luau unless explicitly handled (usually implicit)
			-- We typically run standalone files. If a directory has init.luau, it might be a module not a test.
			-- Convention: test files should be named `test_*.luau` or just be in the test dir.
			-- We run EVERYTHING in these folders.

			if filter and not string.match(fullPath, filter) then
				stats.skipped = stats.skipped + 1
			else
				runTestFile(fullPath)
			end
		end
	end
end

-- Main Execution
local args = { ... } -- process.args available? Need to verify how to get args in main script.
-- For now, using varargs if this is main chunk, or process.args logic if added later.

printHeader()

local categories = {
	{ dir = "tests/globals", label = "GLOBALS" },
	{ dir = "tests/api", label = "API / STD" },
	{ dir = "tests/ffi", label = "FFI" },
	{ dir = "tests/stress", label = "STRESS" },
}

for _, cat in ipairs(categories) do
	if fs.isDir(cat.dir) then
		print(style("bold") .. "\n📦 Category: " .. cat.label .. style("reset"))
		scanAndRun(cat.dir, args[1])
	end
end

-- Summary
print(string.rep("-", 50))
print(style("bold") .. "Summary:" .. style("reset"))
print(string.format("Total:   %d", stats.total))
print(string.format("Passed:  %s%d%s", style("green"), stats.passed, style("reset")))
print(
	string.format(
		"Failed:  %s%d%s",
		stats.failed > 0 and style("red") or style("green"),
		stats.failed,
		style("reset")
	)
)

if stats.skipped > 0 then
	print(string.format("Skipped: %s", stats.skipped))
end

if stats.failed > 0 then
	print(style("bold") .. "\n🔴 Failure Details:" .. style("reset"))
	for _, fail in ipairs(failures) do
		print(string.rep("=", 40))
		print(style("red") .. "FILE: " .. fail.path .. style("reset"))
		print(style("yellow") .. "EXIT CODE: " .. fail.code .. style("reset"))
		if fail.stdout and #fail.stdout > 0 then
			print(style("cyan") .. "--- STDOUT ---" .. style("reset"))
			print(fail.stdout)
		end
		if fail.stderr and #fail.stderr > 0 then
			print(style("red") .. "--- STDERR ---" .. style("reset"))
			print(fail.stderr)
		end
	end
	process.exit(1)
else
	print(style("green") .. "\n✨ All tests passed!" .. style("reset"))
	process.exit(0)
end
