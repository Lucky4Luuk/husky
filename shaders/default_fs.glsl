#version 450 core

in VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} IN;

layout (location = 0) out vec4 Color;

void main() {
    Color = vec4(IN.Color, 1.0);
}
