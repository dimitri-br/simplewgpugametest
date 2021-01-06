#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_vertex.glsl"

const mat4 opengl_to_wgpu = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0
);

void main() {
    v_tex_coords = tex_coords;
    gl_Position = opengl_to_wgpu * vec4(position, 1.0);
}