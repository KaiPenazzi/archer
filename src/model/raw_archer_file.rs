
use serde::Deserialize;

use super::packages::Packages;

#[derive(Debug, Deserialize)]
pub struct RawArcherFile {
    pub name: String,
    pub bashrc: Option<Vec<String>>,
    pub packages: Option<Packages>,
}
