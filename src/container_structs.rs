use serde::{Serialize, Deserialize};
use std::{collections::HashMap};
use crate::container_network::{HostConfig, NetworkSettings};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    #[serde(rename = "IP", default)]
    pub ip: String,
    #[serde(rename = "PrivatePort", default)]
    pub private_port: u16,
    #[serde(rename = "PublicPort", default)]
    pub public_port: u16,
    #[serde(rename = "Type", default)]
    pub _type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Mount {
    #[serde(rename = "Type", default)]
    pub _type: String,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Source", default)]
    pub source: String,
    #[serde(rename = "Destination", default)]
    pub destination: String,
    #[serde(rename = "Driver", default)]
    pub driver: String,
    #[serde(rename = "Mode", default)]
    pub mode: String,
    #[serde(rename = "RW", default)]
    pub rw: bool,
    #[serde(rename = "Propagation", default)]
    pub propagation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id", default)]
    pub id: String,
    #[serde(rename = "Names", default)]
    pub names: Vec<String>,
    #[serde(rename = "Image", default)]
    pub image: String,
    #[serde(rename = "ImageID", default)]
    pub image_id: String,
    #[serde(rename = "Command", default)]
    pub command: String,
    #[serde(rename = "Created", default)]
    pub created: u64,
    #[serde(rename = "State", default)]
    pub state: String,
    #[serde(rename = "Status", default)]
    pub status: String,
    #[serde(rename = "Ports", default)]
    pub ports: Vec<Port>,
    #[serde(rename = "Labels", default)]
    pub labels: HashMap<String, String>,
    #[serde(rename = "SizeRw", default)]
    pub size_rw: Option<u64>,
    #[serde(rename = "SizeRootFs", default)]
    pub size_root_fs: Option<u64>,
    #[serde(rename = "HostConfig", default)]
    pub host_config: HostConfig,
    #[serde(rename = "NetworkSettings", default)]
    pub network_settings: NetworkSettings,
    #[serde(rename = "Mounts", default)]
    pub mounts: Vec<Mount>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            id: String::new(),
            names: Vec::new(),
            image: String::new(),
            image_id: String::new(),
            command: String::new(),
            created: 0,
            state: String::new(),
            status: String::new(),
            ports: Vec::new(),
            labels: HashMap::new(),
            size_rw: None,
            size_root_fs: None,
            host_config: HostConfig {
                network_mode: String::new(),
            },
            network_settings: NetworkSettings {
                networks: HashMap::new(),
            },
            mounts: Vec::new(),
        }
    }
}

impl Clone for Container {
    fn clone(&self) -> Self {
        Container {
            id: self.id.clone(),
            names: self.names.clone(),
            image: self.image.clone(),
            image_id: self.image_id.clone(),
            command: self.command.clone(),
            created: self.created,
            state: self.state.clone(),
            status: self.status.clone(),
            ports: self.ports.clone(),
            labels: self.labels.clone(),
            size_rw: self.size_rw.clone(),
            size_root_fs: self.size_root_fs.clone(),
            host_config: self.host_config.clone(),
            network_settings: self.network_settings.clone(),
            mounts: self.mounts.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GenericDevice {
    #[serde(rename = "PathOnHost", default)]
    pub host_path: String,
    #[serde(rename = "PathInContainer", default)]
    pub container_path: String,
    #[serde(rename = "CgroupPermissions", default)]
    pub options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WarningsResponse {
    #[serde(rename = "Warnings", default)]
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyMap {}
