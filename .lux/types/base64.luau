--!nocheck
--[=[
    @class base64
    Fast Base64 encoding and decoding.
    
    Base64 is a binary-to-text encoding scheme that represents binary data
    using 64 printable ASCII characters. Common uses include:
    - Embedding binary data in JSON/XML
    - Data URLs for images
    - Email attachments (MIME)
    - API authentication tokens
    
    ## Standard Base64
    ```lua
    local base64 = require("@lux/base64")
    
    -- Encode a string
    local encoded = base64.encode("Hello World")
    print(encoded)  -- "SGVsbG8gV29ybGQ="
    
    -- Decode back to buffer
    local decoded = base64.decode(encoded)
    print(buffer.tostring(decoded))  -- "Hello World"
    ```
    
    ## URL-Safe Base64
    Standard Base64 uses `+` and `/` which are not URL-safe.
    URL-safe Base64 uses `-` and `_` instead:
    ```lua
    -- URL-safe encoding (for URLs, filenames)
    local urlSafe = base64.encodeUrl("path/to/file?query=1")
    -- No + or / characters, safe for URLs
    
    local original = base64.decodeUrl(urlSafe)
    ```
    
    ## Binary Data
    Works with buffers for binary data:
    ```lua
    -- Create binary data
    local bin = buffer.create(4)
    buffer.writeu32(bin, 0, 0xDEADBEEF)
    
    -- Encode and decode
    local encoded = base64.encode(bin)
    local decoded = base64.decode(encoded)
    print(buffer.readu32(decoded, 0))  -- 0xDEADBEEF
    ```
    
    ## Data URLs
    ```lua
    local imageData = fs.readFile("image.png")
    local dataUrl = "data:image/png;base64," .. base64.encode(imageData)
    ```
]=]
export type base64 = {
	--- Encodes data to standard Base64
	--- @param data string | buffer -- The data to encode
	--- @return string -- Base64 encoded string
	encode: (data: string | buffer) -> string,

	--- Decodes standard Base64 to a buffer
	--- @param encoded string -- Base64 encoded string
	--- @return buffer -- Decoded binary data
	decode: (encoded: string) -> buffer,

	--- Encodes data to URL-safe Base64 (uses - and _ instead of + and /)
	--- @param data string | buffer -- The data to encode
	--- @return string -- URL-safe Base64 encoded string
	encodeUrl: (data: string | buffer) -> string,

	--- Decodes URL-safe Base64 to a buffer
	--- @param encoded string -- URL-safe Base64 encoded string
	--- @return buffer -- Decoded binary data
	decodeUrl: (encoded: string) -> buffer,
}
return {} :: base64
