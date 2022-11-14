#version 330 core
layout (location = 0) in vec2 aPos;


out VS_OUTPUT {
  vec2 FragPos;
} OUT;


void main()
{
  OUT.FragPos = aPos;
  vec4 pos = vec4(aPos, 0.0, 1.0);
  gl_Position =  pos;
}
