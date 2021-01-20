#version 450
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 1, binding = 0) uniform BloomUniforms {
	bool horizontal;
};


const float blurScale = 15.0f;
const float blurStrength = 15.0f;

void main()
{             
	float weight[5];
	weight[0] = 0.227027;
	weight[1] = 0.1945946;
	weight[2] = 0.1216216;
	weight[3] = 0.054054;
	weight[4] = 0.016216;

     vec2 tex_offset = 1.0 / textureSize(sampler2D(t_diffuse, s_diffuse), 0) * blurScale; // gets size of single texel
     vec3 result = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb * weight[0];
     if(horizontal)
     {
         for(int i = 1; i < 5; ++i)
         {
            result += texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + vec2(tex_offset.x * i, 0.0)).rgb * weight[i] * blurStrength;
            result += texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords - vec2(tex_offset.x * i, 0.0)).rgb * weight[i] * blurStrength;
         }
     }
     else
     {
         for(int i = 1; i < 5; ++i)
         {
             result += texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + vec2(0.0, tex_offset.y * i)).rgb * weight[i] * blurStrength;
             result += texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords - vec2(0.0, tex_offset.y * i)).rgb * weight[i] * blurStrength;
         }
     }
     f_color = vec4(result, 1.0);
}