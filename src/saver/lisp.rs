use crate::camera::{Camera, ViewPort};
use crate::color::Color;
use crate::light::{AmbiantLight, DirectionalLight, Light, OmniDirectionalLight};
use crate::material::Material;
use crate::math::Vector3;
use crate::scene::Scene;
use crate::sdf::{Node, SphereNode, UnionNode};
use std::fs::File;
use std::io::Write;

use super::Saver;

pub struct LispSaver {}

impl LispSaver {
    pub fn write_indent(&self, out: &mut dyn Write, level: u32) {
        write!(out, "\n").unwrap();
        for _ in 0..level {
            write!(out, "    ").unwrap();
        }
    }

    pub fn write_vector3(&self, out: &mut dyn Write, vector: &Vector3, level: u32) {
        self.write_indent(out, level);
        write!(out, "(vector3 {} {} {})", vector.x, vector.y, vector.z).unwrap();
    }

    pub fn write_view_port(&self, out: &mut dyn Write, view_port: &ViewPort, level: u32) {
        self.write_indent(out, level);
        write!(
            out,
            "(view_port {} {} {})",
            view_port.width, view_port.height, view_port.distance
        )
        .unwrap();
    }

    pub fn write_camera(&self, out: &mut dyn Write, camera: &Camera, level: u32) {
        self.write_indent(out, level);
        write!(out, "(camera ").unwrap();
        self.write_vector3(out, &camera.position, level + 1);
        write!(out, " ").unwrap();
        self.write_view_port(out, &camera.view_port, level + 1);
        write!(out, ")").unwrap();
    }

    pub fn write_union_node(&self, out: &mut dyn Write, node: &UnionNode, level: u32) {
        self.write_indent(out, level);
        write!(out, "(union (list").unwrap();
        for node in node.nodes.iter() {
            write!(out, " ").unwrap();
            self.write_node(out, node, level + 1);
        }
        write!(out, "))").unwrap();
    }

    pub fn write_color(&self, out: &mut dyn Write, color: &Color, level: u32) {
        self.write_indent(out, level);
        write!(out, "(color {} {} {})", color.r, color.g, color.b).unwrap();
    }

    pub fn write_material(&self, out: &mut dyn Write, material: &Material, level: u32) {
        self.write_indent(out, level);
        write!(out, "(material ").unwrap();
        self.write_color(out, &material.color, level + 1);
        self.write_indent(out, level + 1);
        write!(out, "{})", material.specular).unwrap();
    }

    pub fn write_sphere_node(&self, out: &mut dyn Write, node: &SphereNode, level: u32) {
        self.write_indent(out, level);
        write!(out, "(sphere ").unwrap();
        self.write_vector3(out, &node.position, level + 1);
        self.write_indent(out, level + 1);
        write!(out, "{}", node.radius).unwrap();
        self.write_material(out, &node.material, level + 1);
        write!(out, ")").unwrap();
    }

    pub fn write_node(&self, out: &mut dyn Write, node: &Box<dyn Node>, level: u32) {
        match node.as_any().downcast_ref::<UnionNode>() {
            Some(node) => self.write_union_node(out, node, level),
            _ => match node.as_any().downcast_ref::<SphereNode>() {
                Some(node) => self.write_sphere_node(out, node, level),
                _ => {}
            },
        }
    }

    pub fn write_ambiant_light(&self, out: &mut dyn Write, light: &AmbiantLight, level: u32) {
        self.write_indent(out, level);
        write!(out, "(ambiant").unwrap();
        self.write_indent(out, level + 1);
        write!(out, "{})", light.intensity).unwrap();
    }

    pub fn write_omnidirectional_light(
        &self,
        out: &mut dyn Write,
        light: &OmniDirectionalLight,
        level: u32,
    ) {
        self.write_indent(out, level);
        write!(out, "(omnidirectional").unwrap();
        self.write_indent(out, level + 1);
        write!(out, "{}", light.intensity).unwrap();
        self.write_vector3(out, &light.position, level + 1);
        write!(out, ")").unwrap();
    }

    pub fn write_directional_light(
        &self,
        out: &mut dyn Write,
        light: &DirectionalLight,
        level: u32,
    ) {
        self.write_indent(out, level);
        write!(out, "(directional").unwrap();
        self.write_indent(out, level + 1);
        write!(out, "{}", light.intensity).unwrap();
        self.write_vector3(out, &light.direction, level + 1);
        write!(out, ")").unwrap();
    }

    pub fn write_light(&self, out: &mut dyn Write, light: &Box<dyn Light>, level: u32) {
        match light.as_any().downcast_ref::<AmbiantLight>() {
            Some(light) => self.write_ambiant_light(out, light, level + 1),
            _ => match light.as_any().downcast_ref::<OmniDirectionalLight>() {
                Some(light) => self.write_omnidirectional_light(out, light, level + 1),
                _ => match light.as_any().downcast_ref::<DirectionalLight>() {
                    Some(light) => self.write_directional_light(out, light, level + 1),
                    _ => {}
                },
            },
        }
    }

    pub fn write_lights(&self, out: &mut dyn Write, lights: &Vec<Box<dyn Light>>, level: u32) {
        self.write_indent(out, level);
        write!(out, "(list").unwrap();
        for light in lights.iter() {
            write!(out, " ").unwrap();
            self.write_light(out, light, level);
        }
        write!(out, ")").unwrap();
    }

    pub fn write_scene(&self, out: &mut dyn Write, scene: &Scene, level: u32) {
        self.write_indent(out, level);
        write!(out, "(scene ").unwrap();
        self.write_camera(out, &scene.camera, level + 1);
        write!(out, " ").unwrap();
        self.write_node(out, &scene.root, level + 1);
        write!(out, " ").unwrap();
        self.write_lights(out, &scene.lights, level + 1);
        write!(out, ")").unwrap();
    }
}

impl Saver for LispSaver {
    fn save_scene_to_file(&self, scene: &crate::scene::Scene, path: &std::path::Path) {
        let mut file = File::create(path).unwrap();
        self.write_scene(&mut file, scene, 0);
    }
}
