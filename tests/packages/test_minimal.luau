--!strict
-- Minimal test for WinLuxIO v4.0

local WinLuxIO = require("@lux_packages/winluxio")

print("Loading WinLuxIO...")
local Theme = WinLuxIO.Theme
local Window = WinLuxIO.Window

print("Theme loaded, isDark:", Theme.isDark)
print("Creating window...")

local win = Window.new({
	Title = "Test Window",
	Width = 400,
	Height = 300,
	Centered = true,
})

print("Window created!")
print("Showing window...")

win:show(false)
print("Window shown, running loop...")
WinLuxIO.run()
