#[derive(Debug, Clone)]
pub enum ModStatus {
    Enabled,
    Disabled,
    NotInstalled,
}

#[derive(Debug)]
pub struct Mod {
    pub name: String,
    pub version: String,
    pub filepath: String,
    pub status: ModStatus,
}

impl Mod {
    pub fn new<S: Into<String>>(name: S, version: S, filepath: S) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            filepath: filepath.into(),
            status: ModStatus::NotInstalled,
        }
    }
}


