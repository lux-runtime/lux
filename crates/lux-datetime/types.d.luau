--!nocheck
--[=[
    @class DateTime
    High-precision date and time handling with timezone support.
    
    DateTime represents an instant in time, stored internally as UTC.
    It can be converted to local time or any timezone for display.
    
    ## Creating DateTime
    ```lua
    -- Current time
    local now = DateTime.now()
    
    -- From Unix timestamp (seconds since 1970-01-01 UTC)
    local epoch = DateTime.fromUnixTimestamp(0)
    
    -- From Unix timestamp in milliseconds
    local precise = DateTime.fromUnixTimestampMillis(1703548800000)
    
    -- From components (UTC)
    local christmas = DateTime.fromUniversalTime(2024, 12, 25, 0, 0, 0, 0)
    
    -- From local time components
    local localTime = DateTime.fromLocalTime(2024, 12, 25, 9, 0, 0, 0)
    
    -- From ISO 8601 string
    local parsed = DateTime.fromIsoDate("2024-12-25T00:00:00Z")
    ```
    
    ## Properties
    ```lua
    local now = DateTime.now()
    print(now.UnixTimestamp)        -- Seconds since epoch
    print(now.UnixTimestampMillis)  -- Milliseconds since epoch
    ```
    
    ## Converting to Components
    ```lua
    local now = DateTime.now()
    
    -- Get UTC components
    local utc = now:ToUniversalTime()
    print(utc.Year, utc.Month, utc.Day)
    print(utc.Hour, utc.Minute, utc.Second, utc.Millisecond)
    
    -- Get local time components
    local local = now:ToLocalTime()
    ```
    
    ## Formatting
    ```lua
    local now = DateTime.now()
    
    -- ISO 8601 format
    print(now:ToIsoDate())  -- "2024-12-25T15:30:45.123Z"
    
    -- Custom format (strftime-style)
    print(now:FormatUniversalTime("%Y-%m-%d"))  -- "2024-12-25"
    print(now:FormatLocalTime("%H:%M:%S"))       -- "12:30:45"
    
    -- With locale
    print(now:FormatLocalTime("%B %d, %Y", "en_US"))  -- "December 25, 2024"
    ```
    
    ## Format Specifiers
    | Specifier | Description | Example |
    |-----------|-------------|---------|
    | %Y | 4-digit year | 2024 |
    | %m | Month (01-12) | 12 |
    | %d | Day (01-31) | 25 |
    | %H | Hour 24h (00-23) | 15 |
    | %M | Minute (00-59) | 30 |
    | %S | Second (00-59) | 45 |
    | %A | Full weekday | Wednesday |
    | %B | Full month | December |
]=]

--[=[
    @interface DateTimeValues
    Components of a date and time.
]=]
export type DateTimeValues = {
	--- 4-digit year (e.g., 2024)
	Year: number,
	--- Month (1-12)
	Month: number,
	--- Day of month (1-31)
	Day: number,
	--- Hour (0-23)
	Hour: number,
	--- Minute (0-59)
	Minute: number,
	--- Second (0-59)
	Second: number,
	--- Millisecond (0-999)
	Millisecond: number,
}

export type DateTime = {
	--- Unix timestamp in seconds (integer)
	UnixTimestamp: number,
	--- Unix timestamp in milliseconds (integer)
	UnixTimestampMillis: number,

	--- Returns date/time components in UTC
	--- @return DateTimeValues
	ToUniversalTime: (self: DateTime) -> DateTimeValues,

	--- Returns date/time components in local timezone
	--- @return DateTimeValues
	ToLocalTime: (self: DateTime) -> DateTimeValues,

	--- Returns ISO 8601 formatted string
	--- @return string -- e.g., "2024-12-25T15:30:45.123Z"
	ToIsoDate: (self: DateTime) -> string,

	--- Formats the date/time in UTC using strftime format
	--- @param format string? -- strftime format string (default: "%Y-%m-%d %H:%M:%S")
	--- @param locale string? -- Locale identifier (e.g., "en_US", "pt_BR")
	--- @return string
	FormatUniversalTime: (self: DateTime, format: string?, locale: string?) -> string,

	--- Formats the date/time in local timezone using strftime format
	--- @param format string? -- strftime format string
	--- @param locale string? -- Locale identifier
	--- @return string
	FormatLocalTime: (self: DateTime, format: string?, locale: string?) -> string,
}

local DateTime: {
	--- Returns the current date and time
	now: () -> DateTime,

	--- Creates a DateTime from a Unix timestamp in seconds
	--- @param timestamp number -- Seconds since 1970-01-01 00:00:00 UTC
	fromUnixTimestamp: (timestamp: number) -> DateTime,

	--- Creates a DateTime from a Unix timestamp in milliseconds
	--- @param millis number -- Milliseconds since epoch
	fromUnixTimestampMillis: (millis: number) -> DateTime,

	--- Creates a DateTime from UTC components
	--- @param year number -- 4-digit year
	--- @param month number -- Month (1-12)
	--- @param day number -- Day (1-31)
	--- @param hour number? -- Hour (0-23, default 0)
	--- @param minute number? -- Minute (0-59, default 0)
	--- @param second number? -- Second (0-59, default 0)
	--- @param millisecond number? -- Millisecond (0-999, default 0)
	fromUniversalTime: (
		year: number,
		month: number,
		day: number,
		hour: number?,
		minute: number?,
		second: number?,
		millisecond: number?
	) -> DateTime,

	--- Creates a DateTime from local time components
	fromLocalTime: (
		year: number,
		month: number,
		day: number,
		hour: number?,
		minute: number?,
		second: number?,
		millisecond: number?
	) -> DateTime,

	--- Parses an ISO 8601 date string
	--- @param isoDate string -- e.g., "2024-12-25T15:30:45Z"
	fromIsoDate: (isoDate: string) -> DateTime,
} =
	{} :: any

return DateTime
