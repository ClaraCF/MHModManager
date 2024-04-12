use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModStatus {
    Enabled,
    Disabled,
    NotInstalled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub name: String,
    pub version: String,
    pub filepath: String,
    pub status: ModStatus,
    pub files: Option<Vec<String>>,
}

impl Mod {
    pub fn new<S: Into<String>>(name: S, version: S, filepath: S) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            filepath: filepath.into(),
            status: ModStatus::NotInstalled,
            files: None,
        }
    }
}
