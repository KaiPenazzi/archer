mod archer_file;
mod config;
mod installer;
mod repo;
mod setup;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = repo::get_repo("configs", "git@github.com:KaiPenazzi/configs.git")?;
    let packages = config::load_toml(&repo)?;

    setup::install_aura()?;
    setup::install_config(&repo)?;
    installer::install_packages(&packages)?;

    println!("Setup abgeschlossen!");
    Ok(())
}
