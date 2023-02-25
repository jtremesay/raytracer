extern crate sdl2;

use raytracer::canvas::FrameBufferCanvas;
use raytracer::image::save_canvas_to_file;
use raytracer::loader::load_scene_from_file;
use raytracer::render::opengl::OpenGLRenderer;
use raytracer::render::software::SoftwareRenderer;
use raytracer::render::{Renderer, RendererType};
use raytracer::sdl::sdl_main;

use std::env;
use std::path::Path;

pub fn main() -> Result<(), String> {
    // Parse the CLI args
    let mut canvas_width = 640;
    let mut canvas_height = 480;
    let mut scene_path = Path::new("");
    let mut output_image_path = None;
    let mut renderer_type = RendererType::Software;
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
        } else if arg == "-o" || arg == "--output" {
            if let Some(next_arg) = next_arg {
                output_image_path = Some(Path::new(next_arg));
                i += 1;
            }
        } else if arg == "-r" || arg == "--renderer" {
            if let Some(next_arg) = next_arg {
                renderer_type = match next_arg.as_str() {
                    "software" => RendererType::Software,
                    "opengl" => RendererType::OpenGL,
                    _ => panic!("unsupported renderer"),
                };
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
    let scene = load_scene_from_file(&scene_path);

    // Create the render
    let renderer: Box<dyn Renderer> = match renderer_type {
        RendererType::Software => Box::new(SoftwareRenderer {}),
        RendererType::OpenGL => Box::new(OpenGLRenderer {}),
    };

    // Create the canvas
    let mut canvas = FrameBufferCanvas::new(canvas_width, canvas_height);

    // Do one render, save it and and return
    if let Some(path) = output_image_path {
        renderer.render(&scene, &mut canvas);
        save_canvas_to_file(&canvas, &path);

        return Ok(());
    }

    sdl_main(&scene, renderer.as_ref(), &mut canvas)
}
