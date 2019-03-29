#version 330

in vec3 position;
in vec2 tex_coords;

uniform mat4 matrix;

smooth out vec2 frag_texcoord;

void main(){
  frag_texcoord = tex_coords;
  gl_Position = matrix * vec4(position,1.0);
}

//=================

#version 330

uniform sampler2D diffuse_texture;
uniform sampler2D light_texture;
uniform sampler2D ui_texture;
smooth in vec2 frag_texcoord;

out vec4 color;

void main(){

  vec3 difftex = texture(diffuse_texture, frag_texcoord).rgb;
  vec3 lighttex = texture(light_texture, frag_texcoord).rgb;
  vec4 uitex = texture(ui_texture, frag_texcoord).rgba;

	if(uitex.a > 0.0){
		color = uitex;

	}else{
		color = vec4(difftex * lighttex, 1.0);
		//color = vec4(difftex , 1.0);
	}
}