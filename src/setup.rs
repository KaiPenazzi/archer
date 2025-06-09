use std::{env, fs, path::Path};

use fs_extra::dir::{CopyOptions, copy};

use crate::archer_file::ArcherFile;

pub fn install_config(repo: &str, program: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let target = Path::new(&home_dir).join(".config");

    match program {
        Some(program) => {
            let path = Path::new(repo).join(program);

            if let Some(archer_file) = ArcherFile::new(&path) {
                archer_file.apply();
            }

            copy(
                &path,
                &target,
                &CopyOptions::new().overwrite(true).copy_inside(true),
            )?;
        }
        None => {
            for entry in fs::read_dir(repo)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let folder_name = entry.file_name();

                    if folder_name == ".git" {
                        continue;
                    };

                    if let Some(archer_file) = ArcherFile::new(&path) {
                        archer_file.apply();
                    }

                    copy(
                        &path,
                        &target,
                        &CopyOptions::new().overwrite(true).copy_inside(true),
                    )?;
                }
            }
        }
    }

    Ok(())
}
