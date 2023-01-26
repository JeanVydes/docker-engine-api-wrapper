use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::container_network::{HostConfig, NetworkSettings};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    #[serde(rename(deserialize = "IP"), default)]
    pub ip: String,
    #[serde(rename(deserialize = "PrivatePort"), default)]
    pub private_port: u16,
    #[serde(rename(deserialize = "PublicPort"), default)]
    pub public_port: u16,
    #[serde(rename(deserialize = "Type"), default)]
    pub _type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Mount {
    #[serde(rename(deserialize = "Type"), default)]
    pub _type: String,
    #[serde(rename(deserialize = "Name"), default)]
    pub name: String,
    #[serde(rename(deserialize = "Source"), default)]
    pub source: String,
    #[serde(rename(deserialize = "Destination"), default)]
    pub destination: String,
    #[serde(rename(deserialize = "Driver"), default)]
    pub driver: String,
    #[serde(rename(deserialize = "Mode"), default)]
    pub mode: String,
    #[serde(rename(deserialize = "RW"), default)]
    pub rw: bool,
    #[serde(rename(deserialize = "Propagation"), default)]
    pub propagation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    #[serde(rename(deserialize = "Id"), default)]
    pub id: String,
    #[serde(rename(deserialize = "Names"), default)]
    pub names: Vec<String>,
    #[serde(rename(deserialize = "Image"), default)]
    pub image: String,
    #[serde(rename(deserialize = "ImageID"), default)]
    pub image_id: String,
    #[serde(rename(deserialize = "Command"), default)]
    pub command: String,
    #[serde(rename(deserialize = "Created"), default)]
    pub created: u64,
    #[serde(rename(deserialize = "State"), default)]
    pub state: String,
    #[serde(rename(deserialize = "Status"), default)]
    pub status: String,
    #[serde(rename(deserialize = "Ports"), default)]
    pub ports: Vec<Port>,
    #[serde(rename(deserialize = "Labels"), default)]
    pub labels: HashMap<String, String>,
    #[serde(rename(deserialize = "SizeRw"), default)]
    pub size_rw: Option<u64>,
    #[serde(rename(deserialize = "SizeRootFs"), default)]
    pub size_root_fs: Option<u64>,
    #[serde(rename(deserialize = "HostConfig"), default)]
    pub host_config: HostConfig,
    #[serde(rename(deserialize = "NetworkSettings"), default)]
    pub network_settings: NetworkSettings,
    #[serde(rename(deserialize = "Mounts"), default)]
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