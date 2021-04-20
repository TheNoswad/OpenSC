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

extends Label


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var x = 0
var y = 0
var z = 0
var camera = null


# Called when the node enters the scene tree for the first time.
func _ready():
	camera = get_viewport().get_camera()
	

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	x = camera.transform.origin.x
	y = camera.transform.origin.y
	z = camera.transform.origin.z
	
	text = str(int(x)) + " " + str(int(y)) + " " + str(int(z))
	#text = str(get_viewport().get_camera().transform.origin)
