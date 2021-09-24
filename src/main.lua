local ffi = require('ffi')

local ext

if ffi.os == 'Linux' then
    ext = 'so'
else
    ext = 'dylib'
end

ffi.cdef[[
char * cid_test();
char * string_from_rust();
size_t v0_to_v1(const char* v0, size_t v0_len, char* result, size_t result_len);
]]

local lib = ffi.load('target/debug/libdouble_input.' .. ext)
local string_from_rust = lib.string_from_rust
local cid_test = lib.cid_test
local v0_to_v1_rust = lib.v0_to_v1

local old_cid = "QmT5NvUtoM5nWFfrQdVrFtvGfKFmG7AHE8P34isapyhCxX"
local new_cid = "bafybeicgmdpvw4duutrmdxl4a7gc52sxyuk7nz5gby77afwdteh3jc5bqa"

local c_str = ffi.new("char[?]", #old_cid)
ffi.copy(c_str, old_cid)

-- print(ffi.string(string_from_rust()))
-- print(ffi.string(cid_test()))
print(old_cid)
print(new_cid)

local function v0_to_v1(v0)
    print("fetching " .. v0)
    local ot = ffi.typeof "char[?]"
    local result_buffer = ffi.new(ot, 255)
    local len = v0_to_v1_rust(v0, #v0, result_buffer, 255)
    return ffi.string(result_buffer, len)
end

local cid = v0_to_v1(old_cid)

print(cid)
