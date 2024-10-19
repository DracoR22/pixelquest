#version 140

in vec2 position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    vec4 world_position = model * vec4(position, 0.0, 1.0);   // Apply the model transformation
    vec4 view_position = view * world_position;               // Apply the view transformation
    gl_Position = perspective * view_position;                // Apply the perspective projection
}
