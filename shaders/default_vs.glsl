#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UV;
layout (location = 2) in vec4 Color;

uniform mat4 mvp;

out VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} OUT;

void main() {
    gl_Position = mvp * vec4(Position, 1.0);
    OUT.Color = Color.xyz;
    OUT.UV = UV;
}
