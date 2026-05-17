#version 330 core

layout(location = 0) in vec3 aPos;         // позиция вершины

//uniform mat4 uModel;
//uniform mat4 uView;
//uniform mat4 uProjection;

void main()
{
    gl_Position = vec4(aPos, 1.0);
}