#version 460

layout(location = 0) out vec4 out_colour;
layout(location = 0) in vec2 in_pos;

void main() {
	out_colour = vec4(in_pos, 0, 1);
}
