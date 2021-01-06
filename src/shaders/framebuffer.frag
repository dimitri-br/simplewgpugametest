#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"
#include "base_uniforms.glsl"


layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 1, binding = 0) uniform texture2D hdr_t_diffuse;
layout(set = 1, binding = 1) uniform sampler hdr_s_diffuse;

// Feel free to steal this :^)
// Consider it MIT licensed, you can link to this page if you want to.

#define SHOW_NOISE 0
#define SRGB 0
// 0: Addition, 1: Screen, 2: Overlay, 3: Soft Light, 4: Lighten-Only
#define BLEND_MODE 1
#define SPEED 2.0
#define INTENSITY 0.001
// What gray level noise should tend to.
#define MEAN 0.0
// Controls the contrast/variance of noise.
#define VARIANCE 0.5

vec3 channel_mix(vec3 a, vec3 b, vec3 w) {
    return vec3(mix(a.r, b.r, w.r), mix(a.g, b.g, w.g), mix(a.b, b.b, w.b));
}

float gaussian(float z, float u, float o) {
    return (1.0 / (o * sqrt(2.0 * 3.1415))) * exp(-(((z - u) * (z - u)) / (2.0 * (o * o))));
}

vec3 madd(vec3 a, vec3 b, float w) {
    return a + a * b * w;
}

vec3 screen(vec3 a, vec3 b, float w) {
    return mix(a, vec3(1.0) - (vec3(1.0) - a) * (vec3(1.0) - b), w);
}

vec3 overlay(vec3 a, vec3 b, float w) {
    return mix(a, channel_mix(
        2.0 * a * b,
        vec3(1.0) - 2.0 * (vec3(1.0) - a) * (vec3(1.0) - b),
        step(vec3(0.5), a)
    ), w);
}

vec3 soft_light(vec3 a, vec3 b, float w) {
    return mix(a, pow(a, pow(vec3(2.0), 2.0 * (vec3(0.5) - b))), w);
}

vec3 film_grain(vec2 coord) {
    vec2 ps = vec2(1.0);
    vec2 uv = coord * ps;
    vec3 color = texture(sampler2D(t_diffuse, s_diffuse), uv).rgb;
    #if SRGB
    color = pow(color, vec4(2.2));
    #endif
    
    float t = iTime * float(SPEED);
    float seed = dot(uv, vec2(12.9898, 78.233));
    float noise = fract(sin(seed) * 43758.5453 + t);
    noise = gaussian(noise, float(MEAN), float(VARIANCE) * float(VARIANCE));
    
    #if SHOW_NOISE
    color = vec3(noise);
    #else    
    vec3 grain = vec3(noise) * (1.0 - color.rgb);
    color = grain;
    #endif

    return color;
}


void main()
{
    const float gamma = sin(iTime) + 1.0;//2.2;
    vec3 grain = film_grain(v_tex_coords);
    vec3 hdrColor = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb;
    vec3 bloomColor = texture(sampler2D(hdr_t_diffuse, hdr_s_diffuse), v_tex_coords).rgb;
    hdrColor += bloomColor;
    // reinhard tone mapping
    vec3 result = hdrColor / (hdrColor + vec3(1.0));
    // gamma correction 
    result = pow(result, vec3(1.0 / gamma));

    float w = float(INTENSITY);
    #if BLEND_MODE == 0
    result += grain * w;
    #elif BLEND_MODE == 1
    result = screen(result, grain, w);
    #elif BLEND_MODE == 2
    result = overlay(result, grain, w);
    #elif BLEND_MODE == 3
    result = soft_light(result, grain, w);
    #elif BLEND_MODE == 4
    result = max(result, grain * w);
    #endif
        
    #if SRGB
    result = pow(result, vec4(1.0 / 2.2));
    #endif

    f_color = vec4(result, 1.0);
}  