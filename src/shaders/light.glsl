#define PI 3.14


//sample_shadow from the 1D distance map
float sample_shadow(texture1D tex, sampler samp, float coord, float r) {

	return step(r, texture(sampler1D(tex, samp), coord).r);

}

vec4 apply_shadows(vec4 light_color, texture1D tex, sampler samp, vec2 uv) {
    vec2 resolution = vec2(256);
	//rectangular to polar
	vec2 norm = uv.st * 2.0 - 1.0;
	float theta = atan(norm.y, norm.x);
	float r = length(norm);	
	float coord = (theta + PI) / (2.0*PI);
	
	//the center tex coord, which gives us hard shadows
	float center = sample_shadow(tex, samp, coord, r);        
	
	//we multiply the blur amount by our distance from center
	//this leads to more blurriness as the shadow "fades away"
	float blur = (1./resolution.x)  * smoothstep(0., 1., r); 
	
	//now we use a simple gaussian blur
	float sum = 0.0;
	
	sum += sample_shadow(tex, samp, coord - 4.0*blur, r) * 0.05;
	sum += sample_shadow(tex, samp, coord - 3.0*blur, r) * 0.09;
	sum += sample_shadow(tex, samp, coord - 2.0*blur, r) * 0.12;
	sum += sample_shadow(tex, samp, coord - 1.0*blur, r) * 0.15;
	
	sum += center * 0.16;
	
	sum += sample_shadow(tex, samp, coord + 1.0*blur, r) * 0.15;
	sum += sample_shadow(tex, samp, coord + 2.0*blur, r) * 0.12;
	sum += sample_shadow(tex, samp, coord + 3.0*blur, r) * 0.09;
	sum += sample_shadow(tex, samp, coord + 4.0*blur, r) * 0.05;
	
	//sum of 1.0 -> in light, 0.0 -> in shadow
 	float lit = mix(center, sum, 1.0);
 	//multiply the summed amount by our distance, which gives us a radial falloff
 	//then multiply by vertex (light) color 
    vec4 result = light_color * vec4(vec3(1.0), lit * smoothstep(1.0, 0.0, r)); 
 	return result;
}