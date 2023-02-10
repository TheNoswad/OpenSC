#version 330
precision mediump float;
in vec2 position;
out vec4 color;
uniform float blue;
void main() {
  color = vec4(position, blue, 1.0);
}