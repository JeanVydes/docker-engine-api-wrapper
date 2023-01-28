use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::container_network::{HostConfig, NetworkingConfig};

/// CreateContainerFrom is the struct that is used to create a container
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateContainerFrom {
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none", default)]
    pub hostname: Option<String>,
    #[serde(rename = "Domainname", skip_serializing_if = "Option::is_none", default)]
    pub domainname: Option<String>,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none", default)]
    pub user: Option<String>,
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none", default)]
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none", default)]
    pub attach_stdout: Option<bool>,
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none", default)]
    pub attach_stderr: Option<bool>,
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none", default)]
    pub tty: Option<bool>,
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none",default)]
    pub open_stdin: Option<bool>,
    #[serde(rename = "StdinOnce", skip_serializing_if = "Option::is_none", default)]
    pub stdin_once: Option<bool>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none", default)]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none", default)]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none", default)]
    pub entrypoint: Option<String>,
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none", default)]
    pub image: Option<String>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none", default)]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none", default)]
    pub volumes: Option<HashMap<String, ()>>,
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none", default)]
    pub working_dir: Option<String>,
    #[serde(rename = "NetworkDisabled", skip_serializing_if = "Option::is_none", default)]
    pub network_disabled: Option<bool>,
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none", default)]
    pub mac_address: Option<String>,
    #[serde(rename = "ExposedPorts", skip_serializing_if = "Option::is_none", default)]
    pub exposed_ports: Option<HashMap<String, ()>>,
    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none", default)]
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout", skip_serializing_if = "Option::is_none", default)]
    pub stop_timeout: Option<i32>,
    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none", default)]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "NetworkingConfig", skip_serializing_if = "Option::is_none", default)]
    pub networking_config: Option<NetworkingConfig>,
}

impl Default for CreateContainerFrom {
    fn default() -> Self {
        CreateContainerFrom {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            cmd: None,
            entrypoint: None,
            image: None,
            labels: None,
            volumes: None,
            working_dir: None,
            network_disabled: None,
            mac_address: None,
            exposed_ports: None,
            stop_signal: None,
            stop_timeout: None,
            host_config: None,
            networking_config: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateContainerResponseFromAPI {
    #[serde(rename = "Id", default)]
    pub id: String,
    #[serde(rename = "Warnings", default)]
    pub warnings: Vec<String>,
}

impl Default for CreateContainerResponseFromAPI {
    fn default() -> Self {
        CreateContainerResponseFromAPI {
            id: String::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyMap {}

// Path: docker-engine-api/src/containerstructs.rs