#version 330 core

in vec2 fragTex;
uniform sampler2D tex;
out vec4 color;

void main() {
    color = texture(tex, fragTex);
}
