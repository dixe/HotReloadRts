#version 330 core
out vec4 FragColor;

in VS_INPUT {
  vec2 FragPos;
} IN;


uniform float health;
uniform float left;
uniform float right;
uniform float screen_w;


void main()
{

  float l = (left + 1.0) / 2.0;
  float r = (right + 1.0) / 2.0;

  float diff = r - l;
  float x = gl_FragCoord.x/screen_w;

  float scaled = (x - l) / diff;
  float g = max(1.0, 0.0);
  if (scaled > health) {
      g = 0.0;
  }

  FragColor = vec4(0.0, g, 0.0, 1.0);
}
