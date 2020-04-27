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
    -- Read data into LVM
    print("reading into lvm")
    local vm = minetest.get_voxel_manip()
    local emin, emax = vm:read_from_map(pos1, pos2)
    local a = VoxelArea:new{
        MinEdge = emin,
        MaxEdge = emax
    }
    local data = vm:get_data()
    local listcounter = 1

    -- Get the data from the requested chunk and store it.
    local data_from_a_chunk = chunks.get_chunk_data(0)

    local blockcounter = "0"
    -- Modify data
    for z = pos1.z, pos2.z do
        --print("tabz" .. z)
        for x = pos1.x, pos2.x do
            --print("tabx" .. x)
            for y = pos1.y, pos2.y do

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


print("offset is ".. chunks.get_chunk_offset(-9, -3))
local thingie1 = {}
local thingie2 = {}
thingie1.x = 16
thingie1.y = 0
thingie1.z = 16

thingie2.x = 31
thingie2.y = 255
thingie2.z = 31

load_chunk_into_world(thingie1, thingie2)
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