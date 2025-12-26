--!nocheck
--[=[
    @class Color3
    Represents a color in RGB color space with values from 0 to 1.
    
    ## Creating Colors
    ```lua
    -- Direct RGB (0-1 range)
    local red = Color3.new(1, 0, 0)
    
    -- From 0-255 RGB values (most common)
    local blue = Color3.fromRGB(0, 0, 255)
    
    -- From HSV (Hue, Saturation, Value)
    local orange = Color3.fromHSV(0.08, 1, 1)
    
    -- From hex string
    local purple = Color3.fromHex("#8B00FF")
    local gold = Color3.fromHex("FFD700")  -- # is optional
    ```
    
    ## Properties
    ```lua
    local c = Color3.fromRGB(255, 128, 64)
    print(c.R)  -- 1.0 (red component, 0-1)
    print(c.G)  -- ~0.5 (green component)
    print(c.B)  -- ~0.25 (blue component)
    ```
    
    ## Methods
    ```lua
    local white = Color3.new(1, 1, 1)
    local black = Color3.new(0, 0, 0)
    
    -- Lerp for smooth color transitions
    local gray = black:Lerp(white, 0.5)
    
    -- Convert to HSV
    local h, s, v = red:ToHSV()  -- h=0, s=1, v=1
    
    -- Convert to hex string
    local hex = red:ToHex()  -- "FF0000"
    ```
    
    ## Common Colors
    ```lua
    local WHITE = Color3.new(1, 1, 1)
    local BLACK = Color3.new(0, 0, 0)
    local RED = Color3.new(1, 0, 0)
    local GREEN = Color3.new(0, 1, 0)
    local BLUE = Color3.new(0, 0, 1)
    ```
]=]
export type Color3 = {
	--- Red component (0-1)
	R: number,
	--- Green component (0-1)
	G: number,
	--- Blue component (0-1)
	B: number,

	--- Linearly interpolates between self and goal color
	--- @param goal Color3 -- The target color
	--- @param alpha number -- Interpolation factor (0-1)
	--- @return Color3 -- The blended color
	Lerp: (self: Color3, goal: Color3, alpha: number) -> Color3,

	--- Converts to HSV color space
	--- @return number, number, number -- Hue (0-1), Saturation (0-1), Value (0-1)
	ToHSV: (self: Color3) -> (number, number, number),

	--- Converts to hexadecimal string (without #)
	--- @return string -- e.g. "FF0000" for red
	ToHex: (self: Color3) -> string,
}

--[=[
    @interface Color3Constructor
    Factory for creating Color3 instances.
]=]
local Color3: {
	--- Creates a Color3 from RGB values in 0-1 range
	--- @param r number -- Red (0-1)
	--- @param g number -- Green (0-1)
	--- @param b number -- Blue (0-1)
	new: (r: number, g: number, b: number) -> Color3,

	--- Creates a Color3 from RGB values in 0-255 range
	--- @param r number -- Red (0-255)
	--- @param g number -- Green (0-255)
	--- @param b number -- Blue (0-255)
	fromRGB: (r: number, g: number, b: number) -> Color3,

	--- Creates a Color3 from HSV values
	--- @param h number -- Hue (0-1, where 0=red, 0.33=green, 0.66=blue)
	--- @param s number -- Saturation (0-1)
	--- @param v number -- Value/Brightness (0-1)
	fromHSV: (h: number, s: number, v: number) -> Color3,

	--- Creates a Color3 from a hex string
	--- @param hex string -- Hex color like "#FF0000" or "FF0000" or "F00"
	fromHex: (hex: string) -> Color3,
} =
	{} :: any

return Color3
