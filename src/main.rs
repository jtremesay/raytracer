extern crate sdl2;

use raytracer::canvas::FrameBufferCanvas;
use raytracer::render::render;
use raytracer::scene::Scene;
use raytracer::sdl::copy_canvas_to_texture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::env;
use std::path::Path;
use std::time::Instant;

pub fn main() -> Result<(), String> {
    // Parse the CLI args
    let mut canvas_width = 640;
    let mut canvas_height = 480;
    let mut scene_path = Path::new("");
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    let mut i = 1;
    while i < args_count {
        let arg = &args[i];
        let next_arg = args.get(i + 1);

        if arg == "-w" || arg == "--width" {
            if let Some(next_arg) = next_arg {
                canvas_width = next_arg.parse().unwrap();
                i += 1;
            }
        } else if arg == "-h" || arg == "--height" {
            if let Some(next_arg) = next_arg {
                canvas_height = next_arg.parse().unwrap();
                i += 1;
            }
        } else {
            scene_path = Path::new(arg);
        }

        i += 1;
    }
    if scene_path.to_str().unwrap() == "" {
        return Err(String::from("missing scene"));
    }

    // Create the scene
    let scene = Scene::load_from_file(&scene_path);
    // Inialize the sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Raytracer", canvas_width, canvas_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut sdl_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = sdl_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, canvas_width, canvas_height)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = FrameBufferCanvas::new(canvas_width, canvas_height);

    let mut previous_now = Instant::now();
    let mut frame_count = 1;
    'running: loop {
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

        sdl_canvas.set_draw_color(Color::RGB(255, 0, 255));
        sdl_canvas.clear();

        render(&scene, &mut canvas);
        copy_canvas_to_texture(&canvas, &mut texture);
        sdl_canvas.copy(
            &texture,
            None,
            Some(Rect::new(0, 0, canvas_width, canvas_height)),
        )?;
        sdl_canvas.present();

        let now = Instant::now();
        let diff = now - previous_now;
        previous_now = now;

        let dt = diff.as_secs_f32();
        println!("frame {}, dt {}s, {} fps", frame_count, dt, 1.0 / dt);
        frame_count += 1;
    }

    Ok(())
}
