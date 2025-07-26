use std::env;

use clap::Parser;
use model::args::ARGs;
use package_manager::PackageManager;

mod archer_file;
mod bashrc;
mod model;
mod package_manager;
mod repo;
mod setup;

//"git@github.com:KaiPenazzi/configs.git"

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ARGs::parse();

    let repo_url = match args.repo {
        Some(repo) => repo,
        None => env::var("CONFIG_REPO").expect("set CONFIG_REPO or execute with --repo <Rep URL>"),
    };

    let repo = repo::get_repo("configs", &repo_url)?;

    PackageManager::install_aura()?;
    setup::install_config(&repo, args.program.as_deref())?;

    println!("Setup abgeschlossen!");
    Ok(())
}
