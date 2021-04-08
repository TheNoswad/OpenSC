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

extends FileDialog


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var blockid = 0x00
	
	var databyte_1 = 0x12
	var databyte_2 = 0x40
	var databyte_3 = 0x00
	var databyte_4 = 0x00
	
	blockid = (databyte_2 >> 6) + (databyte_1)
	print(blockid)




# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Button_button_down():
	popup()


func _on_FileDialog_file_selected(path):
	Gdc.selected_file_path = path
	print("selected_file_path = " + Gdc.selected_file_path)
	Gdc.filename = current_file
	Gdc.open()
