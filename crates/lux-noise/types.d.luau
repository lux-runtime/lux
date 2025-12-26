--!nocheck
--[=[
    @class noise
    Procedural noise generation for terrain, textures, and procedural content.
    
    All noise functions return values in the range [-1, 1].
    
    ## Perlin Noise
    Classic gradient noise, smooth and natural-looking:
    ```lua
    local noise = require("@lux/noise")
    
    -- 2D noise for terrain heightmaps
    local height = noise.perlin2(x * 0.1, y * 0.1)
    
    -- 3D noise for volumetric effects
    local density = noise.perlin3(x, y, z)
    ```
    
    ## Simplex Noise
    Faster than Perlin with fewer directional artifacts:
    ```lua
    local value = noise.simplex2(x, y)
    local value3d = noise.simplex3(x, y, z)
    ```
    
    ## Fractal Brownian Motion (FBM)
    Layered noise for natural-looking detail:
    ```lua
    -- Basic FBM with 4 octaves
    local terrain = noise.fbm2(x, y, 4)
    
    -- Advanced FBM with custom parameters
    local detailed = noise.fbm3(x, y, z, 
        6,    -- octaves (more = more detail)
        2.0,  -- lacunarity (frequency multiplier between octaves)
        0.5   -- gain (amplitude multiplier between octaves)
    )
    ```
    
    ## Seeding
    Set the seed for reproducible noise:
    ```lua
    noise.setSeed(12345)  -- Same seed = same noise pattern
    ```
    
    ## Terrain Generation Example
    ```lua
    local function generateTerrain(width, height)
        noise.setSeed(os.time())
        local heightmap = {}
        
        for y = 1, height do
            heightmap[y] = {}
            for x = 1, width do
                -- Use FBM for natural-looking terrain
                local nx = x / width
                local ny = y / height
                heightmap[y][x] = noise.fbm2(nx * 4, ny * 4, 6)
            end
        end
        
        return heightmap
    end
    ```
]=]
export type noise = {
	--- 2D Perlin noise
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @return number -- Noise value in [-1, 1]
	perlin2: (x: number, y: number) -> number,

	--- 3D Perlin noise
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @param z number -- Z coordinate
	--- @return number -- Noise value in [-1, 1]
	perlin3: (x: number, y: number, z: number) -> number,

	--- 2D Simplex noise (faster than Perlin)
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @return number -- Noise value in [-1, 1]
	simplex2: (x: number, y: number) -> number,

	--- 3D Simplex noise
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @param z number -- Z coordinate
	--- @return number -- Noise value in [-1, 1]
	simplex3: (x: number, y: number, z: number) -> number,

	--- 2D Fractal Brownian Motion (layered noise)
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @param octaves number? -- Number of layers (default: 6)
	--- @param lacunarity number? -- Frequency multiplier (default: 2.0)
	--- @param gain number? -- Amplitude multiplier / persistence (default: 0.5)
	--- @return number -- Combined noise value
	fbm2: (x: number, y: number, octaves: number?, lacunarity: number?, gain: number?) -> number,

	--- 3D Fractal Brownian Motion
	--- @param x number -- X coordinate
	--- @param y number -- Y coordinate
	--- @param z number -- Z coordinate
	--- @param octaves number? -- Number of layers (default: 6)
	--- @param lacunarity number? -- Frequency multiplier (default: 2.0)
	--- @param gain number? -- Amplitude multiplier (default: 0.5)
	--- @return number -- Combined noise value
	fbm3: (
		x: number,
		y: number,
		z: number,
		octaves: number?,
		lacunarity: number?,
		gain: number?
	) -> number,

	--- Sets the global noise seed for reproducibility
	--- @param seed number -- The seed value
	setSeed: (seed: number) -> (),
}
return {} :: noise
