use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Packages {
    pub pacman: Option<Vec<String>>,
    pub aur: Option<Vec<String>>,
}
