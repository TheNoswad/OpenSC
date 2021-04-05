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
	
	
	for i in Gdc.chunk_cache[chunk_location]:
		set_cell_item(chunk_location_global.x + x, chunk_location_global.y + y, z, 1)
		if x == 16:
			x = 0
		if z == 16:
			z = 0
		if y == 256:
			y = 0
	print("loaded chunk at " + String(chunk_location))




func _ready():
	for i in Gdc.chunks_dictionary:
		#print(i)
		load_chunk(i)
		


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
