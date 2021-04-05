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
var chunk_data = {}

var chunks_data_section_start = 786444
var chunk_size = 132112
var chunks_directory_size = 786444


# Called when the node enters the scene tree for the first time.
func _ready():
	print("GDC Init")

func cache_world():
	var ChunkX = 0
	var ChunkY = 0
	var offset = 0
	for n in 65536:
		ChunkX = unsigned16_to_signed(thefile.get_32())
		ChunkY = unsigned16_to_signed(thefile.get_32())
		offset = unsigned16_to_signed(thefile.get_32())
		#Sets the dict name to a vector2 of ChunkX and ChunkY and the value to the chunk offset
		chunks_dictionary[Vector2(ChunkX, ChunkY)] = offset
	print("Chunk Dictionary cached!")
	
	var chunkdata = 0
	
	for key in chunks_dictionary:
		#goto the chunk location
		thefile.seek((chunks_dictionary[key] * chunk_size) + chunks_directory_size)
		
		#skip past the header
		thefile.get_32()
		thefile.get_32()
		
		ChunkX = unsigned16_to_signed(thefile.get_32())
		ChunkY = unsigned16_to_signed(thefile.get_32())
		
		for x in 16:
			for z in 16:
				for y in 256:
					chunk_data[Vector3(x, z, y)] = unsigned16_to_signed(thefile.get_32())
		chunk_cache[key] = chunk_data
		
		
		
		chunk_cache[chunks_dictionary[key]] = chunkdata
		
		
	



func unsigned16_to_signed(unsigned):
	return (unsigned + MAX_15B) % MAX_16B - MAX_15B

func open():
	load_from_fs()

func load_from_fs():
	var error = thefile.open(selected_file_path, File.READ_WRITE)
	#print(error)
	print("File opened")
	
	#add file ext id code
	cache_world()
	get_tree().change_scene("res://Scenes/World_Viewer.tscn")
	
	
