extends Control


# Declare member variables here. Examples:
var Name = null
var Url = null
var Type = null
var Size = null
var RatingsAverage = null
var ExtraText = null

var pressed = false

# Called when the node enters the scene tree for the first time.
func _ready():
	pass
	


func init():
	get_node("PanelContainer/VBoxContainer/Name").text = Name
	get_node("PanelContainer/VBoxContainer/HBoxContainer/Type").text = Type
	get_node("PanelContainer/VBoxContainer/HBoxContainer/Size").text = Size
	get_node("PanelContainer/VBoxContainer/HBoxContainer/Stars").text = RatingsAverage
	get_node("PanelContainer/VBoxContainer/HBoxContainer/ExtraText").text = ExtraText

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_PanelContainer_gui_input(event):
	if event is InputEventMouseMotion:
		pressed = false
	else:
		if event is InputEventMouse:
			if event.is_pressed() == true:
				pressed = true
			else:
				if pressed == true:
					pressed = false
					print(Url)
					OS.shell_open(Url)
#	if event.type == InputEvent.MOUSE_BUTTON \
#	and event.button_index == BUTTON_LEFT \
#	and event.pressed:
#		print("Clicked")

