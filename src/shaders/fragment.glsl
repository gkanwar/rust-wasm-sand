#version 300 es
precision mediump float;

uniform sampler2D texSampler;
in vec2 frag_texCoord;
out vec4 FragColor;

void main() {
    FragColor = texture(texSampler, frag_texCoord);
}