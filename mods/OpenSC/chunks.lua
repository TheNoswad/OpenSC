--[[
This library is responsible for taking a Survivalcraft chunks file "Chunks32h.dat" and making it's data
more accesible to someone without having to interpret hieroglyphics.

For whatever reason "require "struct"" doesen't work so this will have to do
All of this data is little endian
A dictionary entry is 12 bytes long, 3 intergers, and a total of 65536 entries
]]

-- This gets the struct library. It is responsible for taking HEX and converting to nice little integers
-- Thank you Iryont.

--[[

Since this library is being used for minetest and minetest is picky, we use dofile

local struct = require "struct"
]]

local struct = dofile( OpenSCmodpath .. "/struct.lua")


-- Creates an array for storing our functions
local chunks = {}


-- A variable that stores 4 bytes!
local fourbytes = "F8FF"

-- The position for the cursor
-- Starts at zero!!!
local seekposition = "0"

-- The file path for the chunks file
-- Debugging and deving temp
local fileName = 'put chunksfile here'


-- Opens the actual file!
local file = assert(io.open(fileName, 'rb'))

-- Sets the file as the one we are working on
io.input(file)





--#######################################################################################################
-- Functions start here!




-- entries start at 0
-- This function gets a dictionary entry!
-- Just input the offset into the parenthesis
-- It returns x, y, index
local function get_dictionary_entry(entryoffset)

    -- "X" chunk position, (1 unit equals 16 blocks, must be positive)
    local x = "3"

    -- "Z" chunk position, (1 unit equals 16 blocks, must be positive)
    local z = "4"

    -- Chunk index (starting at 0)
    -- To get actual chunk offset, multiply chunk index by chunk size and add dictionary size
    local index = "5"

    -- actualbyteposition is the actual place where the starting byte of the requested dictionary entry is
    -- It is calculated by multiplying entryoffset by 12 (The size of a dictionary entry)
    local actualbyteposition = "0"
    local actualbyteposition = entryoffset * 12

    -- Set the cursor to the first 2 bytes (X chunk position)
    file:seek("set", actualbyteposition)

    -- Reads 4 bytes and sets it to local x
    local x = io.read(4)

    -- Converts the data to a 32 bit signed integer
    local x = struct.unpack("i", x)

    -- #####################################################

    -- The cursor is automatically moved to the next 4 bytes

    -- Reads 4 bytes and sets it to local y
    local z = io.read(4)

    -- Converts the data to a 32 bit signed integer
    local z = struct.unpack("i", z)

    -- ######################################################

    -- Reads 4 bytes and sets it to local index
    local index = io.read(4)

    -- Converts the data to a 32 bit signed integer
    local index = struct.unpack("i", index)



    -- Returns the local vars x, y, and index out of the function
    return x, z, index

    -- Here is some example code!
    --[[
        local a, b, c = get_dictionary_entry(0)
        print(a) -- The X Coordnate of the requested chunk
        print(b) -- The Z Coordnate of the requested chunk
        print(c) -- The Index of the requested chunk
    ]]

end

-- To get block chunk place in file do (the offset * block chunk size (263184)) + dictionary size(786444)
-- DONT FORGET HEADER IS (16bytes)

local function get_chunk_location(chunkoffset)
    --the code
    local chunk_location = "0"
    local chunk_size = "263184"
    local dictionary_size = "786444"
    local chunk_location = (chunkoffset * chunk_size) + dictionary_size
    return chunk_location
end

local function get_block_from_chunk(chunkoffset, x, y, z)
    local chunk_location = get_chunk_location(chunkoffset)

    -- Info for the dimentions of a chunk
    local max_x = "16"
    local max_y = "256"
    local max_z = "16"
    file:seek("set", chunk_location)




end


function chunks.get_chunk_data(chunkoffset)
    print("Getting Chunk Data For Chunk Number", chunkoffset)
    
    local chunk_location = get_chunk_location(chunkoffset)
    file:seek("set", chunk_location)

    -- Read 16 bytes to move past the header
    io.read(16)
    local data = {}
    local positioninchunk = "0"

    for i=1,65536 do
        local onebyte = io.read(1)
        local onebyte = struct.unpack('B', onebyte)
        table.insert(data, onebyte)
        --print(onebyte, file:seek() -1 )
        io.read(3)
        local positioninchunk = positioninchunk + 1
    end
    return data

end

function chunks.get_chunk_offset(originx, originz)
    print("Getting chunk offset for".. originx..   originz)
    file:seek("set", 0)
    local readxpos = "0"
    local readzpos = "0"
    local readoffset = "0"
    local breakout = 0

    while true do
        local readxpos = io.read(4)
        local readzpos = io.read(4)
        local readoffset = io.read(4)
        minetest.debug(readzpos)
        local readxpos = struct.unpack('i', readxpos)
        local readzpos = struct.unpack('i', readzpos)
        local readoffset = struct.unpack('i', readoffset)
        --print(readxpos)
        --print(readoffset)

        if readxpos == originx then
            if readzpos == originz then
                print("Found the requested chunk")
                return readoffset
            else
                print("Nope, Not that one")
            end
        end
        -- Checks to see if we reached the end of the chunks (-1, -1, 0)
        if readoffset == -1 then
            print("Did not find the requested chunk. Aborting.")
            return readoffset
        end
end
return readoffset
end

-- Test codes down here!


return chunks