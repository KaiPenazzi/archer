use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use super::BashrcLine::BashrcLine;

struct BashrcClient {
    name: String,
    lines: Vec<String>,
}

impl BashrcClient {
    pub fn to_bashrc(&self) {
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
                    BashrcLine::new(self.name.clone(), self.lines.clone().len()).to_identifier(),
                );

                for new_line in &self.lines {
                    new_bashrc.push(new_line.to_string());
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
            new_bashrc.push(BashrcLine::new(self.name.clone(), self.lines.len()).to_identifier());

            for line in &self.lines {
                new_bashrc.push(line.to_string());
            }
        }

        let mut file = File::create(&bashrc_path).expect("Failed to open .bashrc for writing");

        for line in new_bashrc {
            file.write(format!("{}\n", line).as_bytes())
                .expect("Failed to write to .bashrc");
        }
    }
}
