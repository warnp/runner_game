#version 330

in vec3 position;
in vec2 tex_coords;

out vec2 frag_coords;

uniform mat4 matrix;

void main(){
	frag_coords = tex_coords;
	gl_Position = matrix * vec4(position,1.0);
}
//=================
#version 330

uniform sampler2D position_texture;
uniform sampler2D normal_texture;
uniform vec4 light_position;
uniform vec3 light_color;
uniform vec3 light_attenuation;
uniform float light_radius;

in vec2 frag_coords;

out vec4 color;


void main(){
	vec4 position = texture(position_texture, frag_coords);
	vec4 normal = texture(normal_texture, frag_coords);
	vec3 light_vector = light_position.xyz - position.xyz;
	float light_distance = abs(length(light_vector));
	vec3 normal_vector = normalize(normal.xyz);
	float diffuse = max(dot(normal_vector, light_vector), 0.0);
	if(diffuse > 0.0){
		float attenuation_factor = 1.0 /(
			light_attenuation.x +
			(light_attenuation.y * light_distance) +
			(light_attenuation.y * light_distance * light_distance)
		);
		attenuation_factor *= (1.0 - pow((light_distance /light_radius), 2.0));
		attenuation_factor = max(attenuation_factor, 0.0);
		diffuse *= attenuation_factor;
	}
	color  = vec4(light_color * diffuse,1.0);
}