#version 330 core
layout (location = 0) in vec3 aPos;

out VS_OUTPUT {
   vec3 FragPos;
   flat vec3 Normal;
} OUT;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
   vec4 pos = vec4(aPos, 1.0);
   OUT.Normal = aPos; //  pos is normal since mesh is unit sphere


    OUT.FragPos = vec3(model * pos);
    gl_Position =  projection * view * model * pos;
}
