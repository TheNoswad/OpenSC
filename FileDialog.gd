extends FileDialog


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass



# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Button_button_down():
	popup()


func _on_FileDialog_file_selected(path):
	Gdc.selected_file_path = path
	print("selected_file_path = " + Gdc.selected_file_path)
