-- this is the library for reading chunks32h.dat

-- The 4 bytes to store
local byte1 = "GG"
local byte2 = "GG"
local byte3 = "GG"
local byte4 = "GG"
-- ************************
-- seekpos is the byte in the file to set the cursor to (like byte 1 or byte 21 ....)
local seekpos = "1"


-- Loads the file path
local fileName = '/home/dawson/snap/minetest/current/mods/OpenSC/Chunks32h.dat'

-- Loads the file
local file = assert(io.open(fileName, 'rb'))

-- Sets file as default
io.input(file)

-- Sets the cursor back where it belongs because it likes to move around for some reason.
file:seek("set", seekpos)
-- Sets byte1 to the first byte needed as decimal format
byte1 = string.byte(io.read(2), seekpos)
-- Sets byte1 to the actual hex value
byte1 = string.format("%X", byte1)

-- Sets the cursor back where it belongs because it likes to move around for some reason.
file:seek("set", seekpos)
-- Sets byte2 to the second byte needed as decimal format
byte2 = string.byte(io.read(), seekpos + 1)
-- Sets byte2 to the actual hex value
byte2 = string.format("%X", byte2)




--file:seek("set", 1)


print(byte1)
print(byte2)
print(byte3)
print(byte4)
print(io.read(2))
print(file:seek())
