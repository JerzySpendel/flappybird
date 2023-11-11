#version 140
in vec2 f_uv;
out vec4 color;
uniform sampler2D tex;

void main(){
    color = texture(tex, f_uv);
}