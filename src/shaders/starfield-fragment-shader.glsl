#version 100

precision highp float;
varying vec4 color;
varying vec2 uv;
varying float iTime;
varying vec2 iMouse;
uniform vec2 iResolution;
uniform float direction_modifier;

#define PI 3.14159
#define NUM_LAYERS 9.

// rotate will take an angle and rotate it using:
// [cos -sin]
// [sin  cos]
mat2 rotate(float angle)
{
    float sine = sin(angle);
    float cosine = cos(angle);
    return mat2(cosine, -sine, sine, cosine);
}

// hash will take in 2 numbers as a vec2 and return
// a pseudo-random number with some funky math.
float hash(vec2 numbers)
{
    numbers = fract(numbers * vec2(123.34, 456.21));
    numbers += dot(numbers, numbers + 45.32);
    return fract(numbers.x * numbers.y);
}

// getStar will draw a star given the provided radius
// and luminosity.
float getStar(vec2 position, float radius, float luminosity)
{
    // the star will emit a glow from it's position to it's radius
    float star = radius / length(position);
    
    // the rays extend past the star's position, but must not surpass
    // the local bounds of the star
    float rays = max(0., 1. - abs(position.x * position.y * 1500.));
    star += rays * luminosity;
    
    // draw a second set of rays rotated at a 45 degree angle
    position *= rotate(PI/4.);
    rays = max(0., 1. - abs(position.x * position.y * 2000.));
    star += rays * luminosity * .3;
    
    // fade out the glow of the star so it doesn't exceed it's bounds
    star *= smoothstep(1., .2, length(position));
    
    return star;
}

// getStarLayer will draw a bunch of stars as a layer.
vec3 getStarLayer(vec2 center)
{
    // start with a black canvas
    vec3 color = vec3(0);
    
    // break the layer up into the fractional component of the center
    vec2 grid = fract(center) - 0.5;
    
    // take the rounded-down int of the center as a unique id for each star
    vec2 id = floor(center);
    
    // loop between all neighboring stars over the grid
    for(int y = -1; y <= 1; y++)
    {
        for (int x = -1; x <= 1; x++)
        {
            // make this star aware of where it is among the neighbors
            vec2 offset = vec2(x, y);
            
            // get randomized values for each star
            float randomNumber = hash(id + offset);
            float randomSize = fract(randomNumber * 223.6);
            float randomLuminosity = smoothstep(.97, 1., randomSize) * .4;
            vec3 randomColor = sin(vec3(.2, .3, .9) * fract(randomNumber * 1122.4) * PI) * .5 + .5;
            
            // get the star
            float star = getStar(
                grid - offset - vec2(randomNumber, fract(randomNumber * 34144.)) + .5,
                0.04 * randomSize,
                randomLuminosity
            );
            
            // twinkle, twinkle, little star
            star *= sin(iTime * 3. + randomNumber * PI * 2.) * .1 + .7;
            
            // add the star to the total color
            color += star * (randomColor * vec3(0.95, 0.8, 0.6));
        }
    }
    
    return color;
}

// main is the entry point of the shader.
void main()
{
    // get a relative speed of movement based on the shader timer
    float time = iTime * .03;
    
    // get the relative position of the mouse
    vec2 mouse = (iMouse.xy - iResolution.xy * .5) / iResolution.y;
    
    // get the center of the total area
    vec2 center = (gl_FragCoord.xy - 0.5 * iResolution.xy) / iResolution.y;
    
    // rotate the canvas based on relative mouse position and speed
    center += mouse * 2.;
    center *= rotate(time * 0.3);
    
    // start with a black canvas
    vec3 color = vec3(0);
    
    // draw star layers
    for (float i = 0.; i < 1.; i += 1./NUM_LAYERS)
    {
        float depth = fract(i + time);
        float scale = mix(20., .5, depth);
        float fade = depth * smoothstep(1., .97, depth);
        color += getStarLayer(center * scale + i * 435.2) * fade;
    }
    
    // set the output to the star layer
    gl_FragColor = vec4(color, 1.0);
}