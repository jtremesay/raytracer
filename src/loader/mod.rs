use crate::scene::Scene;
use std::path::Path;

pub mod yaml;

pub trait Loader {
    fn load_scene_from_file(&self, path: &Path) -> Scene;
}
