#version 330 core

in vec3 position;
in vec2 tex_coords;
uniform mat4 shadow_light_view_matrix;

uniform mat4 matrix;

smooth out vec2 frag_texcoord;
smooth out vec4 shadow_coord;

void main(){
	shadow_coord = shadow_light_view_matrix * vec4(position,1.0);
	frag_texcoord = tex_coords;
	gl_Position = matrix * vec4(position,1.0);
}

//=================

#version 330 core

uniform vec4 shadow_light_position;
uniform mat4 shadow_light_view_matrix;
uniform sampler2D diffuse_texture;
uniform sampler2D light_texture;
uniform sampler2D ui_texture;
uniform sampler2DShadow shadow_texture;
uniform sampler2D shadow_texture_debug;
uniform sampler2D depth_texture;
uniform sampler2D shadow_render_texture;

smooth in vec2 frag_texcoord;
smooth in vec4 shadow_coord;

out vec4 color;

void main(){

  vec3 difftex = texture(diffuse_texture, frag_texcoord).rgb;
  vec3 shadowtex = texture(shadow_render_texture, frag_texcoord).rgb;
  vec3 lighttex = texture(light_texture, frag_texcoord).rgb;
  vec4 uitex = texture(ui_texture, frag_texcoord).rgba;

  vec3 debug_shadow = texture(shadow_texture_debug, frag_texcoord).rgb;
	//if(uitex.a > 0.0){
		// color = uitex;

	// }else{
		//color = vec4(debug_shadow,1.0);
		//color = vec4(difftex * lighttex, 1.0);
		color = vec4(difftex * shadowtex * lighttex, 1.0);
		//color = vec4( shadowtex , 1.0);
		//color = vec4(1.0,1.0,1.0 , 1.0);
	// }
}