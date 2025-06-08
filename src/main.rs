mod installer;
mod model;
mod repo;
mod setup;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = repo::get_repo("configs", "git@github.com:KaiPenazzi/configs.git")?;

    setup::install_aura()?;
    setup::install_config(&repo)?;

    println!("Setup abgeschlossen!");
    Ok(())
}
