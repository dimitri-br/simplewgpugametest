#version 450
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"

const float MAXRADIUS = 65535., // Maximum ray-length of 2 bytes, 2^16-1.
	TAU = 6.2831853071795864769252867665590; // TAU or 2 * pi (shortcut for radial.circular math).

const float in_RayTexSize = 256;
// Base Color Texture
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform samplerShadow s_diffuse;

//alpha threshold for our occlusion map
const float THRESHOLD = 0.75;
const vec3 in_Light = vec3(1.0);


void main() {

    // Converts the current pixel's coordinate from UV to XY space.
	vec2 Coord = floor(v_tex_coords * in_RayTexSize);
    vec2 xyRay = vec2((Coord.y * in_RayTexSize) + Coord.x, TAU * in_Light.z);
	vec2 tex_size = vec2(32);


    // Takes the pixel's XY position, converts it to a vec2(1D-array index, ray count).
	xyRay = vec2((Coord.y * in_RayTexSize) + Coord.x, TAU * in_Light.z);
	// Takes the index/ray_count and converts it to an angle in range of: 0 to 2pi = 0 to ray_count.
	float Theta = TAU * (xyRay.x / xyRay.y);
	// Gets the lengthdir_xy polar cooridinate around the light's center.
	vec2 Delta = vec2(cos(Theta), -sin(Theta));
	// "Step" gets checks whether the current ray index < ray count, if not the ray is not traced (for-loop breaks).
	for(float v = step(xyRay.x,xyRay.y), d = 0.; d < MAXRADIUS * v; d++)
		/*
			"in_Light.z < d" Check if the current ray distance(length) "d" is > light radius (if so, then break).
			"d + in_Light.z * texture2D(...)" If collision in the world map at distance "d" is found, the ray ends
			(add light radius to d to make it greater than the light radius to break out of the for-loop.
		*/
		if (in_Light.z < d + in_Light.z * texture(sampler2D(t_diffuse, s_diffuse), (in_Light.xy + (xyRay = Delta * d)) * tex_size).a) break;
	// Converts the ray length to polar UV coordinates ray_length / light_radius.
	float rayLength = length(xyRay) / in_Light.z;
	// Takes the length of the current ray and splits it into two bytes and stores it in the texture.
	f_color = vec4(vec2(floor(rayLength * 255.0) / 255.0, fract(rayLength * 255.0)), 0.0, 1.0);
}