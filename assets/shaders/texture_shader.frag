#version 330 core
out vec4 FragColor;
uniform vec3 color;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;


in VS_OUTPUT {
   vec3 FragPos;
   flat vec3 Normal;
} IN;

void main()
{
  vec3 col = color;

  // ABIENT
  float ambientStrength = 0.5;
  vec3 ambient = ambientStrength * lightColor;


  //DIFFUSE
  vec3 norm = normalize(IN.Normal);
  vec3 lightDir = normalize(lightPos - IN.FragPos);
  float diff = max(dot(norm, lightDir), 0.0);

  vec3 diffuse = (diff * lightColor) * 0.70;


  // SPECULAR
  float specularStrength = 0.1;
  vec3 viewDir = normalize(viewPos - IN.FragPos);
  vec3 reflectionDir = reflect(-lightDir, IN.Normal);

  float spec = pow(max(dot(viewDir, reflectionDir), 0.0), 10);
  vec3 specular = specularStrength * spec * lightColor;

  FragColor = vec4( (ambient + diffuse + specular) * col, 1.0f);
  FragColor = vec4( col, 1.0f);

}
