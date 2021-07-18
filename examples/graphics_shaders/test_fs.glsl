//This shader flips the red and blue channels, and turns the colour to grayscale based on a float

#version 450 core

in VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} IN;

uniform vec4 drawColor;
uniform float grayness;

layout (location = 0) out vec4 Color;

void main() {
    float luma = 0.33 * drawColor.b + 0.5 * drawColor.g + 0.16 * drawColor.b;
    Color = vec4(IN.Color.xyz, 1.0) * mix(drawColor.bgra, vec4(vec3(luma), 1.0), 1.0 - grayness);
}
