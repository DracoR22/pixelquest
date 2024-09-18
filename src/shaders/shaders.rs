pub static VERTEX_SHADER_SRC: &str = r#"
#version 140

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in uint texture_id;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
out float v_fog_factor;
flat out uint v_texture_id;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform float fog_start;
uniform float fog_end;

void main() {
    v_tex_coords = tex_coords;
    v_texture_id = texture_id;
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    
    // Snap vertices to grid
    vec3 snapped_position = round(position);
    vec4 world_pos = model * vec4(snapped_position, 1.0);
    gl_Position = perspective * view * world_pos;
    v_position = (view * world_pos).xyz;
    
    // Calculate fog factor
    float dist = length(v_position);
    v_fog_factor = clamp((fog_end - dist) / (fog_end - fog_start), 0.0, 1.0);
}
"#;

pub static FRAGMENT_SHADER_SRC: &str = r#"
 #version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
in float v_fog_factor;
flat in uint v_texture_id;

out vec4 color;

uniform vec3 u_light;
uniform sampler2D tex0;
uniform sampler2D tex1;
uniform sampler2D tex2;
uniform sampler2D tex3;
// Add more texture uniforms as needed (tex2, tex3, etc.)
uniform vec3 fog_color;

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
    }
    else if (v_texture_id == 3u) {
        diffuse_color = texture(tex3, v_tex_coords).rgb;
    }
    // Add more texture_id checks as needed
    
    vec3 normalized_normal = normalize(v_normal);
    float diffuse = max(dot(normalized_normal, normalize(u_light)), 0.0);
    
    // Combine ambient and diffuse lighting
    vec3 lighting = ambient_strength * ambient_color + diffuse_strength * diffuse * vec3(1.0);
    vec3 final_color = diffuse_color * lighting;
    
    // Apply fog
    color = vec4(mix(fog_color, final_color, v_fog_factor), 1.0);
}
"#;