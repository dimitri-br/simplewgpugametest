#version 450
layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 1, binding = 0) uniform texture2D hdr_t_diffuse;
layout(set = 1, binding = 1) uniform sampler hdr_s_diffuse;

void main()
{
    const float gamma = 2.2;
    vec3 hdrColor = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb;
    vec3 bloomColor = texture(sampler2D(hdr_t_diffuse, hdr_s_diffuse), v_tex_coords).rgb;
    hdrColor += bloomColor;
    // reinhard tone mapping
    vec3 result = hdrColor / (hdrColor + vec3(1.0));
    // gamma correction 
    result = pow(result, vec3(1.0 / gamma));


    f_color = vec4(result, 1.0);
}  