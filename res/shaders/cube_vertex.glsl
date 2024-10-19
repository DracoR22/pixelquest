#version 140

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in uint texture_id;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
flat out uint v_texture_id;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

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
}