#version 450 core

uniform sampler2D tex;

in VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} IN;

layout(location = 0) out vec4 Color;

void main() {
    // float alpha = texture(tex, IN.UV).r;
    // if (alpha <= 0.0) {
    //     discard;
    // }
    // Color = vec4(IN.Color, 1.0);
    Color = vec4(1.0);
}
