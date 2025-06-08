use crate::installer::is_installed;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageList {
    pub packages: Vec<String>,
}

impl PackageList {
    pub fn get_not_installed(&self) -> Vec<&String> {
        self.packages
            .iter()
            .filter(|pkg| !is_installed(pkg))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Packages {
    pub pacman: Option<PackageList>,
    pub aur: Option<PackageList>,
}
