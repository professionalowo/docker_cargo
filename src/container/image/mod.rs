use std::ffi::{OsStr, OsString};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image {
    pub name: String,
    pub version: String,
}

impl Image {
    pub fn new_latest<I: Into<String>>(name: I) -> Self {
        Self {
            name: name.into(),
            version: "latest".into(),
        }
    }

    pub fn new_with_version<I: Into<String>, O: Into<String>>(name: I, version: O) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

impl Into<String> for Image {
    fn into(self) -> String {
        format!("{}:{}", self.name, self.version)
    }
}

impl Into<OsString> for Image {
    fn into(self) -> OsString {
        let img: String = self.into();
        img.into()
    }
}
