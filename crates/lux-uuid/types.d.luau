--!nocheck
--[=[
    @class uuid
    UUID (Universally Unique Identifier) generation and manipulation.
    
    UUIDs are 128-bit identifiers that are globally unique. This library supports:
    - **UUID v4**: Random-based (most common)
    - **UUID v7**: Timestamp-based, sortable by creation time
    
    ## Generating UUIDs
    ```lua
    local uuid = require("@lux/uuid")
    
    -- Random UUID (v4) - most common
    local id = uuid.v4()  -- "550e8400-e29b-41d4-a716-446655440000"
    
    -- Timestamp-based UUID (v7) - sortable by time
    local sortableId = uuid.v7()
    ```
    
    ## Validation
    ```lua
    local valid = uuid.isValid("550e8400-e29b-41d4-a716-446655440000")  -- true
    local invalid = uuid.isValid("not-a-uuid")  -- false
    ```
    
    ## Binary Conversion
    For efficient storage or network transmission:
    ```lua
    -- Convert to 16-byte buffer
    local buf = uuid.parse(id)  -- buffer (16 bytes)
    
    -- Convert back to string
    local str = uuid.format(buf)  -- "550e8400-..."
    ```
    
    ## Nil UUID
    The nil UUID is all zeros, useful as a placeholder:
    ```lua
    print(uuid["nil"])  -- "00000000-0000-0000-0000-000000000000"
    ```
    
    ## Use Cases
    - **Database primary keys**: Use v4 for random distribution
    - **Time-ordered records**: Use v7 for sortable IDs
    - **Session tokens**: Use v4 for unpredictable identifiers
]=]
export type uuid = {
	--- Generates a random UUID (version 4)
	--- @return string -- A new random UUID like "550e8400-e29b-41d4-a716-446655440000"
	v4: () -> string,

	--- Generates a timestamp-based UUID (version 7)
	--- UUIDs generated later sort after earlier ones
	--- @return string -- A new timestamp-based UUID
	v7: () -> string,

	--- Validates if a string is a valid UUID format
	--- @param s string -- The string to validate
	--- @return boolean -- true if valid UUID format
	isValid: (s: string) -> boolean,

	--- Parses a UUID string into a 16-byte buffer
	--- @param s string -- A valid UUID string
	--- @return buffer? -- 16-byte buffer, or nil if invalid
	parse: (s: string) -> buffer?,

	--- Formats a 16-byte buffer as a UUID string
	--- @param buf buffer -- A 16-byte buffer
	--- @return string -- The formatted UUID string
	format: (buf: buffer) -> string,

	--- The nil UUID constant (all zeros)
	["nil"]: string,
}
return {} :: uuid
