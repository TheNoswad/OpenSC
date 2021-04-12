extends CanvasLayer

# Declare member variables here. Examples:
const cc_url = "http://scresdir.appspot.com/resource" # The community content URL
var items = {}
var NextCursor = null

func _ready():
	# Create an HTTP request node and connect its completion signal.
	var http_request = HTTPRequest.new()
	add_child(http_request)
	http_request.connect("request_completed", self, "_http_request_completed")
	
	
	# Perform a POST request.
	# Note: Don't make simultaneous requests using a single HTTPRequest node.
	
	#var body = {"Action": "list"}
	var body = "Action=list"
	var headers = ["Content-Type: application/x-www-form-urlencoded"]
	
	var error = http_request.request(cc_url, headers, false, HTTPClient.METHOD_POST, body)
	if error != OK:
		push_error("An error occurred in the HTTP request.")

# Called when the HTTP request is completed.
func _http_request_completed(result, response_code, headers, body):
	#var response = parse_json(body.get_string_from_utf8())
	var response = body.get_string_from_utf8()
	
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

func add_cc_items_to_list(cclist_dict_thingie, count_of_items):
	for i in count_of_items:
		var button1 = Button.new()
		button1.text = str(cclist_dict_thingie[i]["Name"])
		button1.show()
		items[i] = button1
		var buttoninstance = preload("res://CC_Item_Button.tscn").instance().init(cclist_dict_thingie[i])
		#$ScrollContainer/VBoxContainer.add_child(items[i])
