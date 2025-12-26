-- tests/api/test_datetime.luau
-- Comprehensive Tests for DateTime global

print("Testing DateTime...")

-- 1. Check DateTime exists
assert(type(DateTime) == "table", "DateTime should be a table")

-- 2. DateTime.now()
print("  > Testing DateTime.now()")
local now = DateTime.now()
assert(type(now) == "userdata", "DateTime.now() returns userdata")

-- 3. Properties
print("  > Testing properties")
local ts = now.UnixTimestamp
assert(type(ts) == "number", "UnixTimestamp is number")
assert(ts > 0, "UnixTimestamp should be positive")

local tsMs = now.UnixTimestampMillis
assert(type(tsMs) == "number", "UnixTimestampMillis is number")
assert(tsMs >= ts * 1000, "UnixTimestampMillis should be >= ts * 1000")

-- 4. ToIsoDate
print("  > Testing ToIsoDate")
local iso = now:ToIsoDate()
assert(type(iso) == "string", "ToIsoDate returns string")
assert(string.match(iso, "%d%d%d%d%-%d%d%-%d%d"), "ISO format YYYY-MM-DD")

-- 5. ToUniversalTime
print("  > Testing ToUniversalTime")
local utc = now:ToUniversalTime()
assert(type(utc) == "table", "ToUniversalTime returns table")
assert(type(utc.Year) == "number", "Year is number")
assert(type(utc.Month) == "number", "Month is number")
assert(type(utc.Day) == "number", "Day is number")
assert(type(utc.Hour) == "number", "Hour is number")
assert(type(utc.Minute) == "number", "Minute is number")
assert(type(utc.Second) == "number", "Second is number")
assert(utc.Year >= 2020, "Year should be >= 2020")
assert(utc.Month >= 1 and utc.Month <= 12, "Month 1-12")
assert(utc.Day >= 1 and utc.Day <= 31, "Day 1-31")

-- 6. ToLocalTime
print("  > Testing ToLocalTime")
local localTime = now:ToLocalTime()
assert(type(localTime) == "table", "ToLocalTime returns table")
assert(type(localTime.Year) == "number", "Local Year is number")

-- 7. fromUnixTimestamp
print("  > Testing fromUnixTimestamp")
local dt2000 = DateTime.fromUnixTimestamp(946684800) -- 2000-01-01 00:00:00 UTC
assert(dt2000 ~= nil, "fromUnixTimestamp should return DateTime")
local utc2000 = dt2000:ToUniversalTime()
assert(utc2000.Year == 2000, "Year should be 2000")
assert(utc2000.Month == 1, "Month should be 1")
assert(utc2000.Day == 1, "Day should be 1")

-- 8. fromUnixTimestampMillis
print("  > Testing fromUnixTimestampMillis")
local dtMillis = DateTime.fromUnixTimestampMillis(946684800000)
assert(dtMillis ~= nil, "fromUnixTimestampMillis should return DateTime")
assert(dtMillis.UnixTimestamp == 946684800, "Timestamp should match")

-- 9. fromUniversalTime
print("  > Testing fromUniversalTime")
local dtCustom = DateTime.fromUniversalTime(2025, 6, 15, 12, 30, 45, 500)
assert(dtCustom ~= nil, "fromUniversalTime should return DateTime")
local utcCustom = dtCustom:ToUniversalTime()
assert(utcCustom.Year == 2025, "Year 2025")
assert(utcCustom.Month == 6, "Month 6")
assert(utcCustom.Day == 15, "Day 15")
assert(utcCustom.Hour == 12, "Hour 12")
assert(utcCustom.Minute == 30, "Minute 30")
assert(utcCustom.Second == 45, "Second 45")

-- 10. fromIsoDate
print("  > Testing fromIsoDate")
local dtIso = DateTime.fromIsoDate("2025-12-26T00:00:00Z")
assert(dtIso ~= nil, "fromIsoDate should return DateTime")
local utcIso = dtIso:ToUniversalTime()
assert(utcIso.Year == 2025, "ISO Year 2025")
assert(utcIso.Month == 12, "ISO Month 12")
assert(utcIso.Day == 26, "ISO Day 26")

-- 11. Round-trip
print("  > Testing round-trip")
local original = DateTime.now()
local isoStr = original:ToIsoDate()
local parsed = DateTime.fromIsoDate(isoStr)
assert(parsed ~= nil, "Parsed should not be nil")
-- Allow 1 second difference due to millisecond truncation
assert(math.abs(parsed.UnixTimestamp - original.UnixTimestamp) < 2, "Round-trip within 2 seconds")

-- 12. FormatUniversalTime
print("  > Testing FormatUniversalTime")
local formatted = now:FormatUniversalTime("%Y-%m-%d")
assert(type(formatted) == "string", "FormatUniversalTime returns string")
assert(string.match(formatted, "%d%d%d%d%-%d%d%-%d%d"), "Format matches pattern")

-- 13. FormatLocalTime
print("  > Testing FormatLocalTime")
local formattedLocal = now:FormatLocalTime("%H:%M:%S")
assert(type(formattedLocal) == "string", "FormatLocalTime returns string")

-- 14. Comparison
print("  > Testing comparison")
local earlier = DateTime.fromUnixTimestamp(1000)
local later = DateTime.fromUnixTimestamp(2000)
assert(earlier ~= later, "Different timestamps should not be equal")

print("DateTime Tests Passed!")
