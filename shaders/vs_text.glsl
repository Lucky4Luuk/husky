#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UV;
layout (location = 2) in vec4 Color;

uniform vec2 offset;
uniform mat4 ortho;
uniform vec2 scale;

out VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} OUT;

void main() {
    gl_Position = ortho * vec4(Position.xy * scale + offset, Position.z, 1.0);
    OUT.Color = Color.xyz;
    OUT.UV = UV;
}
