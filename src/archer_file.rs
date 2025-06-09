use std::{
    fs::{self},
    path::Path,
};

use crate::{
    bashrc::bashrc_client::BashrcClient, model::raw_archer_file::RawArcherFile,
    package_manager::PackageManager,
};

pub struct ArcherFile {
    bashrc: Option<BashrcClient>,
    packages: Option<PackageManager>,
}

impl ArcherFile {
    pub fn new(path: &Path) -> Option<Self> {
        let path_archer = path.join("archer.toml");

        let content = fs::read_to_string(&path_archer).ok()?;

        let raw: RawArcherFile = toml::from_str(&content)
            .map_err(|err| {
                eprintln!(
                    "❌ Fehler beim Parsen von {}, \n{}",
                    path_archer.to_str().unwrap(),
                    err
                );
            })
            .ok()?;

        Some(Self {
            bashrc: raw.bashrc.map(|b| BashrcClient::new(raw.name.clone(), b)),
            packages: raw.packages.map(|p| PackageManager::new(p)),
        })
    }

    pub fn apply(&self) {
        if let Some(pkgs) = &self.packages {
            pkgs.install();
        }
        if let Some(bashrc) = &self.bashrc {
            bashrc.to_bashrc();
        }
    }
}
