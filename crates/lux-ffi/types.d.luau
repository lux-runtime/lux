--!nocheck

--[=[
    @class FFI

    Foreign Function Interface for calling C libraries from Luau.

    The FFI library allows you to:
    - Define C types (structs, unions, enums, typedefs, functions)
    - Load dynamic libraries (.dll, .so, .dylib)
    - Call C functions directly from Luau
    - Create callbacks that C code can call back into Luau
    - Allocate and manipulate C memory

    ### Example usage

    ```lua
    local ffi = require("@lux/ffi")

    -- Define C types and functions
    ffi.cdef[[
        typedef struct Point {
            int x;
            int y;
        } Point;
        
        int MessageBoxA(void* hwnd, const char* text, const char* caption, int type);
    ]]

    -- Create and use a struct
    local point = ffi.new("Point")
    point.x = 100
    point.y = 200
    print(point.x, point.y) -- 100, 200
    print(ffi.sizeof("Point")) -- 8

    -- Load a DLL and call functions
    local user32 = ffi.load("user32")
    user32.MessageBoxA(nil, "Hello from FFI!", "Lux", 0)
    
    -- Create callbacks for Windows APIs
    local callback = ffi.callback("int(*)(int, int)", function(a, b)
        return a + b
    end)
    ```
]=]

-- ============================================================================
-- Types
-- ============================================================================

--[=[
    @interface CData
    @within FFI

    A C data object representing allocated C memory.

    Properties:
    * `size` - The size of the allocated memory in bytes

    Methods:
    * `ptr()` - Returns a raw pointer to the data

    Indexing:
    * `[index]` - Array access (0-indexed for C arrays)
    * `[fieldName]` - Struct field access
    
    ### Example
    ```lua
    -- Arrays
    local arr = ffi.new("int[10]")
    arr[0] = 42
    arr[1] = 100
    print(arr[0]) -- 42
    
    -- Structs  
    local point = ffi.new("Point")
    point.x = 10
    point.y = 20
    print(point.size) -- Size in bytes
    ```
]=]
export type CData = {
	size: number,
	ptr: (self: CData) -> CData,
	[any]: any,
}

--[=[
    @interface CType
    @within FFI

    A C type descriptor returned by `ffi.typeof()`.

    Properties:
    * `name` - The type name
    * `size` - Size in bytes
    * `align` - Alignment in bytes
    
    ### Example
    ```lua
    local PointType = ffi.typeof("Point")
    print(PointType.size)  -- 8
    print(PointType.align) -- 4
    
    -- Use CType with ffi.new
    local p1 = ffi.new(PointType)
    local p2 = ffi.new(PointType)
    ```
]=]
export type CType = {
	name: string,
	size: number,
	align: number,
}

--[=[
    @interface SmartLibrary
    @within FFI

    A loaded dynamic library that can call functions defined via cdef.

    Properties:
    * `path` - The path to the loaded library

    Indexing:
    * `[functionName]` - Returns a callable function from the library

    ### Example
    ```lua
    ffi.cdef[[
        int GetSystemMetrics(int index);
        BOOL SetCursorPos(int x, int y);
    ]]
    
    local user32 = ffi.load("user32")
    local screenWidth = user32.GetSystemMetrics(0)
    local screenHeight = user32.GetSystemMetrics(1)
    print("Screen:", screenWidth, "x", screenHeight)
    
    -- Move cursor to center
    user32.SetCursorPos(screenWidth / 2, screenHeight / 2)
    ```
]=]
export type SmartLibrary = {
	path: string,
	[string]: (...any) -> any,
}

--[=[
    @interface Callback
    @within FFI

    A C-callable callback created from a Lua function.

    Properties:
    * `signature` - The C function signature
    * `func` - The original Lua function

    ### Example
    ```lua
    -- Create a callback for Win32 EnumWindows
    ffi.cdef[[
        typedef int (*WNDENUMPROC)(void* hwnd, long lParam);
        int EnumWindows(WNDENUMPROC lpEnumFunc, long lParam);
    ]]
    
    local count = 0
    local enumProc = ffi.callback("int(*)(void*, long)", function(hwnd, lParam)
        count = count + 1
        return 1 -- Continue enumeration
    end)
    
    local user32 = ffi.load("user32")
    user32.EnumWindows(enumProc, 0)
    print("Found", count, "windows")
    ```
]=]
export type Callback = {
	signature: string,
	func: (...any) -> any,
}

