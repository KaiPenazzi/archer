use std::{
    env,
    fs::{self, File, read_to_string},
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{archer_file::ArcherFile, package_manager::PackageManager};

use super::raw_archer_file::RawArcherFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheFile {
    app_configs: Vec<RawArcherFile>,
    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
}

impl CacheFile {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").expect("pls set HOME environtment variable");
        let path_string = format!("{}/.cache/share/archer/installed.toml", home_dir);
        let path = PathBuf::from_str(&path_string).expect("idk could not path ....");

        Self::read_or_create(path)
    }

    fn read_or_create(path: PathBuf) -> CacheFile {
        match read_to_string(&path) {
            Ok(content) => {
                let mut cache_file: Self =
                    toml::from_str(&content).expect("could not parse cache File");
                cache_file.path = path;
                cache_file
            }
            Err(_) => {
                println!("could not find cache file: {}", &path.to_str().unwrap());
                Self::create_file(&path);

                CacheFile {
                    app_configs: vec![],
                    path,
                }
            }
        }
    }

    fn create_file(path: &PathBuf) {
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

    pub fn remove_packages(&mut self, archer_file: &ArcherFile) {
        if let Some(config_cache) = self
            .app_configs
            .iter_mut()
            .find(|config| config.name == archer_file.name)
            .as_mut()
        {
            if let Some(old_packages) = &mut config_cache.packages
                && let Some(new_packages) = &archer_file.packages
            {
                if let Some(old_pacman) = &old_packages.pacman {
                    let uninstall_pacman = match &new_packages.pacman {
                        Some(new_pacman) => old_pacman
                            .iter()
                            .filter(|package| !new_pacman.contains(package))
                            .cloned()
                            .collect(),
                        None => old_pacman.clone(),
                    };
                    old_packages.pacman = new_packages.pacman.clone();
                    PackageManager::remove(uninstall_pacman);
                } else {
                    old_packages.pacman = None;
                }

                if let Some(old_aur) = &old_packages.aur {
                    let uninstall_aur = match &new_packages.aur {
                        Some(new_pacman) => old_aur
                            .iter()
                            .filter(|package| !new_pacman.contains(package))
                            .cloned()
                            .collect(),
                        None => old_aur.clone(),
                    };
                    old_packages.aur = new_packages.aur.clone();
                    PackageManager::remove(uninstall_aur);
                } else {
                    old_packages.aur = None;
                }
            }
        } else {
            self.app_configs.push(archer_file.raw());
        }
    }

    pub fn write(&self) {
        let toml = toml::to_string_pretty(&self).expect("error while deserializing cache struct");
        _ = fs::write(&self.path, toml).expect("could not write cache file");
    }
}
