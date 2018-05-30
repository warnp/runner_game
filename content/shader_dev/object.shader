#version 330

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 u_matrix;
uniform mat4 u_world;

out vec3 v_normal;
out vec2 v_tex_coords;

void main(){
	v_normal = mat3(u_world) * normal;
	v_tex_coords = tex_coords;
	gl_Position = u_matrix * vec4(position,1.0);
}
//=================
#version 330


in vec3 v_normal;
in vec3 fragVert;
in vec2 v_tex_coords;

uniform mat4 u_matrix;
uniform sampler2DArray tex;

out vec4 diffuse_output;
out vec4 normal_output;
out vec4 position_output;

void main(){
//	diffuse_output = vec4(1.0,0.0,0.0,1.0);
	diffuse_output = texture(tex, vec3(v_tex_coords,1.0)).rgba;
	position_output = vec4(v_normal,1.0);
	normal_output = vec4(v_normal,1.0);
}