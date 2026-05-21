#version 330 core
layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTexCoord;

uniform vec2 uOffset;
uniform vec2 uScale;

out vec2 vTexCoord;

void main()
{
    gl_Position = vec4(aPos * uScale + uOffset, 0.0, 1.0);
    vTexCoord = aTexCoord;
}