use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::container_network::{HostConfig, NetworkingConfig};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateContainerBody<T> {
    #[serde(rename(deserialize = "Hostname"), default)]
    pub hostname: String,
    #[serde(rename(deserialize = "Domainname"), default)]
    pub domainname: String,
    #[serde(rename(deserialize = "User"), default)]
    pub user: String,
    #[serde(rename(deserialize = "AttachStdin"), default)]
    pub attach_stdin: bool,
    #[serde(rename(deserialize = "AttachStdout"), default)]
    pub attach_stdout: bool,
    #[serde(rename(deserialize = "AttachStderr"), default)]
    pub attach_stderr: bool,
    #[serde(rename(deserialize = "Tty"), default)]
    pub tty: bool,
    #[serde(rename(deserialize = "OpenStdin"), default)]
    pub open_stdin: bool,
    #[serde(rename(deserialize = "StdinOnce"), default)]
    pub stdin_once: bool,
    #[serde(rename(deserialize = "Env"), default)]
    pub env: Vec<String>,
    #[serde(rename(deserialize = "Cmd"), default)]
    pub cmd: Vec<String>,
    #[serde(rename(deserialize = "Entrypoint"), default)]
    pub entrypoint: String,
    #[serde(rename(deserialize = "Image"), default)]
    pub image: String,
    #[serde(rename(deserialize = "Labels"), default)]
    pub labels: HashMap<String, String>,
    #[serde(rename(deserialize = "Volumes"), default)]
    pub volumes: HashMap<String, T>,
    #[serde(rename(deserialize = "WorkingDir"), default)]
    pub working_dir: String,
    #[serde(rename(deserialize = "NetworkDisabled"), default)]
    pub network_disabled: bool,
    #[serde(rename(deserialize = "MacAddress"), default)]
    pub mac_address: String,
    #[serde(rename(deserialize = "ExposedPorts"), default)]
    pub exposed_ports: HashMap<String, T>,
    #[serde(rename(deserialize = "StopSignal"), default)]
    pub stop_signal: String,
    #[serde(rename(deserialize = "StopTimeout"), default)]
    pub stop_timeout: i32,
    #[serde(rename(deserialize = "HostConfig"), default)]
    pub host_config: HostConfig,
    #[serde(rename(deserialize = "NetworkingConfig"), default)]
    pub networking_config: NetworkingConfig,
}

impl Default for CreateContainerBody<String> {
    fn default() -> Self {
        CreateContainerBody {
            hostname: String::new(),
            domainname: String::new(),
            user: String::new(),
            attach_stdin: false,
            attach_stdout: false,
            attach_stderr: false,
            tty: false,
            open_stdin: false,
            stdin_once: false,
            env: Vec::new(),
            cmd: Vec::new(),
            entrypoint: String::new(),
            image: String::new(),
            labels: HashMap::new(),
            volumes: HashMap::new(),
            working_dir: String::new(),
            network_disabled: false,
            mac_address: String::new(),
            exposed_ports: HashMap::new(),
            stop_signal: String::new(),
            stop_timeout: 0,
            host_config: HostConfig::default(),
            networking_config: NetworkingConfig::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateContainerResponse {
    #[serde(rename(deserialize = "Id"), default)]
    pub id: String,
    #[serde(rename(deserialize = "Warnings"), default)]
    pub warnings: Vec<String>,
}

impl Default for CreateContainerResponse {
    fn default() -> Self {
        CreateContainerResponse {
            id: String::new(),
            warnings: Vec::new(),
        }
    }
}

// Path: docker-engine-api/src/containerstructs.rs