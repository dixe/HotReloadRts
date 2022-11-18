#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aBoneWeights;
layout (location = 3) in vec2 aBoneIndices;

uniform mat4 uBones[32];


out VS_OUTPUT {
  vec3 FragPos;
  vec3 Normal;
  vec4 FragPosLightSpace;
  vec3 Color;
} OUT;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 lightSpaceMat;
uniform vec3 color;

mat4 boneTransform() {

  if(int(aBoneIndices.x) < 0)
  {
    return mat4(1.0);
  }
  mat4 ret;

  // Weight1 * Bone1 + Weight2 * Bone2
  ret = aBoneWeights.x * uBones[int(aBoneIndices.x)]
    + aBoneWeights.y * uBones[int(aBoneIndices.y)];

  return ret;

}


void main()
{
  mat4 bt = boneTransform();

  int b = 16;

  if ( int(aBoneIndices.x) == b || int(aBoneIndices.y) == b)
  {
    OUT.Color = vec3(0.0, 0.0, 0.0);
  }

  vec4 pos = model * bt * vec4(aPos, 1.0);

  OUT.FragPos = vec3(model * bt * vec4(aPos, 1.0));
  OUT.Normal = mat3(transpose(inverse(model * bt))) * aNormal;
  OUT.FragPosLightSpace = lightSpaceMat * vec4(OUT.FragPos, 1.0);
  OUT.Color = color;

  gl_Position =  projection * view  * pos;

}
