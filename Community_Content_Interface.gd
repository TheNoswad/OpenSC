extends CanvasLayer

# Declare member variables here. Examples:
#const cc_url = "https://scresdir.appspot.com/resource" # The community content URL
const cc_url = "https://cors.bridged.cc/https://scresdir.appspot.com/resource" # The community content URL
var items = {}
var NextCursor = null
var requestwait = false

func request_cc_data(Action, NextCursor):
	if requestwait == true:
		print("request wait")
	else:
		requestwait = true
		
		var http_request = HTTPRequest.new()
		add_child(http_request)
		http_request.connect("request_completed", self, "_http_request_completed")
		
		# Perform a POST request.
		# Note: Don't make simultaneous requests using a single HTTPRequest node.
		
		var body = "Action=" + str(Action)
		
		if NextCursor != null:
			body = body + "&" + "Cursor=" + str(NextCursor)
		
		#var body = "Action=list&UserId=Kaalus"
		#print(body)
		var headers = ["Content-Type: application/x-www-form-urlencoded", "X-Requested-With: XMLHttpRequest"]
		
		var error = http_request.request(cc_url, headers, false, HTTPClient.METHOD_POST, body)
		if error != OK:
			push_error("An error occurred in the HTTP request.")

func _ready():
#	var end_detector_gen = preload("res://ListEndDetector.tscn").instance()
#	$ScrollContainer/VBoxContainer.add_child(end_detector_gen)
	get_tree().get_root().connect("size_changed", self, "resize_update")
	request_cc_data("list", NextCursor)

func _http_request_completed(result, response_code, headers, body):
	#var response = parse_json(body.get_string_from_utf8())
	var response = body.get_string_from_utf8()
	#print("Response Code = " + str(response))
	print("Headers = " + str(headers))
	
	var thexml = XMLParser.new()
	var the_cc_response_dictionary = {}
	thexml.open_buffer(body)
	var index = 0
	while thexml.read() == OK:
		# Dead code
#		var nodename = thexml.get_node_name()
#		var nodedata = thexml.get_node_data()
#		var nodetype = thexml.get_node_type()
		
		# Gets the next cursor. Only in the opening "Results" tag
		if thexml.get_node_name() == 'Results':
			if thexml.get_attribute_count() >= 1:
				print("Got NextCursor")
				#print(thexml.get_attribute_value(0))
				NextCursor = thexml.get_attribute_value(0)
		else:
			
			# Parses and stores the line as a dictionary entry
			the_cc_response_dictionary[index] = {
			Name = thexml.get_attribute_value(0),
			Url = thexml.get_attribute_value(1),
			Type = thexml.get_attribute_value(2),
			Size = thexml.get_attribute_value(3),
			RatingsAverage = thexml.get_attribute_value(4),
			UserId = thexml.get_attribute_value(5),
			ExtraText = thexml.get_attribute_value(6),
			}
			
			index += 1
	
	add_cc_items_to_list(the_cc_response_dictionary, the_cc_response_dictionary.size())
	requestwait = false

func add_cc_items_to_list(cclist_dict_thingie, count_of_items):
	for i in count_of_items:
#		var button1 = Button.new()
#		button1.text = str(cclist_dict_thingie[i]["Name"])
#		button1.show()
#		items[i] = button1
		
		var buttoninstance = preload("res://CC_Item_Button.tscn").instance()
		
		buttoninstance.Name = cclist_dict_thingie[i]["Name"]
		buttoninstance.Url = cclist_dict_thingie[i]["Url"]
		buttoninstance.Type = cclist_dict_thingie[i]["Type"]
		buttoninstance.Size = cclist_dict_thingie[i]["Size"]
		buttoninstance.RatingsAverage = cclist_dict_thingie[i]["RatingsAverage"]
		buttoninstance.ExtraText = cclist_dict_thingie[i]["ExtraText"]
		
		buttoninstance.init()
		$ScrollContainer/VBoxContainer.add_child(buttoninstance)
		$ScrollContainer/VBoxContainer.move_child(get_node("ScrollContainer/VBoxContainer/EndDetect"), $ScrollContainer/VBoxContainer.get_child_count())

func _process(_delta):
	pass
#	print("Vsbr pos = " + str($ScrollContainer.get_v_scrollbar().value))
#	print("Vsbr max = " + str($ScrollContainer.get_v_scrollbar().max_value))
#	print("Page = " + str($ScrollContainer.get_v_scrollbar().page))
	
#	if $ScrollContainer.get_v_scrollbar().ratio >= 0.4:
#		print("past scroll")
#		request_cc_data("list", NextCursor)
		


func _on_VisibilityNotifier2D_viewport_entered(viewport):
	request_cc_data("list", NextCursor)
	

func resize_update():
	print("Resizing: ", get_viewport().size)
	$ScrollContainer.rect_size(get_viewport().size.x, get_viewport().size.y)
	if get_viewport().size.y >= get_viewport().size.x:
		#self.columns = 1
		pass
	else:
		pass
		#self.columns = 2

