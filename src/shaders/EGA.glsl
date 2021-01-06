vec4 ega_style(texture2D tex, sampler samp, vec2 texcoord){
    	vec2 uv = texcoord;
    uv=vec2(floor(uv.x*320.)/320.,floor(uv.y*240.)/240.);
    vec4 samplrr = texture(sampler2D(tex, samp), uv);
    vec4 samplrr2 = texture(sampler2D(tex, samp), uv+vec2(1./ 320.0,0.0));
    vec4 samplrr3 = texture(sampler2D(tex, samp), uv+vec2(0.0,1./ 240.0));
    vec4 samplrr4 = texture(sampler2D(tex, samp), uv+vec2(-1./ 320.0,0.0));
    vec4 samplrr5 = texture(sampler2D(tex, samp), uv+vec2(0.0,-1./ 240.0));
    
    float I=floor(length(samplrr.rgb)+0.5)*.5+1.2;
    vec3 C=vec3(
        		floor(samplrr.r*3.)/3.*I,
        		floor(samplrr.g*3.)/3.*I,
        		floor(samplrr.b*3.)/3.*I
    			);
    float border = floor(distance(samplrr2,samplrr)+distance(samplrr3,samplrr)+distance(samplrr4,samplrr)+distance(samplrr5,samplrr)+0.73);
    uv.x*=0.6+sin(uv.y/7.+iTime)/3.;
    uv.y*=0.3+sin(uv.x+iTime)/5.;
    vec3 effect = vec3(0.0);
    effect.r=sin(sin(uv.x*2.+iTime)+uv.y*10.+2.*iTime+sin(iTime)*2.)*.5+.5;
    effect.g=sin(sin(uv.x*5.+iTime)+uv.y*70.+iTime+sin(iTime/8.)*2.)*.5+.5;
    effect.b=sin(sin(uv.x*8.+iTime)+uv.y*100.+iTime+sin(iTime/3.)*2.)*.5+.5;
    float Ieffect=floor(length(effect.rgb)+0.5)*.5+1.2;
    vec3 Ceffect=vec3(
        		floor(effect.r*3.)/3.*I,
        		floor(effect.g*3.)/3.*I,
        		floor(effect.b*3.)/3.*I
    			);
    vec3 finalColor=vec3(0.);
    
    //laazyy
    if(C.g > 0.5 && C.r<0.5 && C.b<0.5) //laazyy
        finalColor = Ceffect*(1.-vec3(border)); //laazyy
    else { //laazyy
        finalColor = C*(1.-vec3(border)); //laazyy
    } //laazyy
    
	return vec4(finalColor, 1);
}