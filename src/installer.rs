use crate::model::packages::Packages;
use std::process::Command;

pub fn install_packages(packages: &Packages) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pkgs) = &packages.pacman {
        let pacman_missing: Vec<&String> = pkgs.get_not_installed();
        if !pacman_missing.is_empty() {
            println!("Installiere pacman Pakete...");
            Command::new("sudo")
                .args(["aura", "-S", "--noconfirm"])
                .args(&pacman_missing)
                .status()?;
        }
    }

    if let Some(pkgs) = &packages.aur {
        let aur_missing: Vec<&String> = pkgs.get_not_installed();
        if !aur_missing.is_empty() {
            println!("Installiere AUR Pakete...");
            Command::new("aura")
                .args(["-A", "--noconfirm"])
                .args(&aur_missing)
                .status()?;
        }
    }

    Ok(())
}

pub fn is_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Q", pkg])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
