#version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
flat in uint v_texture_id;

out vec4 color;

uniform vec3 u_light;
uniform sampler2D tex0;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform sampler2D tex3;
uniform sampler2D tex4;

const vec3 ambient_color = vec3(0.7, 0.7, 0.7);
const float diffuse_strength = 0.3;
const float ambient_strength = 0.7;

void main() {
    vec3 diffuse_color;
    if (v_texture_id == 0u) {
        diffuse_color = texture(tex0, v_tex_coords).rgb;
    } else if (v_texture_id == 1u) {
        diffuse_color = texture(tex1, v_tex_coords).rgb;
    } else if (v_texture_id == 2u) {
        diffuse_color = texture(tex2, v_tex_coords).rgb;
    } else if (v_texture_id == 3u) {
        diffuse_color = texture(tex3, v_tex_coords).rgb;
    } else if (v_texture_id == 4u) {
        diffuse_color = texture(tex4, v_tex_coords).rgb;
    }

    vec3 normalized_normal = normalize(v_normal);
    float diffuse = max(dot(normalized_normal, normalize(u_light)), 0.0);

    // Combine ambient and diffuse lighting
    vec3 lighting = ambient_strength * ambient_color + diffuse_strength * diffuse * vec3(1.0);
    vec3 final_color = diffuse_color * lighting;

    // Set final color directly without fog
    color = vec4(final_color, 1.0);
}