#version 450 core

in VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} IN;

uniform vec4 drawColor;

layout (location = 0) out vec4 Color;

void main() {
    Color = vec4(IN.Color.xyz, 1.0) * drawColor.bgra;
}
