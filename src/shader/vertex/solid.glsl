#version 330 core

in vec2 pos;
in vec4 col;
out vec4 fragCol;

void main() {
    gl_Position = vec4(pos.x, pos.y, 1.0, 1.0);
    fragCol = col;
}
