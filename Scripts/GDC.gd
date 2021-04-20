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

var voxelset = load("res://2.2voxelset.tres")

# Called when the node enters the scene tree for the first time.
func _ready():
	print("GDC Init")
	
func generate_voxel_set():
	pass
#	print("generating voxel set")
#	var voxelset = VoxelSet.new()
#	#var voxelset = load("res://examples/VoxelWorld/TiledVoxelSet.tres")
#	var blocksdata = parse_blocksdata()
#	#var material = SpatialMaterial.new()
#
#
#	voxelset.set_tiles(load("res://Blocks2.2.png"))
#	voxelset.set_tile_size(Vector2(16, 16))
#
#
#	#voxelset.set_materials()
#	#print(voxelset.get_material(1))
#
#	for i in (blocksdata).size():
#		var voxel = {}
#
#		Voxel.set_material(voxel, i)
#		#voxelset.set_materials(dsfsdf)
#		#voxelset.add_voxel(Voxel.)		
#		voxelset.set_voxel(i, voxel)
#		print("Voxel info added: " + str(voxel))
#
#	#voxelset.add_voxel(Voxel.colored(Color.blue))
#
#	Gdc.voxelset = voxelset
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
		#print("cached directory entry: " + str(Vector2(ChunkX, ChunkY)))
	print("Chunk Dictionary cached!")
	
func parse_blocksdata():
	var thexml = XMLParser.new()
	thexml.open("res://BlocksData.xml")
	var blocksdata = {}
	var blockid = 0
	while thexml.read() == OK:
		blockid = thexml.get_named_attribute_value("BlockId")
		if thexml.get_node_name() == 'Block':
			var data = {}
			#print(thexml.get_named_attribute_value("Name"))
			for i in thexml.get_attribute_count():
				data[thexml.get_attribute_name(i)] = thexml.get_attribute_value(i)
			blocksdata[blockid] = data
	return blocksdata
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

func unsigned16_to_signed(unsigned):
	return (unsigned + MAX_15B) % MAX_16B - MAX_15B

func open():
	print("loading from fs")
	load_from_fs()

func load_from_fs():
	var filetype = "chunks32h"
	var _error = thefile.open(selected_file_path, File.READ_WRITE)
	print(_error)
	print("File opened")
	
	#add file ext id code
	
	if filetype == "chunks32h":
		print("Filetype detected as chunks32h")
		cache_dictionary()
		#cache_world()
		get_tree().change_scene("res://Scenes/World_Viewer.tscn")
	
	if filetype == "pak":
		print("Filetype detected as .pak")
		loadpak()
	
func loadpak():
	# Parses an entire Content.pak file
	# stringlength is the length of the string. I think it's a 7 bit int, but whatever...
	print("Loading pak file")
	
	var directory = {}
	
	thefile.seek(0)
	
	# Get package header
	var packageheader = thefile.get_32()
	
	#Get the unknown headers past the package header
	var num1 = thefile.get_64()
	var num2 = thefile.get_32()
	
	#DEBUG
	num2 = 1
	#END DEBUG
	
	for itemscount in num2:
		print("Position = " + str(thefile.get_position()))
		var stringlength = 0
		
		var name = ""
		var typeName = ""
		var position = 0
		var bytesCount = 0
		
		# Get the name of the item
		stringlength = thefile.get_8()
		name = thefile.get_buffer(stringlength)
		print("The STRINGLENGTH = " + str(stringlength))
		print(name.get_string_from_utf8())
#		for length in stringlength:
#			var thing = thefile.get_8()
#			print(str(thing))
#			#print("position == " + str(thefile.get_position()))
#			name += char(thing)
#			#print(name)
		
		# Get the typeName of the item
		stringlength = thefile.get_8()
		for length in stringlength:
			pass
#			name += char(thefile.get_8())
		
		position = thefile.get_64() + num1
		bytesCount = thefile.get_64()
		
		var item = {
			name = name,
			typeName = typeName,
			position = position,
			bytesCount = bytesCount,
		}
		
		directory[name] = item
		
		print(name)
#		print(typeName)
		print()
	
