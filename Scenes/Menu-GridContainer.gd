extends GridContainer


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	get_tree().get_root().connect("size_changed", self, "myfunc")

func myfunc():
	print("Resizing: ", get_viewport_rect().size)
	if get_viewport_rect().size.y >= get_viewport_rect().size.x:
		self.columns = 1
	else:
		self.columns = 2
