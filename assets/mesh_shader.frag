#version 330 core
out vec4 FragColor;
uniform vec3 color;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;


uniform sampler2D shadowMap;


in VS_OUTPUT {
  vec3 FragPos;
  vec3 Normal;
  vec4 FragPosLightSpace;
  vec3 Apos;
} IN;


float ShadowCalculation(vec4 fragPosLightSpace, vec3 normal, vec3 lightDir)
{
  // get correct projection, when using perspective and ortho
  // in [-1,1]
  vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;

  // map to [0,1]
  projCoords = projCoords * 0.5 + 0.5;

  float closestDepth = texture(shadowMap, projCoords.xy).r;
  float currentDepth = projCoords.z;
  float shadow = currentDepth  > closestDepth ? 1.0 : 0.0;

  // if normal points away from light, we now that it is in shadow
  // this can also eliminate the bias sicne that created notisable
  float angle = dot(normal, lightDir);
  if (angle < 0.0 ) {
    shadow = 1.0;
  }

  // light outside light view frustum z far
  if(projCoords.z > 1.0)
  {
    shadow = 0.0;
  }


  return shadow;
}

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

  float spec = pow(max(dot(viewDir, reflectionDir), 0.0), 5);
  vec3 specular = specularStrength * spec * lightColor;

  float shadow = ShadowCalculation(IN.FragPosLightSpace, norm, lightDir);

  FragColor = vec4((ambient +  (1.0 - shadow) * (diffuse + specular)) * col, 1.0f);
  //FragColor = vec4(vec3(norm), 1.0f);
}
