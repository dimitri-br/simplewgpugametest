#ifndef BASE_UNIFORMS_GLSL
#define BASE_UNIFORMS_GLSL

layout(set = 2, binding = 0) uniform BaseUniforms{
    vec2 iResolution;
    float iTime;
};

#endif