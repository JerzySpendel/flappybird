#version 140

in vec3 position;
in vec2 uv;

out vec2 f_uv;
uniform mat3 model;

void main(){
    gl_Position = vec4(model * position, 1);
    f_uv = uv;
}