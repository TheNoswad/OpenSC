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

local function grass_to_dirt(pos1, pos2)
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

    -- Get the data from a chunk and store it.
    local data_from_a_chunk = chunks.get_chunk_data(0)
    print(data_from_a_chunk[1])
    print(data_from_a_chunk[2])
    print(data_from_a_chunk[3])

    -- Modify data
    for z = pos1.z, pos2.z do
        print("tabz" .. z)
        for x = pos1.x, pos2.x do
            print("tabx" .. x)
            for y = pos1.y, pos2.y do
                --print("taby" .. y)
                local vi = a:index(x, (y + x) + (1+ z *17)+16, z)
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

    -- Write data
    vm:set_data(data)
    vm:write_to_map(true)
end

function worldloader.testaprint(arg1, arg2, arg3)
    print("hi")
end




local thingie1 = {}
local thingie2 = {}
thingie1.x = 0
thingie1.y = 0
thingie1.z = 0

thingie2.x = 16
thingie2.y = 256
thingie2.z = 16

grass_to_dirt(thingie1, thingie2)
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