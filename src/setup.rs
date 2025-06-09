use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use fs_extra::dir::{CopyOptions, copy};

use crate::model::archer_file::ArcherFile;

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
                    archer_file.install_packages().expect(&format!(
                        "could not install package for {}",
                        path.to_str().unwrap()
                    ));
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
