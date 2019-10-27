// This is a demo shader and will be removed in the future

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

// TODO: make uniform
vec2 resolution = vec2(800.0, 600.0);

float zoom_1   = 2.0;
vec2  center_1 = vec2(0.0, 0.0);
float zoom_2   = 0.05;
vec2  center_2 = vec2(0.37, 0.1);

float blend = 0.0;
float zoom   = mix(zoom_1,   zoom_2,   blend);
vec2  center = mix(center_1, center_2, blend);

void main() {
    // Make screen go from [-0.5, -0.5] in up-left to [0.5, 0.5] in low-right
    vec2 norm_coordinates = ( (gl_FragCoord.xy - 0.5) / resolution );
    norm_coordinates -= 0.5;

    // Fix aspect ratio
    norm_coordinates.x *= min( 1.0 , resolution.x / resolution.y);
    norm_coordinates.y *= min( 1.0 , resolution.y / resolution.x);

    norm_coordinates *= zoom;
    norm_coordinates += center;

    vec2 c = norm_coordinates;

    vec2 z = vec2(0.0, 0.0);
    float i;
    for (i = 0.0; i <= 1.0; i += 0.001) {
        z = vec2(
            z.x * z.x - z.y * z.y + c.x,
            z.y * z.x + z.x * z.y + c.y
        );

        if (length(z) > 4.0) {
            break;
        }
    }
    
    i = pow(i, 0.5);
    //i = pow(i, i);

    vec3 color_1 = vec3(0.1);
    vec3 color_2 = vec3(0.7, 0.2, 0.8);
    
    vec3 output_color;
    if (i >= 1.0) {
        output_color = vec3(0.0);
    } else {
        output_color = mix(color_1, color_2, i);
    }

    outColor = vec4(output_color, 1.0);
    //outColor = vec4(vec3(i), 1.0);
	
}