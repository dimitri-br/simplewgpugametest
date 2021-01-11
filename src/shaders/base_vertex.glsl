#ifndef BASE_VERTEX_GLSL
#define BASE_VERTEX_GLSL

// Bare minimum for a vertex shader
layout(location=0) in vec3 position;
layout(location=1) in vec2 tex_coords;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 frag_pos;

#endif