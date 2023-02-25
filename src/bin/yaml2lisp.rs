use std::{env, path::Path};

use raytracer::loader::{yaml::YamlLoader, Loader};
use raytracer::saver::{lisp::LispSaver, Saver};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let scene = YamlLoader {}.load_scene_from_file(Path::new(&args[1]));

    LispSaver {}.save_scene_to_file(&scene, Path::new(&args[2]));
}
