use std::io::ErrorKind;

use super::DockerError;

#[derive(Debug, PartialEq, Clone)]
pub struct Socket {
    pub port: u16,
    pub protocol: String,
}

impl Socket {
    fn new<T: Into<u16>, U: Into<String>>(port: T, protocol: U) -> Self {
        Socket {
            port: port.into(),
            protocol: protocol.into(),
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
        Ok(Socket::new(port, protocol))
    }
}
