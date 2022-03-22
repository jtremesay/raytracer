use raytracer::canvas::ImageCanvas;
use raytracer::render::render;
use raytracer::scene::Scene;
use std::path::Path;

fn main() {
    // Settings
    //let fb_size = (1920, 1080);
    let fb_size = (640, 480);

    // Create the scene
    let scene_path = Path::new("data/scenes/scene1.yml");
    let scene = Scene::load_from_file(&scene_path);

    // Render the scene
    let mut canvas = ImageCanvas::new(fb_size.0, fb_size.1);
    render(&scene, &mut canvas);
    canvas.save("image.png").unwrap();
}
