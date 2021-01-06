#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"

layout(location=1) out vec4 h_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;


void main() {
    vec4 texture = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    f_color = vec4(vec3(1) - texture.rgb, 1.0);
    h_color = vec4(f_color.rgb, 1.0);
}