-- ============================================================================
-- Module
-- ============================================================================

local ffi = {}

--[=[
    @within FFI
    @prop os string
    
    The current operating system. One of: `"windows"`, `"linux"`, `"macos"`.
    
    ### Example
    ```lua
    if ffi.os == "windows" then
        print("Running on Windows")
    elseif ffi.os == "linux" then
        print("Running on Linux")
    end
    ```
]=]
ffi.os = "windows" :: string

--[=[
    @within FFI
    @prop arch string
    
    The current CPU architecture. One of: `"x86_64"`, `"x86"`, `"aarch64"`.
    
    ### Example
    ```lua
    if ffi.arch == "x86_64" then
        print("64-bit architecture")
    elseif ffi.arch == "x86" then
        print("32-bit architecture")
    end
    ```
]=]
ffi.arch = "x86_64" :: string

--[=[
    @within FFI
    @prop C SmartLibrary?

    The system C library (msvcrt on Windows, libc on Linux/macOS).
    
    ### Example
    ```lua
    ffi.cdef[[
        int printf(const char* fmt, ...);
        double sin(double x);
        double cos(double x);
    ]]
    
    ffi.C.printf("Hello %s! Pi = %f\n", "World", math.pi)
    print(ffi.C.sin(math.pi / 2)) -- 1.0
    ```
]=]
ffi.C = nil :: SmartLibrary?

--[=[
    @within FFI

    Defines C types (structs, enums, unions, typedefs, functions).

    Supports:
    - Structs: `typedef struct { ... } Name;`
    - Enums: `typedef enum { ... } Name;`
    - Unions: `typedef union { ... } Name;`
    - Typedefs: `typedef int MyInt;`
    - Functions: `int func(int a, int b);`
    - Packed structs: `__attribute__((packed))`

    @param declarations -- The C declarations to parse
    
    ### Example
    ```lua
    ffi.cdef[[
        // Struct
        typedef struct Vector3 {
            float x, y, z;
        } Vector3;
        
        // Enum
        typedef enum Color {
            RED = 0,
            GREEN = 1,
            BLUE = 2
        } Color;
        
        // Function declarations
        void* malloc(size_t size);
        void free(void* ptr);
        int strcmp(const char* s1, const char* s2);
        
        // Packed struct (no padding)
        typedef struct __attribute__((packed)) PackedData {
            char a;
            int b;
        } PackedData;
    ]]
    ```
]=]
function ffi.cdef(declarations: string) end

--[=[
    @within FFI
    @tag must_use

    Loads a dynamic library.

    On Windows, automatically appends `.dll` if not present.
    On Linux, automatically prepends `lib` and appends `.so` if needed.

    @param path -- Path to the library (.dll, .so, .dylib)
    @return SmartLibrary -- The loaded library
    
    ### Example
    ```lua
    -- Windows
    local kernel32 = ffi.load("kernel32")
    local user32 = ffi.load("user32")
    
    -- Custom library
    local myLib = ffi.load("path/to/mylib.dll")
    local myLib2 = ffi.load("C:/libs/mylibrary") -- .dll auto-added
    
    -- Linux
    local pthread = ffi.load("pthread") -- loads libpthread.so
    ```
]=]
function ffi.load(path: string): SmartLibrary
	return { path = path } :: SmartLibrary
end

--[=[
    @within FFI
    @tag must_use

    Creates a new C data object.

    For variable-length arrays, use `[?]` syntax with size parameter.

    @param typeName -- The type name or CType to allocate
    @param size -- Optional size for variable-length arrays
    @return CData -- The allocated C data
    
    ### Example
    ```lua
    -- Fixed-size struct
    local point = ffi.new("Point")
    point.x = 10
    point.y = 20
    
    -- Fixed-size array
    local arr10 = ffi.new("int[10]")
    for i = 0, 9 do
        arr10[i] = i * i
    end
    
    -- Variable-length array (VLA)
    local size = 100
    local buffer = ffi.new("char[?]", size)
    
    -- Using CType
    local PointType = ffi.typeof("Point")
    local p1 = ffi.new(PointType)
    local p2 = ffi.new(PointType)
    ```
]=]
function ffi.new(typeName: string | CType, size: number?): CData
	return {} :: any
end

