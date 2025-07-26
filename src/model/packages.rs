use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Packages {
    pub pacman: Option<Vec<String>>,
    pub aur: Option<Vec<String>>,
}
