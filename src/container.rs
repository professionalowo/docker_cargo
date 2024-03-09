use std::{io::Error, process::Command};

#[derive(Debug,PartialEq)]
pub struct Container {
    pub container: ContainerData,
    pub status: ConatinerStatus,
    pub socket: Option<String>,
}
#[derive(Debug, PartialEq)]
pub struct ContainerData {
    pub id: String,
    pub image: String,
    pub entrypoint: String,
    pub created: String,
    pub name: String,
}
#[derive(Debug,PartialEq)]
pub enum ConatinerStatus {
    Created,
    Running,
    Stopped,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
}

impl Container {
    pub fn try_start(&self) -> Result<(), std::io::Error> {
        if self.status == ConatinerStatus::Running {
            return Ok(());
        }
        let output = Command::new("docker")
            .args(&["start", &self.container.id])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start container",
            ))
        }
    }

    pub fn try_start_by_id_or_name(name:&str) -> Result<(), std::io::Error> {
        let output = Command::new("docker")
            .args(&["start", name])
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start container",
            ))
        }
    }
}

impl TryFrom<String> for Container {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Implementation goes here
        let values: Vec<&str> = value.split(';').collect();
        if values.len() != 6 {
            return Err(std::io::Error::new(
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
            "running" | "up" => ConatinerStatus::Running,
            "stopped" => ConatinerStatus::Stopped,
            "paused" => ConatinerStatus::Paused,
            "restarting" => ConatinerStatus::Restarting,
            "removing" => ConatinerStatus::Removing,
            "exited" => ConatinerStatus::Exited,
            "dead" => ConatinerStatus::Dead,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid status",
                ))
            }
        };
        let socket: Option<String> = None;
        Ok(Container {
            container,
            status,
            socket,
        })
    }
}

pub fn get_all_containers() -> Result<Vec<Container>, Error> {
    let output = Command::new("docker")
        .args(&[
            "ps",
            "-a",
            "--format",
            "{{.ID}};{{.Image}};{{.Command}};{{.CreatedAt}};{{.Names}};{{.Status}}",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let containers: Vec<&str> = stdout.split('\n').collect();
    let container_strings: Vec<String> = containers.iter().map(|&c| c.to_string()).collect();
    let containers: Vec<Container> = container_strings
        .iter()
        .filter_map(|c| Container::try_from(c.to_string()).ok())
        .collect();
    Ok(containers)
}
