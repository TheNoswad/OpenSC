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

extends GridMap


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _ready():
	var loopcounttest = 0
	#load_chunk(Vector2(0, 3))
	#print(Gdc.chunks_dictionary)
	for i in Gdc.chunks_dictionary:
		
		
		
		#var thread = Thread.new()
		#thread.start(self, "load_chunk", i)
		#print("started thread")
		loopcounttest = loopcounttest + 1
		print(i)
		print(loopcounttest)
		load_chunk(i)
		if loopcounttest == 15:
			break	

# Called when the node enters the scene tree for the first time.
func load_chunk(chunk_location):
	
	
	var voxel_chunk = VoxelMesh.new()
	voxel_chunk.uv_map = true
	voxel_chunk.voxel_set = Gdc.voxelset
	var chunkoffsetx = chunk_location.x
	var chunkoffsety = chunk_location.y
	
	var chunk_location_global = Vector2(chunkoffsetx * 16, chunkoffsety * 16)
	
	
	var blockid = 0
	
	#THIS WORKS!
	#print(Gdc.chunk_cache[chunk_location][Vector3(0, 1, 13)])
	#print(Gdc.chunk_cache.get(chunk_location).size())

	
	Gdc.cache_chunk(chunk_location)
	#voxel_chunk.set_voxels(Gdc.chunk_cache[chunk_location])
	for key in Gdc.chunk_cache[chunk_location]:
		blockid = Gdc.chunk_cache[chunk_location][key]["block_id"]
		
		
			
		#set_cell_item(chunk_location_global.x + key.x, key.y, chunk_location_global.y + key.z, blockid)
		
		
		if blockid != 0:
			pass
			voxel_chunk.set_voxel(Vector3(key.x, key.y, key.z), blockid)
	
	voxel_chunk.update_mesh()
	voxel_chunk.translation.x = chunk_location_global.x/2
	voxel_chunk.translation.z = chunk_location_global.y/2
	#voxel_chunk.translate(Vector3(chunk_location_global.x, 0, chunk_location_global.y))
	add_child(voxel_chunk)
	Gdc.free_cached_chunk(chunk_location)



