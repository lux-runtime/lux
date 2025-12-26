--!nocheck
--[=[
    @class Enum
    Container for all enumeration types. Used for type-safe constants.
    
    ## KeyCode
    Platform-specific key codes for FFI input handling:
    - **Windows**: VK_* codes (for GetAsyncKeyState, etc.)
    - **Linux**: evdev KEY_* codes
    - **macOS**: Carbon kVK_* codes
    
    ```lua
    local ffi = require("@lux/ffi")
    ffi.cdef("int16_t GetAsyncKeyState(int vKey);")
    
    -- Check if 'A' key is pressed
    if ffi.C.GetAsyncKeyState(Enum.KeyCode.A) ~= 0 then
        print("A is pressed!")
    end
    ```
    
    ## MouseButton
    Mouse button identifiers for input handling:
    ```lua
    if mouseButton == Enum.MouseButton.Left then
        -- Handle left click
    end
    ```
    
    ## EasingStyle
    Easing functions for smooth animations:
    ```lua
    local style = Enum.EasingStyle.Quad
    -- Linear, Quad, Cubic, Quart, Quint, Sine, Expo, Circ, Elastic, Back, Bounce
    ```
    
    ## EasingDirection
    Direction for easing functions:
    ```lua
    local direction = Enum.EasingDirection.InOut
    -- In, Out, InOut
    ```
]=]

--[=[
    @interface KeyCode
    Keyboard key codes. Values are platform-specific for direct FFI usage.
    
    ## Letters
    `A-Z` - Standard letter keys
    
    ## Numbers
    `Zero` to `Nine` - Number row keys
    
    ## Function Keys
    `F1` to `F12`
    
    ## Modifiers
    `LeftShift`, `RightShift`, `LeftControl`, `RightControl`, `LeftAlt`, `RightAlt`, `LeftSuper`, `RightSuper`
    
    ## Navigation
    `Up`, `Down`, `Left`, `Right`, `Home`, `End`, `PageUp`, `PageDown`
    
    ## Special
    `Space`, `Return`, `Escape`, `Tab`, `Backspace`, `Delete`, `Insert`
    
    ## Numpad
    `Numpad0` to `Numpad9`, `NumLock`
    
    ## Punctuation
    `Semicolon`, `Comma`, `Period`, `Slash`, `Backslash`, `LeftBracket`, `RightBracket`, `Apostrophe`, `Grave`, `Minus`, `Equals`
]=]
export type KeyCode = {
	-- Letters
	A: number,
	B: number,
	C: number,
	D: number,
	E: number,
	F: number,
	G: number,
	H: number,
	I: number,
	J: number,
	K: number,
	L: number,
	M: number,
	N: number,
	O: number,
	P: number,
	Q: number,
	R: number,
	S: number,
	T: number,
	U: number,
	V: number,
	W: number,
	X: number,
	Y: number,
	Z: number,

	-- Numbers
	Zero: number,
	One: number,
	Two: number,
	Three: number,
	Four: number,
	Five: number,
	Six: number,
	Seven: number,
	Eight: number,
	Nine: number,

	-- Function keys
	F1: number,
	F2: number,
	F3: number,
	F4: number,
	F5: number,
	F6: number,
	F7: number,
	F8: number,
	F9: number,
	F10: number,
	F11: number,
	F12: number,

	-- Modifiers
	LeftShift: number,
	RightShift: number,
	LeftControl: number,
	RightControl: number,
	LeftAlt: number,
	RightAlt: number,
	LeftSuper: number,
	RightSuper: number,
	CapsLock: number,
	NumLock: number,

	-- Navigation
	Up: number,
	Down: number,
	Left: number,
	Right: number,
	Home: number,
	End: number,
	PageUp: number,
	PageDown: number,

	-- Special keys
	Space: number,
	Return: number,
	Escape: number,
	Tab: number,
	Backspace: number,
	Delete: number,
	Insert: number,
	Menu: number,
	PrintScreen: number,
	ScrollLock: number,
	Pause: number,

	-- Numpad
	Numpad0: number,
	Numpad1: number,
	Numpad2: number,
	Numpad3: number,
	Numpad4: number,
	Numpad5: number,
	Numpad6: number,
	Numpad7: number,
	Numpad8: number,
	Numpad9: number,

	-- Punctuation
	Semicolon: number,
	Equals: number,
	Comma: number,
	Minus: number,
	Period: number,
	Slash: number,
	Grave: number,
	LeftBracket: number,
	Backslash: number,
	RightBracket: number,
	Apostrophe: number,
}

--[=[
    @interface MouseButton
    Mouse button identifiers.
]=]
export type MouseButton = {
	--- Primary mouse button (usually left)
	Left: number,
	--- Secondary mouse button (usually right)
	Right: number,
	--- Middle mouse button (wheel click)
	Middle: number,
	--- Extra mouse button 4
	Button4: number,
	--- Extra mouse button 5
	Button5: number,
}

--[=[
    @interface EasingStyle
    Easing function types for animations.
]=]
export type EasingStyle = {
	--- Constant speed, no acceleration
	Linear: number,
	--- Quadratic (t²) easing
	Quad: number,
	--- Cubic (t³) easing
	Cubic: number,
	--- Quartic (t⁴) easing
	Quart: number,
	--- Quintic (t⁵) easing
	Quint: number,
	--- Sinusoidal easing
	Sine: number,
	--- Exponential easing
	Expo: number,
	--- Circular easing
	Circ: number,
	--- Elastic/spring easing (overshoots)
	Elastic: number,
	--- Back easing (overshoots slightly)
	Back: number,
	--- Bouncing effect at the end
	Bounce: number,
}

--[=[
    @interface EasingDirection
    Direction of the easing function.
]=]
export type EasingDirection = {
	--- Ease in (slow start, fast end)
	In: number,
	--- Ease out (fast start, slow end)
	Out: number,
	--- Ease in and out (slow start and end)
	InOut: number,
}

--[=[
    @interface SortOrder
    Order for sorting elements.
]=]
export type SortOrder = {
	--- Sort by LayoutOrder property
	LayoutOrder: number,
	--- Sort alphabetically by Name
	Name: number,
}

export type Enum = {
	--- Keyboard key codes (platform-specific)
	KeyCode: KeyCode,
	--- Mouse button identifiers
	MouseButton: MouseButton,
	--- Animation easing styles
	EasingStyle: EasingStyle,
	--- Animation easing directions
	EasingDirection: EasingDirection,
	--- Element sorting order
	SortOrder: SortOrder,
}

return {} :: Enum
