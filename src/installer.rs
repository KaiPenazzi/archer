use crate::config::Packages;
use std::process::Command;

pub fn install_packages(packages: &Packages) -> Result<(), Box<dyn std::error::Error>> {
    let pacman_missing: Vec<&String> = packages
        .pacman
        .packages
        .iter()
        .filter(|pkg| !is_installed(pkg))
        .collect();

    if !pacman_missing.is_empty() {
        println!("Installiere pacman Pakete...");
        Command::new("sudo")
            .args(["aura", "-S", "--noconfirm"])
            .args(&pacman_missing)
            .status()?;
    }

    let aur_missing: Vec<&String> = packages
        .aur
        .packages
        .iter()
        .filter(|pkg| !is_installed(pkg))
        .collect();

    if !aur_missing.is_empty() {
        println!("Installiere AUR Pakete...");
        Command::new("aura")
            .args(["-A", "--noconfirm"])
            .args(&aur_missing)
            .status()?;
    }

    Ok(())
}

fn is_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Q", pkg])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
