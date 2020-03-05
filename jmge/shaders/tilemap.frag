
#version 330 core

in VS_OUTPUT {
	vec4 Color;
	vec2 TexCoord;
} IN;

uniform sampler2D tex;

out vec4 Color;

void main()
{
	Color = texture2D(tex, IN.TexCoord) * IN.Color;
	//Color = IN.Color;
}

