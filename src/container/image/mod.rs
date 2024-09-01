use std::ffi::OsString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Image {
    //literal tag "<name>:<version>""
    Tag(String),
    // <name>:latest -> 0:latest
    Latest(String),
    // <name>:<version> -> 0:1
    Pinned(String, String),
}

impl Image {
    pub fn new_latest<I: Into<String>>(name: I) -> Self {
        Self::Latest(name.into())
    }

    pub fn new_with_version<I: Into<String>, O: Into<String>>(name: I, version: O) -> Self {
        Self::Pinned(name.into(), version.into())
    }
}

impl Into<String> for Image {
    fn into(self) -> String {
        match self {
            Image::Tag(t) => t,
            Image::Latest(name) => format!("{}:{}", name, "latest"),
            Image::Pinned(name, version) => format!("{}:{}", name, version),
        }
    }
}

impl Into<OsString> for Image {
    fn into(self) -> OsString {
        let img: String = self.into();
        OsString::from(img)
    }
}
