#version 330 core

in vec2 pos;
in vec2 tex;
out vec2 fragTex;

void main() {
    gl_Position = vec4(pos.x, pos.y, 1.0, 1.0);
    fragTex = tex;
}
