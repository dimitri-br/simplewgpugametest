#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_vertex.glsl"

layout(set=1, binding=0) 
uniform Uniforms {
    mat4 proj;
    mat4 view;
};

layout(set=3, binding=0) 
uniform Transform {
    mat4 transform;
};

void main() {
    v_tex_coords = tex_coords;
    gl_Position = proj * view * transform * vec4(position, 1.0);

    frag_pos = vec3(transform * vec4(position, 1.0));
}