// This is a demo shader and will be removed in the future

#version 450
#extension GL_ARB_separate_shader_objects: enable

out gl_PerVertex {
	vec4 gl_Position;
};
layout(location = 0) out float time;
layout(location = 1) out float width;
layout(location = 2) out float height;

layout(set = 0, binding = 0) uniform Data {
	float time;
	float width;
	float height;
} uniforms;

vec2 positions[4] = vec2[](
	vec2( -1.0,  1.0 ),
	vec2( -1.0, -1.0 ),
	vec2(  1.0,  1.0 ),
	vec2(  1.0, -1.0 )
);

void main() {
	gl_Position = vec4( positions[gl_VertexIndex], 0.0, 1.0 );
	time = uniforms.time;
	width = uniforms.width;
	height = uniforms.height;
}
