use raytracer::canvas::ImageCanvas;
use raytracer::render::render;
use raytracer::scene::Scene;
use std::env;
use std::path::Path;

fn main() {
    // Parse the CLI args
    let mut canvas_width = 640;
    let mut canvas_height = 480;
    let mut scene_path = Path::new("");
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    let mut i = 1;
    while (i < args_count) {
        let arg = &args[i];
        let next_arg = args.get(i + 1);

        println!("{}", arg);
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
        println!("error: missing scene");
        return;
    }

    // Create the scene
    let scene = Scene::load_from_file(&scene_path);

    // Render the scene
    let mut canvas = ImageCanvas::new(canvas_width, canvas_height);
    render(&scene, &mut canvas);
    canvas.save("image.png").unwrap();
}
