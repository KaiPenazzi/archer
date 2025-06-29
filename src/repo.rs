use std::{env, path::Path, process::Command};

pub fn get_repo(name: &str, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let repo_path = format!("{}/code/{}", home_dir, name);

    if Path::new(&repo_path).exists() {
        println!("Repo existiert, führe git pull aus...");
        Command::new("git")
            .args(["-C", &repo_path, "pull"])
            .status()?;
    } else {
        println!("Klonen des Repos...");
        Command::new("git")
            .args(["clone", url, &repo_path])
            .status()?;
    }

    Ok(repo_path)
}
