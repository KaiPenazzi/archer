use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use fs_extra::dir::{CopyOptions, copy};

use crate::archer_file::ArcherFile;

pub fn install_config(repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let target = Path::new(&home_dir).join(".config");

    for entry in fs::read_dir(repo)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = entry.file_name();

            if folder_name == ".git" {
                continue;
            };

            match ArcherFile::new(&path) {
                Some(archer_file) => {
                    archer_file.add_bashrc();
                }
                None => {}
            }

            copy(
                &path,
                &target,
                &CopyOptions::new().overwrite(true).copy_inside(true),
            )?;
        }
    }

    Ok(())
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
        .expect("Fehler beim Ausführen von pacman -U");

    Ok(())
}
