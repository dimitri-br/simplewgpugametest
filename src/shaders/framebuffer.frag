#version 450
#extension GL_GOOGLE_include_directive : require
#include "base_frag.glsl"
#include "base_uniforms.glsl"
#include "image_tools.glsl"
#include "tonemapping.glsl"
#include "film_grain.glsl"
#include "light.glsl"

// Base Color Texture
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

// HDR Texture (Used for bloom)
layout(set = 1, binding = 0) uniform texture2D hdr_t_diffuse;
layout(set = 1, binding = 1) uniform sampler hdr_s_diffuse;

// Base Color Texture
layout(set = 3, binding = 0) uniform texture1D shadow_t_diffuse;
layout(set = 3, binding = 1) uniform sampler shadow_s_diffuse;

void main()
{
    const float gamma = 2.2f;
    const float exposure = 0.075f;

    vec3 hdrColor = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb;
    vec3 bloomColor = texture(sampler2D(hdr_t_diffuse, hdr_s_diffuse), v_tex_coords).rgb;

    /* Image Effects */

    // Apply the bloom hdr effect by additive
    hdrColor += bloomColor;
    vec4 result = vec4(vec3(1.0) - exp(-hdrColor * exposure), 1.0);

    vec4 light_result = apply_shadows(vec4(vec3(0.1, 0.9, 0.3), 1.0), shadow_t_diffuse, shadow_s_diffuse, v_tex_coords);
    result *= light_result;
    // Effects
    result += film_grain(0.0015, v_tex_coords);
    result *= vignette(v_tex_coords, 512.0);


    /* Image Correction */

    

    result.rgb = adjustContrast(result.rgb, 0.025);
    
    result.rgb = adjustSaturation(result.rgb, 0.5);

    result.rgb = adjustExposure(result.rgb, 0.5);

    result.rgb *= chromaticAberration(t_diffuse, s_diffuse, v_tex_coords, 0.5);



    // Color Correct
    result.rgb = acesFilm(result.rgb);

    // Gamma Correction for HDR to LDR
    result = pow(result, vec4(1.0 / gamma));

    f_color = result;
}  