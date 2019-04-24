#version 330 core
in vec3 position;

uniform mat4 depth_mvp;

void main() {
	gl_Position = depth_mvp * vec4(position,1.0);
}
//=================
#version 330 core
out float fragment_depth;

void main(){
	fragment_depth = gl_FragCoord.z;
}