--[=[
    @within FFI
    @tag must_use

    Returns a CType object for the given type name.

    @param typeName -- The type name
    @return CType -- The type descriptor
    
    ### Example
    ```lua
    local PointType = ffi.typeof("Point")
    print("Size:", PointType.size)   -- 8
    print("Align:", PointType.align) -- 4
    print("Name:", PointType.name)   -- "Point"
    
    -- Useful for creating many instances
    local points = {}
    for i = 1, 1000 do
        points[i] = ffi.new(PointType)
    end
    ```
]=]
function ffi.typeof(typeName: string): CType
	return { name = typeName, size = 0, align = 0 }
end

--[=[
    @within FFI
    @tag must_use

    Converts a C string pointer to a Lua string.

    @param ptr -- Pointer to the C string
    @param len -- Optional length (if nil, reads until null terminator)
    @return string? -- The Lua string, or nil if pointer is null
    
    ### Example
    ```lua
    ffi.cdef[[
        const char* getenv(const char* name);
        char* strcpy(char* dest, const char* src);
    ]]
    
    -- Read environment variable
    local pathPtr = ffi.C.getenv("PATH")
    local path = ffi.string(pathPtr)
    print("PATH:", path)
    
    -- Read fixed-length buffer
    local buffer = ffi.new("char[100]")
    ffi.C.strcpy(buffer, "Hello, World!")
    print(ffi.string(buffer))        -- "Hello, World!"
    print(ffi.string(buffer, 5))     -- "Hello"
    ```
]=]
function ffi.string(ptr: CData, len: number?): string?
	return nil
end

--[=[
    @within FFI
    @tag must_use

    Gets the size of a type in bytes.

    @param typeName -- The type name or CType
    @return number -- The size in bytes
    
    ### Example
    ```lua
    print(ffi.sizeof("char"))     -- 1
    print(ffi.sizeof("short"))    -- 2
    print(ffi.sizeof("int"))      -- 4
    print(ffi.sizeof("long long")) -- 8
    print(ffi.sizeof("float"))    -- 4
    print(ffi.sizeof("double"))   -- 8
    print(ffi.sizeof("void*"))    -- 8 (on 64-bit)
    print(ffi.sizeof("Point"))    -- 8 (struct with two ints)
    
    -- With CType
    local t = ffi.typeof("Vector3")
    print(ffi.sizeof(t)) -- 12
    ```
]=]
function ffi.sizeof(typeName: string | CType): number
	return 0
end

--[=[
    @within FFI
    @tag must_use

    Gets the alignment of a type in bytes.

    @param typeName -- The type name or CType
    @return number -- The alignment in bytes
    
    ### Example
    ```lua
    print(ffi.alignof("char"))   -- 1
    print(ffi.alignof("short"))  -- 2
    print(ffi.alignof("int"))    -- 4
    print(ffi.alignof("double")) -- 8
    print(ffi.alignof("Point"))  -- 4 (max of members)
    ```
]=]
function ffi.alignof(typeName: string | CType): number
	return 0
end

--[=[
    @within FFI
    @tag must_use

    Gets the byte offset of a field within a struct.

    @param typeName -- The struct type name
    @param field -- The field name
    @return number -- The byte offset
    
    ### Example
    ```lua
    ffi.cdef[[
        typedef struct Person {
            char name[32];   // offset 0
            int age;         // offset 32
            double height;   // offset 40 (aligned to 8)
        } Person;
    ]]
    
    print(ffi.offsetof("Person", "name"))   -- 0
    print(ffi.offsetof("Person", "age"))    -- 32
    print(ffi.offsetof("Person", "height")) -- 40
    ```
]=]
function ffi.offsetof(typeName: string, field: string): number
	return 0
end

--[=[
    @within FFI
    @tag must_use

    Gets a pointer to a specific field within a struct.

    @param cdata -- The CData struct
    @param field -- The field name
    @return CData -- Pointer to the field
    
    ### Example
    ```lua
    ffi.cdef[[
        typedef struct TextSize { int cx; int cy; } TextSize;
    ]]
    
    local size = ffi.new("TextSize")
    local pWidth = ffi.addressof(size, "cx")
    local pHeight = ffi.addressof(size, "cy")
    
    -- Pass pointers to C functions that need out parameters
    gdi.GetTextExtentPoint32A(hdc, "Hello", 5, pWidth, pHeight)
    print("Text size:", size.cx, "x", size.cy)
    ```
]=]
function ffi.addressof(cdata: CData, field: string): CData
	return {} :: any
end

