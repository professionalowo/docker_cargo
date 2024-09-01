use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Protocol {
    All,
    Other(String),
    TCP,
    UDP,
}

impl Into<String> for Protocol {
    fn into(self) -> String {
        match self {
            Protocol::UDP => "udp".into(),
            Protocol::TCP => "tcp".into(),
            Protocol::Other(s) => s,
            _ => "".into(),
        }
    }
}
