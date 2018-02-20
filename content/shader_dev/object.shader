#version 150

in vec3 position;
in vec3 normal;
in vec3 tex_coords;

uniform mat4 u_matrix;
uniform mat4 u_world;

out vec3 v_normal;

void main(){
	v_normal = mat3(u_world) * normal;
	gl_Position = u_matrix * vec4(position,1.0);
}
//=================
#version 140


in vec3 v_normal;
in vec3 fragVert;
uniform mat4 u_matrix;

out vec4 diffuse_output;
out vec4 normal_output;
out vec4 position_output;

void main(){
	diffuse_output = vec4(1.0,0.0,0.0,1.0);
	position_output = vec4(v_normal,1.0);
	normal_output = vec4(v_normal,1.0);
}