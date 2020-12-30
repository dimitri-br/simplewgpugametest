#version 450
#extension GL_ARB_separate_shader_objects : enable
layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

layout(set=1, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};
layout(set=3, binding=0) 
uniform Transform {
    mat4 transform;
};

void main() {
    v_tex_coords = a_tex_coords;
    gl_Position = u_view_proj * transform * vec4(a_position, 1.0);
}