use serde::Deserialize;
use std::{fs, path::Path};

use crate::installer::is_installed;

#[derive(Debug, Deserialize)]
pub struct PackageList {
    pub packages: Vec<String>,
}

impl PackageList {
    pub fn get_not_installed(&self) -> Vec<&String> {
        self.packages
            .iter()
            .filter(|pkg| !is_installed(pkg))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Packages {
    pub pacman: PackageList,
    pub aur: PackageList,
}

pub fn load_toml(repo_path: &str) -> Result<Packages, Box<dyn std::error::Error>> {
    let path = Path::new(repo_path).join("arch/packages.toml");
    let content = fs::read_to_string(path)?;
    let packages: Packages = toml::from_str(&content)?;
    Ok(packages)
}
