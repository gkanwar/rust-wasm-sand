#version 300 es
precision mediump float;

uniform sampler2D texture;
varying vec2 frag_texCoord;

void main() {
    gl_FragColor = texture2D(texture, frag_texCoord);
}