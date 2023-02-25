use crate::canvas::Canvas;
use crate::math::lerp_color8;
use crate::render::Renderer;
use crate::scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::Instant;

pub fn sdl_main(
    scene: &Scene,
    renderer: &dyn Renderer,
    canvas: &mut dyn Canvas,
) -> Result<(), String> {
    // Inialize the sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create the windows
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let window = video_subsystem
        .window("Raytracer", canvas_width, canvas_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // Create the SDL canvas
    let mut sdl_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Create the output texture
    let texture_creator = sdl_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, canvas_width, canvas_height)
        .map_err(|e| e.to_string())?;

    // Initialize the event loop
    let mut event_pump = sdl_context.event_pump()?;

    // Start the main loop
    let mut previous_now = Instant::now();
    let mut frame_count = 1;
    let mut fps_counter = 0.0;
    'running: loop {
        // Handle the events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Render the scene to our canvas
        renderer.render(&scene, canvas);

        // Clear the SDL canvas with a nice magenta color for
        // catching undrawn pixels
        sdl_canvas.set_draw_color(Color::RGB(255, 0, 255));
        sdl_canvas.clear();

        // Copy the content of our canvas to the SDL texture
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let (width, height) = (canvas.width(), canvas.height());
                for y in 0..height {
                    for x in 0..width {
                        let color = canvas.get_pixel(x, y);
                        let offset_out = (height - 1 - y) as usize * pitch + x as usize * 3;

                        buffer[offset_out] = lerp_color8(color.r);
                        buffer[offset_out + 1] = lerp_color8(color.g);
                        buffer[offset_out + 2] = lerp_color8(color.b);
                    }
                }
            })
            .unwrap();

        // Draw the SDL texture
        sdl_canvas.copy(
            &texture,
            None,
            Some(Rect::new(0, 0, canvas_width, canvas_height)),
        )?;

        // Present the SDL canvas
        sdl_canvas.present();

        // Update the timer
        let now = Instant::now();
        let diff = now - previous_now;
        previous_now = now;

        // Print dt und update the frame count
        let dt = diff.as_secs_f32();
        fps_counter += dt;
        if frame_count % 60 == 0 {
            println!(
                "frame {}, avg dt {}s, avg {} fps",
                frame_count,
                fps_counter / 60.0,
                1.0 / (fps_counter / 60.0)
            );
            fps_counter = 0.0;
        }
        frame_count += 1;
    }
    Ok(())
}
