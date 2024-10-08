pub mod builder;
pub mod image;
pub mod socket;
use std::io::ErrorKind;
use std::{io::Error, process::Command};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct DockerError {
    pub kind: ErrorKind,
    pub message: String,
}

impl DockerError {
    fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        DockerError {
            kind,
            message: message.into(),
        }
    }
}

impl From<Error> for DockerError {
    fn from(e: Error) -> Self {
        DockerError::new(e.kind(), e.to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Container {
    pub container: ContainerData,
    pub status: ConatinerStatus,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContainerData {
    pub id: String,
    pub image: String,
    pub entrypoint: String,
    pub created: String,
    pub name: String,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConatinerStatus {
    Created,
    Running(String),
    Stopped,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
}

impl Container {
    pub fn try_start(&self) -> Result<(), DockerError> {
        if let ConatinerStatus::Running(_) = &self.status {
            return Ok(());
        }
        let output = Command::new("docker")
            .args(&["start", &self.container.id])
            .output()
            .unwrap();
        if output.status.success() {
            Ok(())
        } else {
            Err(DockerError::new(
                std::io::ErrorKind::Other,
                "Failed to start container",
            ))
        }
    }

    pub fn try_stop(&self) -> Result<(), DockerError> {
        if self.status == ConatinerStatus::Stopped {
            return Ok(());
        }
        let output = Command::new("docker")
            .args(&["stop", &self.container.id])
            .output()
            .map_err(|e| DockerError::new(e.kind(), e.to_string()))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(DockerError::new(
                std::io::ErrorKind::Other,
                "Failed to stop container",
            ))
        }
    }

    pub fn try_stop_by_id_or_name(name: &str) -> Result<(), DockerError> {
        let output = Command::new("docker")
            .args(&["stop", name])
            .output()
            .map_err(|e| {
                DockerError::new(
                    e.kind(),
                    format!("Failed to start container: {}", e.to_string()),
                )
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(DockerError::new(
                std::io::ErrorKind::Other,
                "Failed to stop container",
            ))
        }
    }

    pub fn try_start_by_id_or_name(name: &str) -> Result<(), DockerError> {
        let output = Command::new("docker")
            .args(&["start", name])
            .output()
            .map_err(|e| {
                DockerError::new(
                    e.kind(),
                    format!("Failed to start container: {}", e.to_string()),
                )
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(DockerError::new(
                std::io::ErrorKind::Other,
                "Failed to start container",
            ))
        }
    }

    pub fn try_get_by_id_or_name(name: &str) -> Result<Self, DockerError> {
        match get_all_containers() {
            Ok(containers) => {
                let container = containers
                    .iter()
                    .find(|c| c.container.id == name || c.container.name == name);
                match container {
                    Some(c) => Ok(c.clone()),
                    None => Err(DockerError::new(
                        std::io::ErrorKind::Other,
                        "Container not found",
                    )),
                }
            }
            Err(e) => Err(DockerError::new(e.kind(), e.to_string())),
        }
    }
}

impl TryFrom<String> for Container {
    type Error = DockerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values: Vec<&str> = value.split(';').collect();
        if values.len() < 6 {
            return Err(DockerError::new(
                std::io::ErrorKind::InvalidInput,
                "Too few arguments",
            ));
        }
        let container = ContainerData {
            id: values[0].to_string(),
            image: values[1].to_string(),
            entrypoint: values[2].to_string(),
            created: values[3].to_string(),
            name: values[4].to_string(),
        };

        let status = match values[5].split(' ').next().unwrap().to_lowercase().as_str() {
            "created" => ConatinerStatus::Created,
            "running" | "up" => ConatinerStatus::Running(values[6].into()),
            "stopped" => ConatinerStatus::Stopped,
            "paused" => ConatinerStatus::Paused,
            "restarting" => ConatinerStatus::Restarting,
            "removing" => ConatinerStatus::Removing,
            "exited" => ConatinerStatus::Exited,
            "dead" => ConatinerStatus::Dead,
            _ => {
                return Err(DockerError::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid status",
                ))
            }
        };

        Ok(Container { container, status })
    }
}

pub fn get_all_containers() -> Result<Vec<Container>, Error> {
    let output = Command::new("docker")
        .args(&[
            "ps",
            "-a",
            "--format",
            "{{.ID}};{{.Image}};{{.Command}};{{.CreatedAt}};{{.Names}};{{.Status}};{{split .Ports \",\"}}",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let containers: Vec<&str> = stdout.split('\n').collect();
    let containers: Vec<Container> = containers
        .iter()
        .filter_map(|c| Container::try_from(c.to_string()).ok())
        .collect();
    Ok(containers)
}
