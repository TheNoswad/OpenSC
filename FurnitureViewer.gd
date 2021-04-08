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
var values = "164*0,1*21,3*0,1*21,4*0,1*21,3*0,1*21,4*0,1*21,3*0,1*21,4*0,5*21,76*0,5*21,76*0,5*21,76*0,5*21,49*0,1*21,3*0,1*21,4*0,1*21,3*0,1*21,4*0,1*21,3*0,1*21,4*0,5*21,4*0,1*21,1*0,1*21,1*0,1*21,4*0,1*21,1*0,1*21,1*0,1*21,4*0,5*21,4*0,1*21,3*0,1*21,4*0,5*21,164*0"
var resolution = 9

# Called when the node enters the scene tree for the first time.
func _ready():
	var thethingarray = values.rsplit(",", true)
	var linpos = 0
	
	var xpos = 1
	var ypos = 1
	var zpos = 1
	
	for i in thethingarray:
		var blocklong = i.split("*")
		
		#print(blocklong[0])
		
		# The loop that itterates for each item in the "values" string.
		for n in int(blocklong[0]):
			print(n)
			if int(blocklong[1]) != 0:
				print("setting block at: " + str(xpos) + " " + str(ypos) + " " + str(zpos))
				set_cell_item(xpos, ypos, zpos, 1)
				xpos = xpos + 1
				
				if xpos >= resolution + 1:
					print("xpos rollover")
					xpos = 1
					ypos = ypos + 1
					
				if ypos >= resolution + 1:
					print("ypos rollover")
					ypos = 1
					zpos = zpos + 1
				
			else:
				print("Air Block")
				xpos = xpos + 1
				
				if xpos >= resolution + 1:
					print("xpos rollover")
					xpos = 1
					ypos = ypos + 1
					
				if ypos >= resolution + 1:
					print("ypos rollover")
					ypos = 1
					zpos = zpos + 1
		
		if xpos >= resolution + 1:
			#print("xpos rollover")
			xpos = 1
			ypos = ypos + 1
			
		if ypos >= resolution + 1:
			print("ypos rollover")
			ypos = 1
			zpos = zpos + 1
		#print("xpos: " + str(xpos) + " ypos: " + str(ypos) + " zpos: " + str(zpos))
		
		
		
#	var thexml = XMLParser.new()
#	thexml.open("res://FurnitureDesigns.xml")
#
#	while thexml.read() == OK:
#		var nodename = thexml.get_node_name()
#		var nodedata = thexml.get_node_data()
#		var nodetype = thexml.get_node_type()
#
#		#print("Nodename: " + nodename)
#		#print("Nodedata: " + nodedata)
#		#print(nodetype)
#
#		if nodename == 'Value':
#			print(thexml.get_named_attribute_value("Name"))
#			#print(nodetype)

		
		

	
	set_cell_item(5, 10, 5, 0)

