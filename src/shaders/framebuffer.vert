#version 450
/*
    Render a triangle based on gl_VertexIndex instead of a buffer to reduce overdraw and speed up performance a bit

    https://github.com/SaschaWillems/Vulkan/blob/master/data/shaders/glsl/bloom/gaussblur.vert -> Used Sascha Willems Examples to help with this
*/
layout (location = 0) out vec2 outUV;
layout (location = 1) out vec3 frag_pos;

out gl_PerVertex
{
	vec4 gl_Position;
};

void main() 
{
    mat4 OPENGL_TO_WGPU_MATRIX = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0
    );
	outUV = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
	gl_Position = OPENGL_TO_WGPU_MATRIX * vec4(outUV * 2.0f - 1.0f, 0.0f, 1.0f);
    outUV = vec2((gl_VertexIndex << 1) & 2, 1.0f - (gl_VertexIndex & 2));
    frag_pos = gl_Position.xyz;
}
