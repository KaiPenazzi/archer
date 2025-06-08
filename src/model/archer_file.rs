use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use serde::Deserialize;

use crate::installer;

use super::packages::Packages;

#[derive(Debug, Deserialize)]
pub struct ArcherFile {
    name: String,
    bashrc: Option<Vec<String>>,
    packages: Option<Packages>,
}

impl ArcherFile {
    pub fn new(path: &Path) -> Option<Self> {
        let path_archer = path.join("archer.toml");

        let content = match fs::read_to_string(path_archer) {
            Ok(content) => content,
            Err(_) => {
                return None;
            }
        };

        match toml::from_str(&content) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("could not parse: {}", path.to_str().unwrap());
                println!("{}", err);
                None
            }
        }
    }

    pub fn add_bashrc(&self) {
        let bashrc_path = Path::new(&env::var("HOME").unwrap()).join(".bashrc");
        let bashrc = fs::File::open(&bashrc_path).unwrap();
        let reader = BufReader::new(bashrc);

        let mut new_bashrc: Vec<String> = Vec::new();
        let mut wait = 0;
        let mut included = false;

        for line in reader.lines() {
            let line = line.unwrap();

            let bashrc_line = BashrcLine::from_line(&line);

            if bashrc_line.is_some() && bashrc_line.clone().unwrap().name == self.name {
                new_bashrc.push(
                    BashrcLine::new(self.name.clone(), self.bashrc.clone().unwrap().len())
                        .to_identifier(),
                );

                for new_line in self.bashrc.clone().unwrap() {
                    new_bashrc.push(new_line);
                }

                wait = bashrc_line.unwrap().count.clone();
                included = true;
            } else {
                if wait > 0 {
                    wait -= 1;
                } else {
                    new_bashrc.push(line);
                }
            }
        }

        if !included {
            new_bashrc.push(
                BashrcLine::new(
                    self.name.clone(),
                    self.bashrc
                        .clone()
                        .expect(&format!("toml syntax error in: {}", &self.name))
                        .len(),
                )
                .to_identifier(),
            );

            for line in self.bashrc.clone().unwrap() {
                new_bashrc.push(line);
            }
        }

        let mut file = File::create(&bashrc_path).expect("Failed to open .bashrc for writing");

        for line in new_bashrc {
            file.write(format!("{}\n", line).as_bytes())
                .expect("Failed to write to .bashrc");
        }
    }

    pub fn install_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.packages {
            Some(packages) => {
                installer::install_packages(packages)?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
struct BashrcLine {
    pub name: String,
    pub count: usize,
}

impl BashrcLine {
    pub fn new(name: String, count: usize) -> Self {
        BashrcLine { name, count }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        if !line.starts_with("# archer(") {
            return None;
        }

        let inner = &line["# archer(".len()..line.len() - 1];

        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return None;
        }

        let name = parts[0].to_string();
        let count = parts[1].parse::<usize>().ok()?;

        Some(BashrcLine { name, count })
    }

    pub fn to_identifier(&self) -> String {
        format!("# archer({}, {})", self.name, self.count)
    }
}
