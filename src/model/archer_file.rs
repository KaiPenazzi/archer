use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use serde::Deserialize;

use crate::installer;

use super::packages::Packages;

#[derive(Debug, Deserialize)]
pub struct ArcherFile {
    name: String,
    bashrc: Option<Vec<String>>,
    packages: Option<Packages>,
}

impl ArcherFile {
    pub fn new(path: &Path) -> Option<Self> {
        let path_archer = path.join("archer.toml");

        let content = match fs::read_to_string(path_archer) {
            Ok(content) => content,
            Err(_) => {
                return None;
            }
        };

        match toml::from_str(&content) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("could not parse: {}", path.to_str().unwrap());
                println!("{}", err);
                None
            }
        }
    }

    pub fn install_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.packages {
            Some(packages) => {
                installer::install_packages(packages)?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
