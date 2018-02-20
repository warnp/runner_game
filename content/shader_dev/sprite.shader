#version 140

in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 tex_coords;
//in uint i_tex_id;

out vec4 colorV;
out vec3 v_normal;
out vec2 v_tex_coords;
//flat out uint v_tex_id;

uniform mat4 matrix;

	void main(){
	v_tex_coords = tex_coords;
	gl_Position = matrix * vec4(position,1.0);
	//v_tex_id = i_tex_id;
}
//=================
#version 140

in vec4 colorV;
in vec2 v_tex_coords;
flat in uint v_tex_id;

out vec4 color;

uniform sampler2DArray tex;

void main(){
	color = texture(tex, vec3(v_tex_coords,0.0)).rgba;
	//color  = vec4(1.0,0.0,0.0,1.0);
}