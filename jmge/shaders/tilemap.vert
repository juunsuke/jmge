#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 TexCoord;

out VS_OUTPUT {
	vec4 Color;
	vec2 TexCoord;
} OUT;

uniform mat4 Projection;
uniform mat4 Transform;

void main()
{
	gl_Position = Projection * Transform * vec4(Position, 0.0, 1.0);
	OUT.Color = Color;
	OUT.TexCoord = TexCoord;
}
