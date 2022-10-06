#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;


out VS_OUTPUT {
  vec3 FragPos;
  vec3 Normal;
  vec4 FragPosLightSpace;
  vec3 Apos;
} OUT;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 lightSpaceMat;

void main()
{
  vec4 pos = vec4(aPos, 1.0);

  OUT.Normal = transpose(inverse(mat3(model))) * aNormal;
  OUT.FragPos = vec3(model * pos);
  OUT.FragPosLightSpace = lightSpaceMat * vec4(OUT.FragPos, 1.0);
  OUT.Apos = aPos;

  gl_Position = projection * view * model * pos;
}