--[=[
    @within FFI
    @tag must_use

    Casts a value to a different type.

    @param typeName -- The target type
    @param value -- The value to cast
    @return CData -- The casted value
    
    ### Example
    ```lua
    -- Cast number to pointer
    local ptr = ffi.cast("void*", 0x12345678)
    
    -- Cast between pointer types
    local voidPtr = ffi.new("void*")
    local intPtr = ffi.cast("int*", voidPtr)
    
    -- Cast callback to function pointer
    local callback = ffi.callback("int(*)(int)", function(x) return x * 2 end)
    local funcPtr = ffi.cast("void*", callback)
    
    -- Cast number to int
    local n = ffi.cast("int", 3.7) -- Truncates to 3
    ```
]=]
function ffi.cast(typeName: string, value: any): CData
	return {} :: any
end

--[=[
    @within FFI
    @tag must_use

    Creates a C-callable callback from a Lua function.

    The callback can be passed to C functions that expect function pointers.

    @param signature -- The C function signature (e.g., `"int(*)(int, int)"`)
    @param func -- The Lua function to wrap
    @return Callback -- The callback object
    
    ### Example
    ```lua
    -- Simple callback
    local add = ffi.callback("int(*)(int, int)", function(a, b)
        return a + b
    end)
    
    -- Win32 window procedure
    ffi.cdef[[
        typedef long (*WNDPROC)(void*, unsigned int, unsigned long, long);
        long DefWindowProcA(void*, unsigned int, unsigned long, long);
    ]]
    
    local user32 = ffi.load("user32")
    
    local wndProc = ffi.callback("long(*)(void*, unsigned int, unsigned long, long)", 
        function(hwnd, msg, wParam, lParam)
            if msg == 0x0010 then -- WM_CLOSE
                print("Window closing!")
                return 0
            end
            return user32.DefWindowProcA(hwnd, msg, wParam, lParam)
        end
    )
    
    -- EnumWindows callback
    local windowCount = 0
    local enumCallback = ffi.callback("int(*)(void*, long)", function(hwnd, lParam)
        windowCount = windowCount + 1
        return 1 -- Continue
    end)
    user32.EnumWindows(enumCallback, 0)
    print("Total windows:", windowCount)
    ```
]=]
function ffi.callback(signature: string, func: (...any) -> any): Callback
	return { signature = signature, func = func }
end

--[=[
    @within FFI
    @tag must_use

    Batch processes an array of doubles using a C function.
    
    Zero-allocation overhead loop for high performance number crunching.
    
    @param func -- The cached C function (e.g. ffi.C.sin)
    @param input -- Input pointer (CData or memory address)
    @param output -- Output pointer (CData or memory address)
    @param count -- Number of elements to process
    @return number -- Number of elements processed
    
    ### Example
    ```lua
    local count = 1000
    local input = ffi.new("double[?]", count)
    local output = ffi.new("double[?]", count)
    
    -- Fill input
    for i = 0, count-1 do input[i] = i * 0.1 end
    
    -- Batch process 'sin'
    ffi.batch(ffi.C.sin, input, output, count)
    ```
]=]
function ffi.batch(func: any, input: CData | number, output: CData | number, count: number): number
	return 0
end

--[=[
    @within FFI
    @tag must_use

    Batch processes two arrays of doubles using a binary C function.
    
    @param func -- The cached C function (e.g. ffi.C.pow)
    @param input1 -- First input pointer
    @param input2 -- Second input pointer
    @param output -- Output pointer
    @param count -- Number of elements to process
    @return number -- Number of elements processed
]=]
function ffi.batch2(
	func: any,
	input1: CData | number,
	input2: CData | number,
	output: CData | number,
	count: number
): number
	return 0
end

