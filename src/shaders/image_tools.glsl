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