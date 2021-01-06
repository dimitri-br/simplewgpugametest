#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_vertex.glsl"

layout(set=1, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};

layout(set=3, binding=0) 
uniform Transform {
    mat4 transform;
};

void main() {
    v_tex_coords = tex_coords;
    gl_Position = u_view_proj * transform * vec4(position, 1.0);
}