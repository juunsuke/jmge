#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 TexCoord;
layout (location = 3) in vec2 Translate;
layout (location = 4) in vec2 Scale;
layout (location = 5) in float Angle;
layout (location = 6) in vec2 Origin;

out VS_OUTPUT {
	vec4 Color;
	vec2 TexCoord;
} OUT;

uniform mat4 Projection;

mat4 translate(in vec2 v)
{
	return mat4(
		1, 0, 0, 0,
		0, 1, 0, 0,
		0, 0, 1, 0,
		v.x, v.y, 0, 1
	);
}

mat4 rotate(in float angle)
{
	return mat4(
		cos(angle),		sin(angle),			0,			0,
		-sin(angle),	cos(angle),			0,			0,
		0,				0,					1,			0,
		0,				0,					0,			1
	);
}

mat4 scale(in vec2 v)
{
	return mat4(
		v.x,	0,		0,		0,
		0,		v.y,	0,		0,
		0,		0,		1,		0,
		0,		0,		0,		1
	);
}

void main()
{
	gl_Position = Projection * translate(Translate) * rotate(Angle) * scale(Scale) * translate(Origin) * vec4(Position, 0.0, 1.0);
	OUT.Color = Color;
	OUT.TexCoord = TexCoord;
}
