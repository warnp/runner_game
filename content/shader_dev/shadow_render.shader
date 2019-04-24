#version 330 core

in vec3 position;
in vec3 normal;

uniform mat4 u_worldViewProjection;
uniform mat4 u_world;
uniform mat4 model;
uniform mat4 depth_bias_mvp;

out vec4 v_position;
out vec4 shadow_coord;
out vec3 v_normal;

void main(){
	v_normal = mat3(transpose(model)) * normal;
	shadow_coord = depth_bias_mvp * vec4(position,1.0);
	v_position = u_worldViewProjection * vec4(position,1.0);
	gl_Position = u_worldViewProjection * vec4(position,1.0);
}
//=================
#version 330 core



in vec3 v_normal;
in vec3 v_position;
in vec4 shadow_coord;

uniform mat4 u_matrix;
uniform sampler2DShadow shadow_texture;
uniform vec3 light_loc;
uniform vec3 light_dir;

out vec4 color;

void main(){
	vec4 divideW = shadow_coord / shadow_coord.w;
	float bias = max(0.05*(1.0-dot(v_normal, light_dir)),0.005);
	divideW.z += bias;
	vec2 texelSize = 1.0 / textureSize(shadow_texture, 0);
	float shadow = 1.0;
	
	//0.19 is empirical value to get rid of distant light artifact
	if(divideW.z > 0.19){
		shadow = 1.0;
	}else
		for(int i = -1; i <= 1; i++){
			for(int j = -1; j <= 1; j++){
				vec2 around_shadow_coord = shadow_coord.xy + vec2(i,j) * texelSize;
				vec4 new_shadow_coord = vec4(around_shadow_coord.x,around_shadow_coord.y, shadow_coord.z, shadow_coord.w);
				float pcf = textureProj(shadow_texture, new_shadow_coord);
				shadow += shadow_coord.w > pcf ? 0.0 :1.0;
			}
		}
		
		shadow /= 9.0;
	}
	color = vec4(shadow, shadow, shadow,1.0);
}