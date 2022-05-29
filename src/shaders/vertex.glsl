#version 300 es
precision mediump float;

in vec3 vertexPosition;
in vec2 vert_texCoord;
out vec2 frag_texCoord;

void main() {
    frag_texCoord = vert_texCoord;
    gl_Position = vec4(vertexPosition, 1.0);
}