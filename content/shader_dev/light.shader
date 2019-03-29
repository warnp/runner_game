#version 330

in vec3 position;
in vec2 tex_coords;

out vec2 frag_coords;

uniform mat4 world_view_projection;
uniform mat4 world;

void main(){
	
	frag_coords = tex_coords;
	gl_Position = world * vec4(position,1.0);
}
//=================
#version 330
int NB_LIGHTS = 1;

uniform sampler2D position_texture;
uniform sampler2D diffuse_texture;
uniform sampler2D normal_texture;
uniform vec4 light_position;
uniform vec3 light_color;
uniform vec3 light_attenuation;
uniform float light_radius;
uniform vec4 viewPos;
uniform bool light_is_distant;
uniform vec3 light_direction;

in vec2 frag_coords;

out vec4 color;


void main(){
	vec3 FragPos = texture(position_texture, frag_coords).rgb;
	vec3 Normal = normalize(texture(normal_texture, frag_coords).rgb);
	vec3 Albedo = texture(diffuse_texture, frag_coords).rgb;
	
	vec3 viewDir = normalize(-FragPos);
	vec3 half_direction = normalize(normalize(light_position.rgb) + viewDir);
	float specular = pow(max(dot(half_direction, normalize(Normal)), 0.0), 16.0);
	
	vec3 lightning;// = Albedo * 0.1;
	if(light_is_distant){
		lightning = vec3(dot(Normal, normalize(light_direction)));	
	}else{
		
		vec3 lightDir = normalize(light_position.rgb - FragPos);
		vec3 diffuse = max(dot(Normal, lightDir), 0.0) * Albedo* light_color;
		lightning = diffuse;
	}
	
	color  = vec4(lightning + specular * vec3(1.0,1.0,1.0),1.0);
}