use std::{
    env,
    fs::{self, File, read_to_string},
    path::Path,
};

use serde::{Deserialize, Serialize};

use super::raw_archer_file::RawArcherFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheFile {
    app_configs: Vec<RawArcherFile>,
}

impl CacheFile {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").expect("pls set HOME environtment variable");
        let path_string = format!("{}/.cache/share/archer/installed.toml", home_dir);
        let path_cache = Path::new(&path_string);

        read_or_create(path_cache)
    }
}

fn read_or_create(path: &Path) -> CacheFile {
    match read_to_string(path) {
        Ok(content) => toml::from_str(&content).expect("could not parse cache File"),
        Err(_) => {
            println!("could not find cache file: {}", &path.to_str().unwrap());
            create_file(path);

            CacheFile {
                app_configs: vec![],
            }
        }
    }
}

fn create_file(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(&format!(
            "could not create directory: {}",
            parent.to_str().unwrap()
        ));
    }
    File::create(path).expect(&format!(
        "could not create directory: {}",
        path.to_str().unwrap()
    ));
}
