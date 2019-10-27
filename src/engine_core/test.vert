// This is a demo shader and will be removed in the future

#version 450
#extension GL_ARB_separate_shader_objects: enable

out gl_PerVertex {
	vec4 gl_Position;
};

layout(location = 0) out vec3 fragColor;

vec2 positions[4] = vec2[](
	vec2( -1.0,  1.0 ),
	vec2( -1.0, -1.0 ),
	vec2(  1.0,  1.0 ),
	vec2(  1.0, -1.0 )
);

vec3 colors[4] = vec3[](
	vec3( 1.0, 0.0, 0.0 ),
	vec3( 1.0, 1.0, 1.0 ),
	vec3( 1.0, 0.0, 0.0 ),
	vec3( 1.0, 0.0, 0.0 )
);

void main() {
	gl_Position = vec4( positions[gl_VertexIndex], 0.0, 1.0 );
	fragColor = colors[gl_VertexIndex];
}
