#version 450 core

layout(location = 0) uniform sampler2D tex;

in VS_OUTPUT {
    vec3 Color;
    vec2 UV;
} IN;

layout(location = 0) out vec4 Color;

void main() {
    vec4 data = texture(tex, IN.UV);
    float alpha = data.a;
    vec3 col = data.rgb;
    if (alpha <= 0.0) {
        discard;
    }
    // Color = vec4(IN.Color, alpha);
    // Color = vec4(1.0);
    Color = vec4(col, 1.0);
}
