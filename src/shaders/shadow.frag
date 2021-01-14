#version 450
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"

#define PI 3.14


// Base Color Texture
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform samplerShadow s_diffuse;

//alpha threshold for our occlusion map
const float THRESHOLD = 0.75;

void main() {
  float distance = 1.0;
  vec2 resolution = vec2(256);

  for (float y=0.0; y<resolution.y; y+=1.0) {
  		//rectangular to polar filter
		vec2 norm = vec2(v_tex_coords.s, y/resolution.y) * 2.0 - 1.0;
		float theta = PI*1.5 + norm.x * PI; 
		float r = (1.0 + norm.y) * 0.5;
		
		//coord which we will sample from occlude map
		vec3 coord = vec3(vec2(-r * sin(theta), -r * cos(theta))/2.0 + 0.5, 0);
		
		//sample the depth map
        float near = 0.1;
        float far = 25.0;
		float depth = texture(sampler2DShadow(t_diffuse, s_diffuse), coord);
		//depth = 2.0 * depth - 1.0;
        //depth = (2.0 * near) / (far + near - depth * (far - near));

		//the current distance is how far from the top we've come
		float dst = y/resolution.y;
		
		//if we've hit an opaque fragment (occluder), then get new distance
		//if the new distance is below the current, then we'll use that for our ray
		float caster = depth;
		if (caster > THRESHOLD) {
			distance = min(distance, dst);
			//NOTE: we could probably use "break" or "return" here
  		}
  } 
  f_color = vec4(vec3(distance), 1.0);
}