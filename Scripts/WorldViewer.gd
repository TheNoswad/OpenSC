extends GridMap


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func load_chunk(chunk_location):
	var chunkoffsetx = chunk_location.x
	var chunkoffsety = chunk_location.y
	
	var chunk_location_global = Vector2(chunkoffsetx * 16, chunkoffsety * 16)
	
	var x = 0
	var z = 0
	var y = 0
	
	var blockid = 0
	
	#THIS WORKS!
	#print(Gdc.chunk_cache[chunk_location][Vector3(0, 1, 13)])
	
	for key in Gdc.chunk_cache[chunk_location]:
		# print("Placing blocks ")
		#print(x + " " + y + "" + z)
		#print(get_8(key.get()))
		blockid = Gdc.chunk_cache[chunk_location][key]["block_id"]
		#print(blockid)
		
			
		set_cell_item(chunk_location_global.x + key.x, key.y, chunk_location_global.y + key.z, blockid)
	#print("loaded chunk at offset " + String(chunk_location))




func _ready():
	load_chunk(Vector2(0, 2))
	#for i in Gdc.chunks_dictionary:
	#	print(i)
	#	load_chunk(i)
		


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
