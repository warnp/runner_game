#version 330

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 u_worldViewProjection;
uniform mat4 u_world;
uniform mat4 model;

out vec3 v_normal;
out vec2 v_tex_coords;
out vec3 v_position;

void main(){
	v_normal = mat3(transpose(model)) * normal;
	v_tex_coords = tex_coords;
	v_position = vec3(u_worldViewProjection) * position;
	gl_Position = u_worldViewProjection * vec4(position,1.0);
}
//=================
#version 330 core


in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform mat4 u_matrix;
uniform sampler2DArray tex;
uniform vec3 reverse_light_direction;

out vec4 diffuse_output;
out vec4 normal_output;
out vec4 position_output;

void main(){
	//vec3 normal = normalize(v_normal);
	//float light = dot(normal, normalize(vec3(1.0,0.0,0.0))) ;
	
//	diffuse_output = vec4(1.0,0.0,0.0,1.0);
	diffuse_output = texture(tex, vec3(v_tex_coords,1.0)).rgba ;
	position_output = vec4(v_position,1.0);
	normal_output = vec4(v_normal,1.0);
}