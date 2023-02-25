use std::path::Path;

use crate::scene::Scene;

pub mod lisp;

pub trait Saver {
    fn save_scene_to_file(&self, scene: &Scene, path: &Path);
}
