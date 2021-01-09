vec3 adjustBrightness(vec3 color, float value){
    return color + value;
}

vec3 adjustContrast(vec3 color, float value){
    return 0.5 + (1.0 + value) * (color - 0.5);
}

vec3 adjustExposure(vec3 color, float value){
    return (1.0 + value) * color;
}

vec3 adjustSaturation(vec3 color, float value){
    const vec3 luminosityFactor = vec3(0.2126, 0.7152, 0.0722);
    vec3 greyscale = vec3(dot(color, luminosityFactor));

    return mix(greyscale, color, 1.0 + value);
}

vec3 chromaticAberration(texture2D tex, sampler samp, vec2 uv, float intensity){
	float amount = intensity;
	amount = pow(amount, 3.0);

	amount *= 0.05;
	
    vec3 col;
    col.r = texture( sampler2D(tex, samp), vec2(uv.x+amount,uv.y) ).r;
    col.g = texture( sampler2D(tex, samp), uv ).g;
    col.b = texture( sampler2D(tex, samp), vec2(uv.x-amount,uv.y) ).b;

	col *= (1.0 - amount * 0.5);

    return col;
}

vec4 vignette(vec2 uv, float intensity){  
    uv *=  1.0 - uv.yx;   //vec2(1.0)- uv.yx; -> 1.-u.yx; Thanks FabriceNeyret !
    
    float vig = uv.x*uv.y * intensity; // multiply with sth for intensity
    
    vig = pow(vig, 0.25); // change pow for modifying the extend of the  vignette

    return vec4(vig);
}