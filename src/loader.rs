use crate::color::Color;
use crate::geometry::Sphere;
use crate::light::Light;
use crate::material::Material;
use crate::math::Vector3;
use crate::scene::Scene;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::YamlLoader;

pub fn load_scene_from_file(path: &Path) -> Scene {
    // Read the file
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    // Parse the data
    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let scene_data = &docs[0];

    // Create the scene
    let mut scene = Scene::new();

    // Configure the camera
    scene.camera.position.x = scene_data["camera"]["position"]["x"].as_f64().unwrap() as f32;
    scene.camera.position.y = scene_data["camera"]["position"]["y"].as_f64().unwrap() as f32;
    scene.camera.position.z = scene_data["camera"]["position"]["z"].as_f64().unwrap() as f32;
    scene.camera.view_port.width =
        scene_data["camera"]["view_port"]["width"].as_f64().unwrap() as f32;
    scene.camera.view_port.height = scene_data["camera"]["view_port"]["height"]
        .as_f64()
        .unwrap() as f32;
    scene.camera.view_port.distance = scene_data["camera"]["view_port"]["distance"]
        .as_f64()
        .unwrap() as f32;

    // Create the lights
    for light_data in scene_data["lights"].as_vec().unwrap().iter() {
        let light_type = light_data["type"].as_str().unwrap();
        let light_intensity = light_data["intensity"].as_f64().unwrap() as f32;
        let light = if light_type == "ambient" {
            Light::create_ambiant(light_intensity)
        } else if light_type == "omnidirectional" {
            Light::create_omnidirectional(
                light_intensity,
                Vector3::new(
                    light_data["source"]["x"].as_f64().unwrap() as f32,
                    light_data["source"]["y"].as_f64().unwrap() as f32,
                    light_data["source"]["z"].as_f64().unwrap() as f32,
                ),
            )
        } else if light_type == "directional" {
            Light::create_directional(
                light_intensity,
                Vector3::new(
                    light_data["direction"]["x"].as_f64().unwrap() as f32,
                    light_data["direction"]["y"].as_f64().unwrap() as f32,
                    light_data["direction"]["z"].as_f64().unwrap() as f32,
                ),
            )
        } else {
            panic!("Unsupported light type");
        };
        scene.lights.push(light)
    }

    // Create the spheres
    for sphere_data in scene_data["spheres"].as_vec().unwrap().iter() {
        let sphere = Sphere::new(
            Vector3::new(
                sphere_data["position"]["x"].as_f64().unwrap() as f32,
                sphere_data["position"]["y"].as_f64().unwrap() as f32,
                sphere_data["position"]["z"].as_f64().unwrap() as f32,
            ),
            sphere_data["radius"].as_f64().unwrap() as f32,
            Material::new(
                Color::new(
                    sphere_data["color"]["r"].as_f64().unwrap() as f32,
                    sphere_data["color"]["g"].as_f64().unwrap() as f32,
                    sphere_data["color"]["b"].as_f64().unwrap() as f32,
                ),
                sphere_data["specular"].as_f64().unwrap() as f32,
            ),
        );
        scene.spheres.push(sphere);
    }

    return scene;
}
