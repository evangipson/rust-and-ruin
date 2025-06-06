#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;
varying vec2 iMouse;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;
uniform vec2 _Mouse;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
    iMouse = _Mouse.xy;
}