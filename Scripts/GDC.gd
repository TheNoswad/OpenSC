#Copyright 2020-2021 Noswad
#
#This program is free software: you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation, either version 3 of the License, or
#(at your option) any later version.
#
#This program is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.
#
#You should have received a copy of the GNU General Public License
#along with this program.  If not, see <http://www.gnu.org/licenses/>.

extends Node

##This is the global data cashe.
##The global data cashe manages cashed data

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

const MAX_15B = 1 << 15
const MAX_16B = 1 << 16

var test = 420
var selected_file_path = null
var thefile = File.new()
var thefilename = "null"
var chunks_dictionary = {}
var chunk_cache = {}
#var chunk_data = {}
var block_data = {}

var chunks_data_section_start = 786444
var chunk_size = 263184
var chunks_directory_size = 786444


# Called when the node enters the scene tree for the first time.
func _ready():
	print("GDC Init")
	
func cache_dictionary():
	var ChunkX = null
	var ChunkY = null
	var offset = null
	thefile.seek(0)
	
	for n in 65536:
		ChunkX = unsigned16_to_signed(thefile.get_32())
		ChunkY = unsigned16_to_signed(thefile.get_32())
		offset = unsigned16_to_signed(thefile.get_32())
		#Sets the dict name to a vector2 of ChunkX and ChunkY and the value to the chunk offset
		chunks_dictionary[Vector2(ChunkX, ChunkY)] = offset
		print("cached directory entry: " + str(Vector2(ChunkX, ChunkY)))
	print("Chunk Dictionary cached!")
	
func cache_chunk(location):
	print("caching chunk: " + str(location))
	#goto the chunk location
	#           |the offset of the chunk| 
	thefile.seek((chunks_dictionary[location] * chunk_size) + chunks_directory_size)
	#print("parsing chunk: " + str(key))
	#print("chunks_dictionary: " + str(chunks_dictionary[key]))
	#print("the seeek pos: " + str(int(chunks_dictionary[key] * chunk_size) + chunks_directory_size))
	
	#skip past the header
	thefile.get_32()
	thefile.get_32()
	
	var chunk_data = {}
	
	#ChunkX = unsigned16_to_signed(thefile.get_32())
	#ChunkY = unsigned16_to_signed(thefile.get_32())
	
	for x in 16:
		for y in 16:
			for z in 256:
				var databyte_1 = thefile.get_8()
				var databyte_2 = thefile.get_8()
				var databyte_3 = thefile.get_8()
				var databyte_4 = thefile.get_8()
				
				#shift bits to get the bits!
				#this is not finished. Idk how to fix it rn.
				var block_id = databyte_1
				var light_value = databyte_2 << 6
				var block_state = databyte_2 << 6
				
				block_data = {
					block_id = block_id,
					light_value = light_value,
					block_state = block_state,
				}
				
				#block_data[Vector3(x, y, z)] = thefile.get_8()
				#chunk_data[Vector3(x, z, y)] = unsigned16_to_signed(thefile.get_32())
				
				#chunk_data[Vector3(key.x, z, key.y)] = block_data
				chunk_data[Vector3(x, z, y)] = block_data
	chunk_cache[location] = chunk_data
	print("Finished")

func free_cached_chunk(location):
	print("Freeing cached chunk: " + str(location))
	chunk_cache.erase(location)

func cache_world():
	
	#iterates over the entire 65536 dictionary entries
	cache_dictionary()	

	
	#var chunkdata = 0
	
	#cache each chunk
	#for key in chunks_dictionary:
	#	cache_chunk(key)

func unsigned16_to_signed(unsigned):
	return (unsigned + MAX_15B) % MAX_16B - MAX_15B

func open():
	load_from_fs()

func load_from_fs():
	var error = thefile.open(selected_file_path, File.READ_WRITE)
	#print(error)
	print("File opened")
	
	#add file ext id code
	cache_dictionary()
	#cache_world()
	get_tree().change_scene("res://Scenes/World_Viewer.tscn")
	
	
