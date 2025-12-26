--!nocheck
--[=[
    @class UDim
    A 1-dimensional UI measurement with scale and offset components.
    
    The final size is calculated as: `parentSize * Scale + Offset`
    
    ## Example
    ```lua
    -- 50% of parent plus 10 pixels
    local size = UDim.new(0.5, 10)
    print(size.Scale)   -- 0.5
    print(size.Offset)  -- 10
    ```
]=]
export type UDim = {
	--- The scale component (0-1 typically, where 1 = 100% of parent)
	Scale: number,
	--- The offset component in pixels
	Offset: number,
}

--[=[
    @class UDim2
    A 2-dimensional UI measurement for position and size.
    
    ## Creating UDim2
    ```lua
    -- Full constructor: (xScale, xOffset, yScale, yOffset)
    local size = UDim2.new(0.5, 0, 0.5, 0)  -- 50% width and height
    
    -- Scale only (no offset)
    local fullscreen = UDim2.fromScale(1, 1)
    
    -- Offset only (fixed pixels)
    local button = UDim2.fromOffset(200, 50)
    ```
    
    ## Properties
    ```lua
    local u = UDim2.new(0.5, 10, 0.25, 20)
    print(u.X.Scale)   -- 0.5
    print(u.X.Offset)  -- 10
    print(u.Y.Scale)   -- 0.25
    print(u.Y.Offset)  -- 20
    ```
]=]
export type UDim2 = {
	--- The X dimension (horizontal)
	X: UDim,
	--- The Y dimension (vertical)
	Y: UDim,
}

--[=[
    @class Rect
    A rectangle defined by minimum and maximum corners.
    
    ## Example
    ```lua
    local rect = Rect.new(10, 20, 110, 220)
    print(rect.Width)   -- 100
    print(rect.Height)  -- 200
    ```
    
    Note: Coordinates are automatically normalized (min < max).
]=]
export type Rect = {
	--- The width of the rectangle (Max.X - Min.X)
	Width: number,
	--- The height of the rectangle (Max.Y - Min.Y)
	Height: number,
}

--[=[
    @class NumberRange
    A range between two numbers, useful for random values or constraints.
    
    ## Example
    ```lua
    local range = NumberRange.new(0, 100)
    print(range.Min)  -- 0
    print(range.Max)  -- 100
    
    -- Coordinates are normalized
    local inverted = NumberRange.new(100, 0)
    print(inverted.Min)  -- 0 (automatically corrected)
    ```
]=]
export type NumberRange = {
	--- The minimum value of the range
	Min: number,
	--- The maximum value of the range
	Max: number,
}

local UDim: {
	--- Creates a new UDim
	--- @param scale number -- The scale component (0-1)
	--- @param offset number -- The offset in pixels
	new: (scale: number, offset: number) -> UDim,
} =
	{} :: any

local UDim2: {
	--- Creates a new UDim2 with all components
	--- @param xScale number -- Horizontal scale
	--- @param xOffset number -- Horizontal offset
	--- @param yScale number -- Vertical scale
	--- @param yOffset number -- Vertical offset
	new: (xScale: number, xOffset: number, yScale: number, yOffset: number) -> UDim2,

	--- Creates a UDim2 with only scale (offset = 0)
	--- @param xScale number -- Horizontal scale
	--- @param yScale number -- Vertical scale
	fromScale: (xScale: number, yScale: number) -> UDim2,

	--- Creates a UDim2 with only offset (scale = 0)
	--- @param xOffset number -- Horizontal offset in pixels
	--- @param yOffset number -- Vertical offset in pixels
	fromOffset: (xOffset: number, yOffset: number) -> UDim2,
} =
	{} :: any

local Rect: {
	--- Creates a new Rect from corner coordinates
	--- @param minX number -- Left edge
	--- @param minY number -- Top edge
	--- @param maxX number -- Right edge
	--- @param maxY number -- Bottom edge
	new: (minX: number, minY: number, maxX: number, maxY: number) -> Rect,
} =
	{} :: any

local NumberRange: {
	--- Creates a new NumberRange
	--- @param min number -- The minimum value
	--- @param max number -- The maximum value
	new: (min: number, max: number) -> NumberRange,
} =
	{} :: any

return { UDim = UDim, UDim2 = UDim2, Rect = Rect, NumberRange = NumberRange }
