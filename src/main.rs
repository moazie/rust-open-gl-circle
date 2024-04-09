use raylib::prelude::*;

fn main() {
    let mut width_screen = 800;
    let mut height_screen = 600;

    let (mut rl, thread) = raylib::init()
        .size(width_screen, height_screen)
        .title("Circle Movement")
        .resizable()
        .build();

    let inf = f64::INFINITY as u32;
    rl.set_target_fps(inf);

    let font = rl.load_font(&thread, "src/font/Helvetica.ttf").expect("Could not load font");

    let mut scale_x = width_screen as f32 / 800.0;
    let mut scale_y = height_screen as f32 / 450.0;
    let mut scale: f32 = 1.0;
    let mut circle_pos = Vector2::new(width_screen as f32 / 2.0, height_screen as f32 / 2.0);
    let lerp_factor = 0.06; // Adjust this value for smoother or quicker movement

    while !rl.window_should_close() {
        
        let fps = rl.get_fps();
        if rl.is_window_resized() {
            height_screen = rl.get_screen_height();
            width_screen = rl.get_screen_width();
            scale_y = height_screen as f32 / 450.0;
            scale_x = width_screen as f32 / 800.0;
        }

        let mouse_position = rl.get_mouse_position();
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            rl.disable_cursor();
            // Interpolate the circle's position towards the mouse position
            circle_pos.x += (mouse_position.x - circle_pos.x) * lerp_factor;
            circle_pos.y += (mouse_position.y - circle_pos.y) * lerp_factor;
        } else {
            rl.enable_cursor();
        }

        circle_pos.x = circle_pos.x.abs();
        circle_pos.y = circle_pos.y.abs();

        let scroll = rl.get_mouse_wheel_move();
        if scroll != 0.0 {
            scale *= if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw a circle
        d.draw_circle_sector(
            circle_pos, // Center of the circle
            150.0 * scale_x.min(scale_y) * scale, // Radius of the circle
            0.0,                                   // Start angle
            360.0,                                 // End angle to cover a full circle
            90,                                   // Number of segments
            Color::WHITE,                          // Color of the circle
        );

        let str_circle_posx = circle_pos.x.round();
        let str_circle_posy = circle_pos.y.round();
        let zoom_round = (scale * 10.0).round() / 10.0;
        d.draw_text_ex(&font, &format!("Screen Width: {}", width_screen), Vector2::new(20.0, 10.0), 12.0, 2.0, Color::WHITE);
        d.draw_text_ex(&font, &format!("Screen Height: {}", height_screen), Vector2::new(20.0, 30.0), 12.0, 2.0, Color::WHITE);
        d.draw_text_ex(&font, &format!("Zoom: {}x", zoom_round), Vector2::new(20.0, 50.0), 12.0, 2.0, Color::WHITE);
        d.draw_text_ex(&font, &format!("Circle Pos: ({}, {})", str_circle_posx, str_circle_posy), Vector2::new(20.0, 70.0), 12.0, 2.0, Color::WHITE);
        d.draw_text_ex(&font, &format!("FPS: {}", fps), Vector2::new(20.0, 90.0), 12.0, 2.0, Color::WHITE);
    }
}
