-- Typedefs for rere_std_fs

--[[
    Reads the file at ``path`` and returns it's contents as a string.
]]
local function readFile(path: string): string return nil :: any end

--[[
    Writes ``contents`` to the file at ``path``.
]]
local function writeFile(path: string, contents: string | buffer): () end

--[[
    Removes the file at ``path``.
]]
local function removeFile(path: string): () end

--[[
    Removes the directory at ``path``.
]]
local function removeDir(path: string): () end

--[[
    Appends ``contents`` to the file at ``path``.
]]
local function appendFile(path: string, contents: string | buffer): () end

--[[
    Returns whether or not the file at ``path`` is set to read only.
]]
local function isReadOnly(path: string): boolean return nil :: any end

--[[
    Enables or disables the read only property on the file at ``path`` depending on ``readonly``.
]]
local function readOnly(path: string, readonly: boolean): () end

--[[
    Returns whether or not the file at ``path`` is a directory.
]]
local function isDir(path: string): boolean return nil :: any end

--[[
    Returns whether or not the file at ``path`` is a file.
]]
local function isFile(path: string): boolean return nil :: any end

--[[
    Returns the size of the file at ``path`` in bytes.
]]
local function size(path: string): number return nil :: any end

--[[
    Returns a table of strings with all files or directories in the directory at ``path``.
]]
local function readDir(path: string): {string} return nil :: any end

--[[
    Returns whether or not the file/directory at ``path`` exists.
]]
local function exists(path: string): boolean return nil :: any end

return {
    readFile = readFile,
    writeFile = writeFile,
    removeFile = removeFile,
    removeDir = removeDir,
    appendFile = appendFile,
    isReadOnly = isReadOnly,
    readOnly = readOnly,
    isDir = isDir,
    isFile = isFile,
    size = size,
    readDir = readDir,
    exists = exists
}