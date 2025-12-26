-- tests/api/test_fs.luau
-- Tests for @lux/fs

local fs = require("@lux/fs")
local process = require("@lux/process")

print("Testing @lux/fs...")

-- Prepare a temp directory
local TMP_DIR = "tests/tmp_fs"
if fs.isDir(TMP_DIR) then
	fs.removeDir(TMP_DIR)
end
fs.writeDir(TMP_DIR)

-- 1. Write File
local filePath = TMP_DIR .. "/hello.txt"
local content = "Hello Lux!"
fs.writeFile(filePath, content)
assert(fs.isFile(filePath), "fs.isFile should return true for created file")

-- 2. Read File (text)
local readBack = fs.readFile(filePath)
assert(readBack == content, "fs.readFile should return correct content")

-- 3. Read Dir
local entries = fs.readDir(TMP_DIR)
local found = false
for _, name in ipairs(entries) do
	if name == "hello.txt" then
		found = true
	end
end
assert(found, "fs.readDir should list created file")

-- 4. Metadata
local meta = fs.metadata(filePath)
assert(type(meta) == "table", "fs.metadata should return a table")
assert(meta.kind == "file", "meta.kind should be file")
assert(meta.exists == true, "meta.exists should be true")

-- 5. Copy & Move
local copyPath = TMP_DIR .. "/copy.txt"
fs.copy(filePath, copyPath)
assert(fs.isFile(copyPath), "fs.copy should create file copy")
assert(fs.readFile(copyPath) == content, "copied file content should match")

local movePath = TMP_DIR .. "/moved.txt"
fs.move(copyPath, movePath)
assert(fs.isFile(movePath), "fs.move should move file")
assert(not fs.isFile(copyPath), "original file should be gone after move")

-- 6. Read/Write Binary (buffer-like string)
local binPath = TMP_DIR .. "/data.bin"
local binData = "\0\1\2\3\255"
fs.writeFile(binPath, binData)
local binRead = fs.readFile(binPath)
assert(binRead == binData, "binary read/write should preserve bytes")

-- Cleanup
fs.removeDir(TMP_DIR)
assert(not fs.isDir(TMP_DIR), "fs.removeDir should remove directory")

print("FS Tests Passed!")
