-- Get content IDs during load time, and store into a local
local c_dirt  = minetest.get_content_id("default:dirt")
local c_grass = minetest.get_content_id("default:dirt_with_grass")
local c_air   = minetest.get_content_id("air")
local c_bedrock = minetest.get_content_id("default:stone_with_tin")
local c_sand = minetest.get_content_id("default:sand")
local c_stone = minetest.get_content_id("default:stone")


local OpenSC_Content_IDS = {}


-- Sets up content id's in a table so it is faster.
for x = 0, 19 do
    print("working", x)
    table.insert(OpenSC_Content_IDS, x, minetest.get_content_id(OpenSCBlockIds[x]))
end


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
    for x = chunk_coords_near.x, chunk_coords_far.x do
        --print("tabz" .. z)
        for z = chunk_coords_near.z, chunk_coords_far.z do
            --print("tabx" .. x)
            for y = chunk_coords_near.y, chunk_coords_far.y do

                local vi = a:index(x, y, z)
                --local currentdata = data_from_a_chunk[listcounter]--OpenSCBlockIds[currentdata]
                --print("the data is" .. data_from_a_chunk[listcounter] .. "end")
                --if data[vi] == c_grass then
                if true then
                    data[vi] = OpenSC_Content_IDS[data_from_a_chunk[listcounter]]
                    --data[vi] = OpenSCBlockIds[data_from_a_chunk[listcounter]]
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


-- A chunk's origin is at the starting coordinate of it. Ex: (x,y,z)  16, 0, 16     35, 255, 31
--                                                           The origin would be 16, 16

load_chunk_into_world(-9, -4)
load_chunk_into_world(-9, -3)
load_chunk_into_world(-9, -2)
load_chunk_into_world(-8, -4)
load_chunk_into_world(-8, -3)
load_chunk_into_world(-8, -2)
load_chunk_into_world(-8, -1)
load_chunk_into_world(-8, 0)
load_chunk_into_world(-8, 1)
load_chunk_into_world(-7, 2)
load_chunk_into_world(-7, -4)
load_chunk_into_world(-7, -3)
load_chunk_into_world(-7, -1)
load_chunk_into_world(-7, 0)
load_chunk_into_world(-7, 1)
load_chunk_into_world(-8, 2)
load_chunk_into_world(-6, -4)
load_chunk_into_world(-6, -3)
load_chunk_into_world(-6, -2)
load_chunk_into_world(-6, -1)
load_chunk_into_world(-6, 0)
load_chunk_into_world(-6, 1)
load_chunk_into_world(-9, 2)


return worldloader