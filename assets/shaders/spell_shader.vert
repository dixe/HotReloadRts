#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;


out VS_OUTPUT {
  vec3 FragPos;
  vec3 Normal;
} OUT;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
  vec4 pos = vec4(aPos, 1.0);

  OUT.Normal = transpose(inverse(mat3(model))) * aNormal;
  OUT.FragPos = pos.xyz;

  gl_Position = projection * view * model * pos;
}
