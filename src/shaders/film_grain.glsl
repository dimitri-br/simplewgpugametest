// Feel free to steal this :^)
// Consider it MIT licensed, you can link to this page if you want to.

#define SHOW_NOISE 0
#define SRGB 0
// 0: Addition, 1: Screen, 2: Overlay, 3: Soft Light, 4: Lighten-Only
#define BLEND_MODE 1
#define SPEED 2.5
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

vec4 film_grain(float intensity, vec2 texcoord) {
    vec2 ps = vec2(1.0);
    vec2 uv = texcoord * ps;
    vec4 color = vec4(0);
    
    float t = iTime * float(SPEED);
    float seed = dot(uv, vec2(12.9898, 78.233));
    float noise = fract(sin(seed) * 43758.5453 + t);
    noise = gaussian(noise, float(MEAN), float(VARIANCE) * float(VARIANCE));
    
    #if SHOW_NOISE
    color = vec4(noise);
    #else    
    // Ignore these mouse stuff if you're porting this
    // and just use an arbitrary intensity value.
    float w = intensity;

    vec3 grain = vec3(noise) * (1.0 - color.rgb);
    
    #if BLEND_MODE == 0
    color.rgb += grain * w;
    #elif BLEND_MODE == 1
    color.rgb = screen(color.rgb, grain, w);
    #elif BLEND_MODE == 2
    color.rgb = overlay(color.rgb, grain, w);
    #elif BLEND_MODE == 3
    color.rgb = soft_light(color.rgb, grain, w);
    #elif BLEND_MODE == 4
    color.rgb = max(color.rgb, grain * w);
    #endif
        
    #if SRGB
    color = pow(color, vec4(1.0 / 2.2));
    #endif
    #endif
    
    return color;
}