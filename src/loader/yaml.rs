use crate::camera::Camera;
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
use yaml_rust::YamlLoader as YamlLoader_;

use super::Loader;

pub struct YamlLoader {}

impl YamlLoader {
    pub fn parse_vector3(&self, data: &Yaml) -> Vector3 {
        Vector3::new(
            data["x"].as_f64().unwrap() as f32,
            data["y"].as_f64().unwrap() as f32,
            data["z"].as_f64().unwrap() as f32,
        )
    }

    pub fn parse_camera(&self, data: &Yaml) -> Camera {
        Camera {
            position: self.parse_vector3(&data["position"]),
            view_port: self.parse_vector3(&data["view_port"]),
        }
    }

    pub fn parse_union_node(&self, data: &Yaml) -> Box<UnionNode> {
        Box::new(UnionNode {
            nodes: data["nodes"]
                .to_owned()
                .into_iter()
                .map(|node| self.parse_node(&node))
                .collect(),
        })
    }

    fn parse_color(&self, data: &Yaml) -> Color {
        Color {
            r: data["r"].as_f64().unwrap() as f32,
            g: data["g"].as_f64().unwrap() as f32,
            b: data["b"].as_f64().unwrap() as f32,
        }
    }

    fn parse_material(&self, data: &Yaml) -> Material {
        Material {
            color: self.parse_color(&data["color"]),
            specular: data["specular"].as_f64().unwrap() as f32,
        }
    }

    pub fn parse_sphere_node(&self, data: &Yaml) -> Box<SphereNode> {
        Box::new(SphereNode {
            position: self.parse_vector3(&data["position"]),
            radius: data["radius"].as_f64().unwrap() as f32,
            material: self.parse_material(&data["material"]),
        })
    }
    pub fn parse_node(&self, data: &Yaml) -> Box<dyn Node> {
        match data["type"].as_str().unwrap() {
            "union" => self.parse_union_node(data),
            "sphere" => self.parse_sphere_node(data),
            _ => panic!("unexpected node type"),
        }
    }

    pub fn parse_ambiant_light(&self, data: &Yaml) -> Box<AmbiantLight> {
        Box::new(AmbiantLight {
            intensity: data["intensity"].as_f64().unwrap() as f32,
        })
    }

    pub fn parse_omnidirectional_light(&self, data: &Yaml) -> Box<OmniDirectionalLight> {
        Box::new(OmniDirectionalLight {
            intensity: data["intensity"].as_f64().unwrap() as f32,
            position: self.parse_vector3(&data["position"]),
        })
    }

    pub fn parse_directional_light(&self, data: &Yaml) -> Box<DirectionalLight> {
        Box::new(DirectionalLight {
            intensity: data["intensity"].as_f64().unwrap() as f32,
            direction: self.parse_vector3(&data["direction"]),
        })
    }

    pub fn parse_ligth(&self, data: &Yaml) -> Box<dyn Light> {
        match data["type"].as_str().unwrap() {
            "ambiant" => self.parse_ambiant_light(data),
            "omnidirectional" => self.parse_omnidirectional_light(data),
            "directional" => self.parse_directional_light(data),
            _ => panic!("unexpected"),
        }
    }

    pub fn parse_ligths(&self, data: &Yaml) -> Vec<Box<dyn Light>> {
        data.to_owned()
            .into_iter()
            .map(|light| self.parse_ligth(&light))
            .collect()
    }

    pub fn parse_scene(&self, data: &Yaml) -> Scene {
        Scene {
            camera: self.parse_camera(&data["camera"]),
            root: self.parse_node(&data["root"]),
            lights: self.parse_ligths(&data["lights"]),
        }
    }
}

impl Loader for YamlLoader {
    fn load_scene_from_file(&self, path: &Path) -> Scene {
        // Read the file
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        // Parse the data
        let docs = YamlLoader_::load_from_str(&buffer).unwrap();
        let scene_data = &docs[0];

        self.parse_scene(scene_data)
    }
}
