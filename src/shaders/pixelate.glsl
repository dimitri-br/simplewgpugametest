vec3 pixelate(float amount, texture2D tex, sampler samp, vec2 vertTexCoord) {

    vec2 resolution = iResolution;

    float d = 1.0 / amount;
	float ar = resolution.x / resolution.y;
	float u = floor(vertTexCoord.x / d) * d;

	d = ar / amount;

	float v = floor(vertTexCoord.y / d) * d;

    vec3 color = texture(sampler2D(tex, samp), vec2(u, v)).rgb;
	return color;
}