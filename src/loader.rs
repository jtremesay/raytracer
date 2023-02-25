use crate::camera::{Camera, ViewPort};
use crate::color::Color;
use crate::light::{AmbiantLight, DirectionalLight, Light, OmniDirectionalLight};
use crate::material::Material;
use crate::math::Vector3;
use crate::scene::Scene;
use crate::sdf::{Node, SphereNode, UnionNode};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn parse_vector3(data: &Yaml) -> Vector3 {
    Vector3::new(
        data["x"].as_f64().unwrap() as f32,
        data["y"].as_f64().unwrap() as f32,
        data["z"].as_f64().unwrap() as f32,
    )
}

pub fn parse_view_port(data: &Yaml) -> ViewPort {
    ViewPort {
        width: data["width"].as_f64().unwrap() as f32,
        height: data["height"].as_f64().unwrap() as f32,
        distance: data["distance"].as_f64().unwrap() as f32,
    }
}

pub fn parse_camera(data: &Yaml) -> Camera {
    Camera {
        position: parse_vector3(&data["position"]),
        view_port: parse_view_port(&data["view_port"]),
    }
}

pub fn parse_union_node(data: &Yaml) -> Box<UnionNode> {
    Box::new(UnionNode {
        nodes: data["nodes"]
            .to_owned()
            .into_iter()
            .map(|node| parse_node(&node))
            .collect(),
    })
}

fn parse_color(data: &Yaml) -> Color {
    Color {
        r: data["r"].as_f64().unwrap() as f32,
        g: data["g"].as_f64().unwrap() as f32,
        b: data["b"].as_f64().unwrap() as f32,
    }
}

fn parse_material(data: &Yaml) -> Material {
    Material {
        color: parse_color(&data["color"]),
        specular: data["specular"].as_f64().unwrap() as f32,
    }
}

pub fn parse_sphere_node(data: &Yaml) -> Box<SphereNode> {
    Box::new(SphereNode {
        position: parse_vector3(&data["position"]),
        radius: data["radius"].as_f64().unwrap() as f32,
        material: parse_material(&data["material"]),
    })
}
pub fn parse_node(data: &Yaml) -> Box<dyn Node> {
    match data["type"].as_str().unwrap() {
        "union" => parse_union_node(data),
        "sphere" => parse_sphere_node(data),
        _ => panic!("unexpected node type"),
    }
}

pub fn parse_ambiant_light(data: &Yaml) -> Box<AmbiantLight> {
    Box::new(AmbiantLight {
        intensity: data["intensity"].as_f64().unwrap() as f32,
    })
}

pub fn parse_omnidirectional_light(data: &Yaml) -> Box<OmniDirectionalLight> {
    Box::new(OmniDirectionalLight {
        intensity: data["intensity"].as_f64().unwrap() as f32,
        position: parse_vector3(&data["position"]),
    })
}

pub fn parse_directional_light(data: &Yaml) -> Box<DirectionalLight> {
    Box::new(DirectionalLight {
        intensity: data["intensity"].as_f64().unwrap() as f32,
        direction: parse_vector3(&data["direction"]),
    })
}

pub fn parse_ligth(data: &Yaml) -> Box<dyn Light> {
    match data["type"].as_str().unwrap() {
        "ambiant" => parse_ambiant_light(data),
        "omnidirectional" => parse_omnidirectional_light(data),
        "directional" => parse_directional_light(data),
        _ => panic!("unexpected"),
    }
}

pub fn parse_ligths(data: &Yaml) -> Vec<Box<dyn Light>> {
    data.to_owned()
        .into_iter()
        .map(|light| parse_ligth(&light))
        .collect()
}

pub fn parse_scene(data: &Yaml) -> Scene {
    Scene {
        camera: parse_camera(&data["camera"]),
        root: parse_node(&data["root"]),
        lights: parse_ligths(&data["ligths"]),
    }
}

pub fn load_scene_from_file(path: &Path) -> Scene {
    // Read the file
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    // Parse the data
    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let scene_data = &docs[0];

    parse_scene(scene_data)
}
