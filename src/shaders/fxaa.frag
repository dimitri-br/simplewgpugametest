#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_uniforms.glsl"
#include "fxa.glsl"
#include "base_frag.glsl"

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main(void)
{
	f_color = applyFXAA(gl_FragCoord.xy, t_diffuse, s_diffuse);
}