-- Typedefs for rere_std_io

--[[
    Writes ``string`` to stdout. Accepts ASCII.
]]
local function write(string: string): () end

--[[
    Writes ``string`` to stderr. Accepts ASCII.
]]
local function ewrite(string: string): () end

return {
    write = write,
    ewrite = ewrite
}