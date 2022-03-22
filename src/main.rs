use raytracer::canvas::ImageCanvas;
use raytracer::color::Color;
use raytracer::geometry::Sphere;
use raytracer::math::Vector3;
use raytracer::render::render;
use raytracer::scene::Scene;

fn main() {
    // Settings
    //let fb_size = (1920, 1080);
    let fb_size = (640, 480);

    // Create the scene
    let mut scene = Scene::new();

    let sphere = Sphere::new(Vector3::new(0.0, -1.0, 3.0), 1.0, Color::RED);
    scene.spheres.push(sphere);

    let sphere = Sphere::new(Vector3::new(2.0, 0.0, 4.0), 1.0, Color::BLUE);
    scene.spheres.push(sphere);

    let sphere = Sphere::new(Vector3::new(-2.0, 0.0, 4.0), 1.0, Color::GREEN);
    scene.spheres.push(sphere);

    // Render the scene
    let mut canvas = ImageCanvas::new(fb_size.0, fb_size.1);
    render(&scene, &mut canvas);
    canvas.save("image.png").unwrap();
}
