print("we made it to worldloader.lua !!!")

local worldloader = {}



function worldloader.testforlearn(nothing)
    print("We made it to the function!!")
    
    local pos1 = {}
    local pos2 = {}

    pos1.x = 10
    pos1.y = 12
    pos1.z = 14

    pos2.x = 20
    pos2.y = 22
    pos2.z = 24

    local vm = minetest.get_voxel_manip()
    local emin, emax = vm:read_from_map(pos1, pos2)


    local a = VoxelArea:new{
        MinEdge = emin,
        MaxEdge = emax
    }

    print(a)

    local data = vm:get_data()
    local x = "1"
    local y = "1"
    local z = "1"


    -- Get node's index
    local idx = a:index(x, y, z)

    -- Read node
    print(data[idx])

end

--worldloader.testforlearn = testforlearn()

return worldloader