--[=[
    @within FFI

    Associates a metatable with a C type for operator overloading.

    Supported metamethods: `__add`, `__sub`, `__mul`, `__div`, `__eq`, 
    `__lt`, `__le`, `__tostring`, `__index`, `__newindex`

    @param typeName -- The type name
    @param metatable -- The metatable with metamethods
    @return CType -- The type with metatable attached
    
    ### Example
    ```lua
    ffi.cdef[[
        typedef struct Vector2 { float x, y; } Vector2;
    ]]
    
    ffi.metatype("Vector2", {
        __add = function(a, b)
            local result = ffi.new("Vector2")
            result.x = a.x + b.x
            result.y = a.y + b.y
            return result
        end,
        __mul = function(a, scalar)
            local result = ffi.new("Vector2")
            result.x = a.x * scalar
            result.y = a.y * scalar
            return result
        end,
        __tostring = function(v)
            return string.format("Vector2(%.2f, %.2f)", v.x, v.y)
        end,
        __eq = function(a, b)
            return a.x == b.x and a.y == b.y
        end,
    })
    
    local v1 = ffi.new("Vector2"); v1.x, v1.y = 1, 2
    local v2 = ffi.new("Vector2"); v2.x, v2.y = 3, 4
    
    local v3 = v1 + v2      -- Uses __add
    local v4 = v1 * 2       -- Uses __mul
    print(v3)               -- Vector2(4.00, 6.00)
    print(v1 == v2)         -- false
    ```
]=]
function ffi.metatype(typeName: string, metatable: { [string]: any }): CType
	return { name = typeName, size = 0, align = 0 }
end

--[=[
    @within FFI

    Copies memory from source to destination.

    @param dst -- Destination pointer or CData
    @param src -- Source pointer, CData, or string
    @param len -- Number of bytes to copy
    
    ### Example
    ```lua
    -- Copy string to buffer
    local buffer = ffi.new("char[100]")
    ffi.copy(buffer, "Hello, World!", 13)
    print(ffi.string(buffer)) -- Hello, World!
    
    -- Copy between CData
    local src = ffi.new("int[5]")
    local dst = ffi.new("int[5]")
    for i = 0, 4 do src[i] = i * 10 end
    
    ffi.copy(dst, src, ffi.sizeof("int") * 5)
    print(dst[2]) -- 20
    ```
]=]
function ffi.copy(dst: CData, src: CData | string, len: number) end

--[=[
    @within FFI

    Fills memory with a byte value.

    @param dst -- Destination pointer or CData
    @param len -- Number of bytes to fill
    @param value -- Byte value to fill with (default: 0)
    
    ### Example
    ```lua
    local buffer = ffi.new("char[100]")
    
    -- Zero-fill (default)
    ffi.fill(buffer, 100)
    
    -- Fill with specific byte
    ffi.fill(buffer, 50, 0x41)  -- Fill first 50 bytes with 'A'
    print(ffi.string(buffer, 10)) -- AAAAAAAAAA
    
    -- Clear a struct
    local point = ffi.new("Point")
    ffi.fill(point, ffi.sizeof("Point"), 0)
    ```
]=]
function ffi.fill(dst: CData, len: number, value: number?) end

--[=[
    @within FFI
    @tag must_use

    Checks if a CData is of a specific type.

    @param typeName -- The type name to check
    @param cdata -- The CData to check
    @return boolean -- True if the CData is of the specified type
    
    ### Example
    ```lua
    local point = ffi.new("Point")
    local vector = ffi.new("Vector3")
    
    print(ffi.istype("Point", point))    -- true
    print(ffi.istype("Vector3", point))  -- false
    print(ffi.istype("Vector3", vector)) -- true
    
    -- Type checking in functions
    local function processPoint(data)
        if not ffi.istype("Point", data) then
            error("Expected Point, got something else")
        end
        return data.x + data.y
    end
    ```
]=]
function ffi.istype(typeName: string, cdata: CData): boolean
	return false
end

--[=[
    @within FFI

    Sets a finalizer for a CData object (called when garbage collected).

    Use this to ensure C resources are properly freed.

    @param cdata -- The CData object
    @param finalizer -- Function to call when the CData is collected
    @return CData -- The same CData object with finalizer attached
    
    ### Example
    ```lua
    ffi.cdef[[
        void* malloc(size_t size);
        void free(void* ptr);
        void* CreateFileA(const char* name, int access, int share, 
                          void* sa, int disp, int flags, void* tmpl);
        int CloseHandle(void* handle);
    ]]
    
    -- Auto-free memory
    local ptr = ffi.C.malloc(1024)
    ffi.gc(ptr, function(p)
        ffi.C.free(p)
        print("Memory freed!")
    end)
    -- ptr will be freed when garbage collected
    
    -- Auto-close file handle
    local kernel32 = ffi.load("kernel32")
    local handle = kernel32.CreateFileA("test.txt", 0x80000000, 1, nil, 3, 0, nil)
    ffi.gc(handle, function(h)
        kernel32.CloseHandle(h)
        print("File closed!")
    end)
    ```
]=]
function ffi.gc(cdata: CData, finalizer: (cdata: CData) -> ()): CData
	return cdata
end

return ffi
