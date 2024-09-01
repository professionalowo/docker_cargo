use super::DockerError;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Socket {
    pub port: u16,
    pub protocol: Option<String>,
}

impl Socket {
    pub fn new(port: u16, protocol: Option<String>) -> Self {
        Socket { port, protocol }
    }
    pub fn format_protocol(&self) -> String {
        if let Some(prot) = &self.protocol {
            format!("{}/{}", self.port, prot)
        } else {
            format!("{}", self.port)
        }
    }
}

impl TryFrom<String> for Socket {
    type Error = DockerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 2 {
            return Err(DockerError::new(
                ErrorKind::InvalidInput,
                "Invalid socket format",
            ));
        }
        let [port_str, protocol] = match parts[0..2] {
            [port, protocol] => [port, protocol],
            _ => {
                return Err(DockerError::new(
                    ErrorKind::InvalidInput,
                    "Invalid socket format",
                ))
            }
        };
        let port: u16 = match port_str.parse() {
            Ok(port) => port,
            Err(_) => return Err(DockerError::new(ErrorKind::InvalidInput, "Invalid port")),
        };
        Ok(Socket::new(port, Some(protocol.to_string())))
    }
}
