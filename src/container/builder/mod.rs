mod action;
mod socket_map;
use std::{collections::HashMap, ffi::OsString, process::Command};

use action::CreateAction;
use socket_map::SocketMap;

use super::{image::Image, socket::Socket, DockerError};
#[derive(Debug, Clone)]
pub struct ContainerCommandBuilder {
    name: Option<String>,
    detatched: bool,
    image: Option<Image>,
    sockets: BoundSockets,
    environment: HashMap<String, String>,
    action: CreateAction,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundSockets {
    Dynamic,
    Static(Vec<SocketMap>),
}

impl ContainerCommandBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            detatched: false,
            sockets: BoundSockets::Dynamic,
            image: None,
            environment: HashMap::new(),
            action: CreateAction::Create,
        }
    }

    pub fn detached(mut self) -> Self {
        self.detatched = true;
        self
    }

    pub fn selfbound_socket(self, socket: Socket) -> Self {
        self.socket(SocketMap::self_bound(socket))
    }

    pub fn socket(mut self, socket: SocketMap) -> Self {
        match self.sockets {
            BoundSockets::Dynamic => {
                self.sockets = BoundSockets::Static(vec![socket]);
            }
            BoundSockets::Static(ref mut sockets) => sockets.push(socket),
        };
        self
    }

    pub fn dynamic_socket(mut self) -> Self {
        self.sockets = BoundSockets::Dynamic;
        self
    }

    pub fn image(mut self, image: Image) -> Self {
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

    pub fn named<I: Into<String>>(mut self, name: I) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn action(mut self, action: CreateAction) -> Self {
        self.action = action;
        self
    }

    pub fn build(&self) -> Result<Command, DockerError> {
        if self.detatched && self.action == CreateAction::Create {
            return Err(DockerError::new(
                std::io::ErrorKind::InvalidData,
                "can't create a detached container",
            ));
        }

        //add base command
        let mut command = Command::new("docker");
        let action: String = self.action.into();
        command.arg(action);
        if let Some(name) = &self.name {
            command.args(["--name", &name]);
        }

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
        Ok(command)
    }
}

impl Default for ContainerCommandBuilder {
    fn default() -> Self {
        ContainerCommandBuilder::new()
    }
}
#[cfg(test)]
mod tests {

    use crate::container::{
        builder::CreateAction,
        image::Image,
        socket::{protocol::Protocol, Socket},
        DockerError,
    };

    use super::{socket_map::SocketMap, BoundSockets, ContainerCommandBuilder};

    #[test]
    fn constructor_test() {
        let builder = ContainerCommandBuilder::new();
        assert_eq!(builder.image, None);
        assert!(builder.sockets == BoundSockets::Dynamic);
        assert_eq!(builder.environment.len(), 0);
        assert!(!builder.detatched);
    }
    #[test]
    fn image_test() {
        let builder = ContainerCommandBuilder::new().image(Image::new_latest("redis"));
        assert!(builder
            .image
            .is_some_and(|i| i == Image::Latest("redis".into())));
    }

    #[test]
    fn socket_test() {
        let builder = ContainerCommandBuilder::new()
            .socket(SocketMap::self_bound(Socket::new(8080u16, Protocol::TCP)));

        if let BoundSockets::Static(socks) = builder.sockets {
            assert!(socks.first().is_some_and(
                |s| s.inner_socket.port == 8080u16 && s.inner_socket.protocol == Protocol::TCP
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

    #[test]
    fn build_test() {
        let command = ContainerCommandBuilder::new()
            .action(CreateAction::Run)
            .selfbound_socket(Socket::new(6379, Protocol::All))
            .image(Image::new_with_version("redis", "7.4.0-alpine"))
            .detached()
            .named("Redis")
            .build();
        print!("{:#?}", command);
        let output = command
            .ok()
            .unwrap()
            .output()
            .map_err(|e| DockerError::new(e.kind(), e.to_string()));
        assert!(output.is_ok())
    }
}
