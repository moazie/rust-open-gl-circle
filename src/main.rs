// Rust Standard Library
use std::f32::consts::PI;

// Reference to use Open GL
use gl;
use sdl2::event::Event;

// Reference to Objects.rs
mod objects;
use objects::*;

fn main() {
    // Init Window terms
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Bounds for Window width/height
    let mut window_width:i32 = 800;
    let mut window_height:i32 = 600;

    // Window
    let window = video_subsystem
        .window("Circle", window_width.try_into().unwrap(), window_height.try_into().unwrap())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
    }

    // Making a program
    let program = create_program().unwrap();
    program.set();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Center of the circle
    let mut center_x = window_width as f32 / 2.0;
    let mut center_y = window_height as f32 / 2.0;
    let radius = 400.0; // Adjust this value as needed
    let segments = 50; // Number of line segments to approximate the circle

    // Generate vertices
    for i in 0..=segments {
        let theta = 2.0 * PI * (i as f32 / segments as f32);
        let x = center_x + radius * theta.cos();
        let y = center_y + radius * theta.sin();
        vertices.push(x / window_width as f32); // Normalize to [0, 1]
        vertices.push(y / window_height as f32); // Normalize to [0, 1]
    }

    // Generate indices for drawing triangles
    for i in 1..segments {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }
    indices.push(0);
    indices.push(segments);
    indices.push(1);

    // vbo, vao, ibo
    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    // While running loop
    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(new_width, new_height) => {
                            // Update center coordinates based on new window dimensions
                            center_x = new_width as f32 / 2.0;
                            center_y = new_height as f32 / 2.0;
                            unsafe {
                                gl::Viewport(0, 0, new_width, new_height);
                            }
                            window_width = new_width;
                            window_height = new_height;

                            // Regenerate vertices with new dimensions
                            vertices.clear();
                            for i in 0..=segments {
                                let theta = 2.0 * PI * (i as f32 / segments as f32);
                                let x = center_x + radius * theta.cos();
                                let y = center_y + radius * theta.sin();
                                vertices.push(x / window_width as f32); // Normalize to [0, 1]
                                vertices.push(y / window_height as f32); // Normalize to [0, 1]
                            }
                            vbo.set(&vertices);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            )
        }
        window.gl_swap_window();
    }
}
