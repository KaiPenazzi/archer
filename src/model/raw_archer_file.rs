use serde::{Deserialize, Serialize};

use super::packages::Packages;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawArcherFile {
    pub name: String,
    pub bashrc: Option<Vec<String>>,
    pub packages: Option<Packages>,
}
