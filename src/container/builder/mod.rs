mod socket_map;
use std::{collections::HashMap, ffi::OsString, process::Command};

use socket_map::SocketMap;

use super::{image::Image, socket::Socket};
#[derive(Debug, Clone)]
pub struct ContainerCommandBuilder {
    detatched: bool,
    image: Option<Image>,
    sockets: BoundSockets,
    environment: HashMap<String, String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundSockets {
    Dynamic,
    Static(Vec<SocketMap>),
}

impl ContainerCommandBuilder {
    pub fn new() -> Self {
        Self {
            detatched: false,
            sockets: BoundSockets::Dynamic,
            image: None,
            environment: HashMap::new(),
        }
    }

    pub fn detached(mut self) -> Self {
        self.detatched = true;
        self
    }

    pub fn with_selfbound_socket(self, socket: Socket) -> Self {
        self.with_socket(SocketMap::self_bound(socket))
    }

    pub fn with_socket(mut self, socket: SocketMap) -> Self {
        match self.sockets {
            BoundSockets::Dynamic => {
                self.sockets = BoundSockets::Static(vec![socket]);
            }
            BoundSockets::Static(ref mut sockets) => sockets.push(socket),
        };
        self
    }

    pub fn with_image(mut self, image: Image) -> Self {
        self.image = Some(image);
        self
    }

    pub fn with_environment_variable<I: Into<String>, O: Into<String>>(
        mut self,
        key: I,
        value: O,
    ) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn build(&self) -> Command {
        //add base command
        let mut command = Command::new("docker");
        command.arg("run");

        //add environment
        for (k, v) in &self.environment {
            command.args(["-e", k, "=", v]);
        }

        match &self.sockets {
            BoundSockets::Dynamic => {
                command.arg("-P");
            }
            BoundSockets::Static(sockets) => {
                for s in sockets {
                    let formatted: String = s.clone().into();
                    command.args(["-p", &formatted]);
                }
            }
        };
        //add sockets

        //detach
        if self.detatched {
            command.arg("-d");
        }

        let image: OsString = self.image.clone().unwrap().into();
        command.arg(image);
        command
    }
}
#[cfg(test)]
mod tests {

    use crate::container::{image::Image, socket::Socket, DockerError};

    use super::{socket_map::SocketMap, BoundSockets, ContainerCommandBuilder};

    #[test]
    fn constructor_test() {
        let builder = ContainerCommandBuilder::new();
        assert_eq!(builder.image, None);
        assert!(builder.sockets == BoundSockets::Dynamic);
    }
    #[test]
    fn image_test() {
        let builder = ContainerCommandBuilder::new().with_image(Image::new_latest("redis"));
        assert!(builder
            .image
            .is_some_and(|image| { image.name == "redis" && image.version == "latest" }));
    }

    #[test]
    fn socket_test() {
        let builder = ContainerCommandBuilder::new().with_socket(SocketMap::self_bound(
            Socket::new(8080u16, Some("TCP".into())),
        ));

        if let BoundSockets::Static(socks) = builder.sockets {
            assert!(socks.first().is_some_and(
                |s| s.inner_socket.port == 8080u16 && s.inner_socket.protocol.is_some()
            ))
        } else {
            assert!(false)
        }
    }

    #[test]
    fn detached_test() {
        let builder = ContainerCommandBuilder::new();
        assert!(!builder.detatched);
        let builder_detached = builder.detached();
        assert!(builder_detached.detatched);
    }

    //#[test]
    fn build_test() {
        let mut command = ContainerCommandBuilder::new()
            .with_selfbound_socket(Socket::new(6379, None))
            .with_image(Image::new_with_version("redis", "7.4.0-alpine"))
            .detached()
            .build();
        print!("{:#?}", command);
        let output = command
            .output()
            .map_err(|e| DockerError::new(e.kind(), e.to_string()));
        assert!(output.is_ok())
    }
}
