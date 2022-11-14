#version 330 core
out vec4 FragColor;
uniform vec3 color;

in VS_OUTPUT {
  vec3 FragPos;
  vec3 Normal;
} IN;


// return > 0 to keep
float sdf(float x, float y) {


  float r = sqrt(x*x + y*y);

  if (r < 0.5) {
      return 0.0 ;
  }


  if ( r > 1.0 ) {
    return 0.0;
  }

  return 1.0;

}


void main()
{


  float x = IN.FragPos.x * 2;
  float y = IN.FragPos.y * 2;
  vec3 col = color;


  float keep = sdf(x, y);

  if (keep <= 0.0) {
    discard;
  }

  FragColor = vec4(col, 1.0);

}
