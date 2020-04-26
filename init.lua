--local worldloader = assert(dofile(minetest.get_modpath("opensc") .. "/grasstodirt.lua"))
--dofile("/home/dawson/snap/minetest/current/mods/opensc/worldloader.lua")
--local worldloader = require "/home/dawson/snap/minetest/current/mods/opensc/grasstodirt.lua"
OpenSCmodpath = minetest.get_modpath( "opensc" )



minetest.after(5,
        function(params)
        local worldloader = dofile( OpenSCmodpath .. "/loader.lua")
        --local worldloader
        worldloader.testaprint()

        print("Hello, World")
	end
)

