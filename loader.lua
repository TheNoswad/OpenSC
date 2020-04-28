-- Get content IDs during load time, and store into a local
local c_dirt  = minetest.get_content_id("default:dirt")
local c_grass = minetest.get_content_id("default:dirt_with_grass")
local c_air   = minetest.get_content_id("air")
local c_bedrock = minetest.get_content_id("default:stone_with_tin")
local c_sand = minetest.get_content_id("default:sand")
local c_stone = minetest.get_content_id("default:stone")

local worldloader = {}

print("The thing is running!")

local chunks = dofile( OpenSCmodpath .. "/chunks.lua")

local chunkdata = {}

local function load_chunk_into_world(pos1, pos2)

    -- Sets up the array to store coordinate values for the lvm
    local chunk_coords_near = {}
    local chunk_coords_far = {}

    -- This is the origin of the chunk
    chunk_coords_near.x = pos1 *16
    chunk_coords_near.y = 0
    chunk_coords_near.z = pos2 *16

    -- Add 15 to calculate the far origin(The ending corner of the chunk)
    chunk_coords_far.x = (pos1 * 16) + 15
    chunk_coords_far.y = 255
    chunk_coords_far.z = (pos2 * 16) + 15
    
    -- Read data into LVM
    print("reading into lvm")
    local vm = minetest.get_voxel_manip()
    local emin, emax = vm:read_from_map(chunk_coords_near, chunk_coords_far)
    local a = VoxelArea:new{
        MinEdge = emin,
        MaxEdge = emax
    }
    local data = vm:get_data()
    local listcounter = 1

    local chunk_offset =chunks.get_chunk_offset(pos1, pos2)
    -- Get the data from the requested chunk and store it.
    local data_from_a_chunk = chunks.get_chunk_data(chunk_offset)

    local blockcounter = "0"

    print("Loading chunk at origin".. chunk_coords_near.x, chunk_coords_near.z)

    -- Modify data
    for z = chunk_coords_near.z, chunk_coords_far.z do
        --print("tabz" .. z)
        for x = chunk_coords_near.x, chunk_coords_far.x do
            --print("tabx" .. x)
            for y = chunk_coords_near.y, chunk_coords_far.y do

                -- Sets the position inside the voxel manipulator
                local vi = a:index(x, y, z)

                --if data[vi] == c_grass then
                if data_from_a_chunk[listcounter] == 1 then
                    data[vi] = c_bedrock
                    --print("found air!")
                    --print(data_from_a_chunk[listcounter])
                    listcounter = listcounter + 1
                else
                    if data_from_a_chunk[listcounter] == 2 then
                        data[vi] = c_dirt
                        listcounter = listcounter + 1
                    else
                        if data_from_a_chunk[listcounter] == 3 then
                            data[vi] = c_stone
                            listcounter = listcounter + 1
                        else
                            listcounter = listcounter + 1
                            data[vi] = c_air
                            --print("There is a block at" .. listcounter)
                            --print(data_from_a_chunk[listcounter])
                        end
                    end
                end
            end
        end
    end

    -- Set the final data in the voxel manipulator
    vm:set_data(data)
    -- Write the final data to the map
    vm:write_to_map(true)
end

function worldloader.testaprint(arg1, arg2, arg3)
    print("hi")
end

-- A chunk's origin is at the starting coordinate of it. Ex: (x,y,z)  16, 0, 16     35, 255, 31
--                                                           The origin would be 16, 16


--print("offset is ".. chunks.get_chunk_offset(-9, -4))
local thingie1 = {}
local thingie2 = {}
thingie1.x = 16
thingie1.y = 0
thingie1.z = 16

thingie2.x = 31
thingie2.y = 255
thingie2.z = 31

load_chunk_into_world(-9, -4)
load_chunk_into_world(-9, -3)
load_chunk_into_world(-9, -2)
load_chunk_into_world(-8, -4)
load_chunk_into_world(-8, -3)
--[[
local data_from_a_chunk = chunks.get_chunk_data(0)
print(data_from_a_chunk[1])
print(data_from_a_chunk[2])
print(data_from_a_chunk[3])
print(data_from_a_chunk[4])
print(data_from_a_chunk[5])
print(data_from_a_chunk[6])
print(data_from_a_chunk[7])
print(data_from_a_chunk[8])
print(data_from_a_chunk[9])
print(data_from_a_chunk[10])
]]

return worldloader