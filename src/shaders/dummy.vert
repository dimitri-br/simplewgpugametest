#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_vertex.glsl"

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 1.0);
}