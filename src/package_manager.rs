use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use crate::model::packages::Packages;

pub struct PackageManager {
    pub aur: Option<Vec<String>>,
    pub pacman: Option<Vec<String>>,
}

impl PackageManager {
    pub fn new(packages: Packages) -> Self {
        Self {
            aur: packages.aur,
            pacman: packages.pacman,
        }
    }

    pub fn install(&self) {
        if let Some(pkgs) = &self.pacman {
            let pacman_missing: Vec<&String> = Self::get_missing_packages(pkgs);
            if !pacman_missing.is_empty() {
                println!("Installiere pacman Pakete...");
                Command::new("sudo")
                    .args(["aura", "-S", "--noconfirm"])
                    .args(&pacman_missing)
                    .status()
                    .expect("could not install missing pacman packages");
            }
        }

        if let Some(pkgs) = &self.aur {
            let aur_missing: Vec<&String> = Self::get_missing_packages(pkgs);
            if !aur_missing.is_empty() {
                println!("Installiere AUR Pakete...");
                Command::new("aura")
                    .args(["-A", "--noconfirm"])
                    .args(&aur_missing)
                    .status()
                    .expect("could not install missing aur packages");
            }
        }
    }

    fn get_missing_packages(pkgs: &Vec<String>) -> Vec<&String> {
        pkgs.iter()
            .filter(|pkg| !PackageManager::is_installed(pkg))
            .collect()
    }

    fn is_installed(pkg: &str) -> bool {
        Command::new("pacman")
            .args(["-Q", pkg])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn install_aura() -> Result<(), Box<dyn std::error::Error>> {
        if Command::new("which")
            .arg("aura")
            .stdout(Stdio::null())
            .status()?
            .success()
        {
            println!("Aura ist bereits installiert.");
            return Ok(());
        }

        let aura_dir = "/tmp/aura";
        if Path::new(aura_dir).exists() {
            fs::remove_dir_all(aura_dir)?;
        }

        Command::new("git")
            .args(["clone", "https://aur.archlinux.org/aura.git", aura_dir])
            .status()
            .expect("Fehler beim Klonen des aura-Repos");

        Command::new("makepkg")
            .current_dir(aura_dir)
            .args(["-s", "--noconfirm"])
            .status()
            .expect("Fehler beim Ausführen von makepkg");

        let entries = fs::read_dir(aura_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "zst")
                    && entry.file_name().to_string_lossy().contains("pkg.tar")
            })
            .collect::<Vec<_>>();

        if entries.is_empty() {
            return Err("Keine Paketdatei gefunden.".into());
        }

        let pkg_path = entries[0].path();
        Command::new("sudo")
            .arg("pacman")
            .arg("-U")
            .arg(pkg_path)
            .status()
            .expect("Fehler beim installieren von aura");

        Ok(())
    }

    pub fn remove(packages: Vec<String>) {
        if packages.len() > 0 {
            println!("remove packages: ");
            Command::new("aura")
                .args(["-R", "--noconfirm"])
                .args(&packages)
                .status()
                .expect("could not remove packages");
        }
    }

    pub fn packages(&self) -> Packages {
        Packages {
            pacman: self.pacman.clone(),
            aur: self.aur.clone(),
        }
    }
}
