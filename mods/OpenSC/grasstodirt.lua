-- Get content IDs during load time, and store into a local
local c_dirt  = minetest.get_content_id("tnt:tnt")
local c_grass = minetest.get_content_id("air")

print("The thing is running!")

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

    -- Modify data
    for z = pos1.z, pos2.z do
        for y = pos1.y, pos2.y do
            for x = pos1.x, pos2.x do
                local vi = a:index(x, y, z)
                if data[vi] == c_grass then
                    data[vi] = c_dirt
                end
            end
        end
    end

    -- Write data
    vm:set_data(data)
    vm:write_to_map(true)
end
--[[
function testaprint(arg1, arg2, arg3)
    print("hi")
end
]]

local thingie1 = {}
local thingie2 = {}
thingie1.x = 0
thingie1.y = 0
thingie1.z = 0

thingie2.x = 20
thingie2.y = 22
thingie2.z = 24

grass_to_dirt(thingie1, thingie2)