#version 330 core

layout(location = 0) in vec2 Position;

uniform vec2 ScreenSize; // Uniform variable for passing screen size

void main()
{
    // Convert Position from range [0, 1] to range [-1, 1]
    vec2 normalizedPosition = (Position - vec2(0.5));
    
    // Adjust normalized position based on screen aspect ratio

    gl_Position = vec4(normalizedPosition, 0.0, 1.0);
}
