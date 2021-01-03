#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;
layout(location=1) out vec4 h_color;
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set=2, binding=0)
uniform Material{
    float shininess;
    float metallic;
};

void main() {

    vec4 texture = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    f_color = texture;

    
    float brightness = dot(f_color.rgb, vec3(0.2126, 0.7152, 0.0722));
    if(brightness > 1.0)
        h_color = vec4(f_color.rgb, 1.0);
    else
        h_color = vec4(0.0, 0.0, 0.0, 1.0);
    h_color = vec4(f_color.rgb, 1.0);
}