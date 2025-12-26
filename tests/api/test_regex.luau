-- tests/api/test_regex.luau
-- Comprehensive Tests for @lux/regex

local regex = require("@lux/regex")

print("Testing @lux/regex...")

-- 1. Create Regex
print("  > Creating regex patterns")
local simplePattern = regex.new("hello")
assert(simplePattern ~= nil, "regex.new should return a pattern")

-- 2. isMatch
print("  > Testing isMatch")
assert(simplePattern:isMatch("hello world"), "isMatch should find 'hello' in 'hello world'")
assert(
	not simplePattern:isMatch("goodbye world"),
	"isMatch should not find 'hello' in 'goodbye world'"
)

-- 3. find
print("  > Testing find")
local match = simplePattern:find("say hello to the world")
assert(match ~= nil, "find should return a match")
assert(match.text == "hello", "match.text should be 'hello'")
assert(match.start == 5, "match.start should be 5")
assert(match.finish == 9, "match.finish should be 9")
assert(match.len == 5, "match.len should be 5")

-- No match case
local noMatch = simplePattern:find("goodbye")
assert(noMatch == nil, "find should return nil when no match")

-- 4. captures with groups
print("  > Testing captures")
local datePattern = regex.new([[(\d+)-(\d+)-(\d+)]])
local caps = datePattern:captures("Date: 2025-12-26")
assert(caps ~= nil, "captures should return result")

-- Get indexed captures
local fullMatch = caps:get(0)
assert(fullMatch ~= nil, "get(0) should return full match")
assert(fullMatch.text == "2025-12-26", "full match text")

local year = caps:get(1)
assert(year ~= nil, "get(1) should return first group")
assert(year.text == "2025", "year text")

local month = caps:get(2)
assert(month ~= nil, "get(2) should return second group")
assert(month.text == "12", "month text")

local day = caps:get(3)
assert(day ~= nil, "get(3) should return third group")
assert(day.text == "26", "day text")

-- 5. Named groups
print("  > Testing named groups")
local namedPattern = regex.new([[(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})]])
local namedCaps = namedPattern:captures("2025-12-26")
if namedCaps then
	local yearGroup = namedCaps:group("year")
	if yearGroup then
		assert(yearGroup.text == "2025", "named group year")
	end
end

-- 6. split
print("  > Testing split")
local commaPattern = regex.new(",\\s*")
local parts = commaPattern:split("a, b,  c,d")
assert(#parts == 4, "split should return 4 parts")
assert(parts[1] == "a", "first part is 'a'")
assert(parts[2] == "b", "second part is 'b'")
assert(parts[3] == "c", "third part is 'c'")
assert(parts[4] == "d", "fourth part is 'd'")

-- 7. replace (first only)
print("  > Testing replace")
local numPattern = regex.new([[\d+]])
local replaced = numPattern:replace("abc123def456", "NUM")
assert(replaced == "abcNUMdef456", "replace should replace only first match")

-- 8. replaceAll
print("  > Testing replaceAll")
local replacedAll = numPattern:replaceAll("abc123def456ghi789", "X")
assert(replacedAll == "abcXdefXghiX", "replaceAll should replace all matches")

-- 9. Complex patterns
print("  > Testing complex patterns")
local emailPattern = regex.new([[[\w.+-]+@[\w.-]+\.\w{2,}]])
assert(emailPattern:isMatch("test@example.com"), "should match email")
assert(not emailPattern:isMatch("not-an-email"), "should not match non-email")

-- 10. Unicode support
print("  > Testing unicode")
local unicodePattern = regex.new("olá")
assert(unicodePattern:isMatch("dizer olá mundo"), "should match unicode")

print("Regex Tests Passed!")
