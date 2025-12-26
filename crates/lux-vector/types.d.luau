--!nocheck
--[=[
    @class Vector2
    A 2D vector with X and Y components. Optimized for mathematical operations.
    
    ## Creating Vectors
    ```lua
    local v1 = Vector2.new(3, 4)
    local origin = Vector2.zero  -- (0, 0)
    local unit = Vector2.one     -- (1, 1)
    ```
    
    ## Properties
    - `X`, `Y`: The components of the vector
    - `Magnitude`: The length of the vector (read-only)
    - `Unit`: A normalized vector with the same direction (read-only)
    
    ## Methods
    ```lua
    local v1 = Vector2.new(3, 4)
    local v2 = Vector2.new(1, 0)
    
    -- Interpolation
    local mid = v1:Lerp(v2, 0.5)  -- Halfway between v1 and v2
    
    -- Dot product (returns scalar)
    local dot = v1:Dot(v2)  -- 3*1 + 4*0 = 3
    
    -- Cross product (2D returns scalar: z-component of 3D cross)
    local cross = v1:Cross(v2)  -- 3*0 - 4*1 = -4
    ```
    
    ## Operators
    ```lua
    local sum = v1 + v2      -- Vector addition
    local diff = v1 - v2     -- Vector subtraction
    local scaled = v1 * 2    -- Scalar multiplication
    local divided = v1 / 2   -- Scalar division
    local neg = -v1          -- Negation
    local eq = v1 == v2      -- Equality check
    print(tostring(v1))      -- "3, 4"
    ```
]=]
export type Vector2 = {
	--- The X component of the vector
	X: number,
	--- The Y component of the vector
	Y: number,
	--- The length (magnitude) of the vector: sqrt(X² + Y²)
	Magnitude: number,
	--- A normalized vector with magnitude 1, pointing in the same direction
	Unit: Vector2,

	--- Linearly interpolates between self and goal by alpha (0-1)
	--- @param goal Vector2 -- The target vector to interpolate towards
	--- @param alpha number -- Interpolation factor: 0 = self, 1 = goal
	--- @return Vector2 -- The interpolated vector
	Lerp: (self: Vector2, goal: Vector2, alpha: number) -> Vector2,

	--- Calculates the dot product with another vector
	--- @param other Vector2 -- The other vector
	--- @return number -- The dot product (X₁*X₂ + Y₁*Y₂)
	Dot: (self: Vector2, other: Vector2) -> number,

	--- Calculates the 2D cross product (z-component of 3D cross)
	--- @param other Vector2 -- The other vector
	--- @return number -- The cross product (X₁*Y₂ - Y₁*X₂)
	Cross: (self: Vector2, other: Vector2) -> number,
}

--[=[
    @class Vector3
    A 3D vector with X, Y, and Z components. Essential for 3D math operations.
    
    ## Creating Vectors
    ```lua
    local v1 = Vector3.new(1, 2, 3)
    local origin = Vector3.zero  -- (0, 0, 0)
    local unit = Vector3.one     -- (1, 1, 1)
    ```
    
    ## Cross Product
    Returns a vector perpendicular to both input vectors:
    ```lua
    local right = Vector3.new(1, 0, 0)
    local up = Vector3.new(0, 1, 0)
    local forward = right:Cross(up)  -- (0, 0, 1)
    ```
]=]
export type Vector3 = {
	--- The X component of the vector
	X: number,
	--- The Y component of the vector
	Y: number,
	--- The Z component of the vector
	Z: number,
	--- The length (magnitude) of the vector: sqrt(X² + Y² + Z²)
	Magnitude: number,
	--- A normalized vector with magnitude 1
	Unit: Vector3,

	--- Linearly interpolates between self and goal by alpha (0-1)
	Lerp: (self: Vector3, goal: Vector3, alpha: number) -> Vector3,

	--- Calculates the dot product with another vector
	Dot: (self: Vector3, other: Vector3) -> number,

	--- Calculates the cross product, returning a perpendicular vector
	Cross: (self: Vector3, other: Vector3) -> Vector3,
}

--[=[
    @interface Vector2Constructor
    Factory for creating Vector2 instances.
]=]
local Vector2: {
	--- Creates a new Vector2 with the given coordinates
	--- @param x number -- The X component
	--- @param y number -- The Y component
	--- @return Vector2
	new: (x: number, y: number) -> Vector2,

	--- Constant vector (0, 0)
	zero: Vector2,

	--- Constant vector (1, 1)
	one: Vector2,
} =
	{} :: any

--[=[
    @interface Vector3Constructor
    Factory for creating Vector3 instances.
]=]
local Vector3: {
	--- Creates a new Vector3 with the given coordinates
	new: (x: number, y: number, z: number) -> Vector3,

	--- Constant vector (0, 0, 0)
	zero: Vector3,

	--- Constant vector (1, 1, 1)
	one: Vector3,
} =
	{} :: any

return { Vector2 = Vector2, Vector3 = Vector3 }
