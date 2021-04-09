extends CanvasLayer
# Declare member variables here. Examples:
# var a = 2
# var b = "text"
const cc_url = "http://scresdir.appspot.com/resource"
func _ready():
	# Create an HTTP request node and connect its completion signal.
	var http_request = HTTPRequest.new()
	add_child(http_request)
	http_request.connect("request_completed", self, "_http_request_completed")
	
	# Perform a POST request. The URL below returns JSON as of writing.
	# Note: Don't make simultaneous requests using a single HTTPRequest node.
	# The snippet below is provided for reference only.
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

	# Will print the user agent string used by the HTTPRequest node (as recognized by httpbin.org).
	#print(response.headers["User-Agent"])
	print(response)
	print(response_code)
	print(headers)
	print(body)
