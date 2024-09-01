use super::DockerError;
use protocol::Protocol;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
pub mod protocol;
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Socket {
    pub port: u16,
    pub protocol: Protocol,
}

impl Socket {
    pub fn new(port: u16, protocol: Protocol) -> Self {
        Socket { port, protocol }
    }
    pub fn format_protocol(&self) -> String {
        match self.protocol {
            Protocol::Other(_) | Protocol::All => format!("{}", self.port),
            _ => {
                let f: String = self.protocol.clone().into();
                format!("{}/{}", self.port, f)
            }
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

        let prot = match protocol.to_lowercase().as_str() {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            x if x.len() > 0 => Protocol::Other(x.to_string()),
            _ => Protocol::All,
        };

        Ok(Socket::new(port, prot))
    }
}
