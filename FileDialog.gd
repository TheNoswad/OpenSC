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
