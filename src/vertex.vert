#version 330
in vec2 in_position;
out vec2 position;
void main() {
  position = in_position;
  gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
}