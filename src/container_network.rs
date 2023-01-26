use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HostConfig {
    #[serde(rename(deserialize = "NetworkMode"), default)]
    pub network_mode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkSettingsNet {
    #[serde(rename(deserialize = "NetworkID"), default)]
    pub network_id: String,
    #[serde(rename(deserialize = "EndpointID"), default)]
    pub endpoint_id: String,
    #[serde(rename(deserialize = "Gateway"), default)]
    pub gateway: String,
    #[serde(rename(deserialize = "IPAddress"), default)]
    pub ip_address: String,
    #[serde(rename(deserialize = "IPPrefixLen"), default)]
    pub ip_prefix_len: u8,
    #[serde(rename(deserialize = "IPv6Gateway"), default)]
    pub ipv6_gateway: String,
    #[serde(rename(deserialize = "GlobalIPv6Address"), default)]
    pub global_ipv6_address: String,
    #[serde(rename(deserialize = "GlobalIPv6PrefixLen"), default)]
    pub global_ipv6_prefix_len: u8,
    #[serde(rename(deserialize = "MacAddress"), default)]
    pub mac_address: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NetworkSettings {
    #[serde(rename(deserialize = "Networks"), default)]
    pub networks: HashMap<String, NetworkSettingsNet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkingConfig {
    #[serde(rename(deserialize = "EndpointsConfig"))]
    pub endpoints_config: EndpointsConfig,
}

impl Default for NetworkingConfig {
    fn default() -> Self {
        NetworkingConfig {
            endpoints_config: EndpointsConfig {
                isolated_nw: IsolatedNw {
                    ipam_config: IPAMConfig {
                        link_local_ips: Vec::new(),
                        ipv4_address: String::new(),
                        ipv6_address: String::new(),
                    },
                    links: Vec::new(),
                    aliases: Vec::new(),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointsConfig {
    #[serde(rename(deserialize = "isolated_nw"))]
    pub isolated_nw: IsolatedNw,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolatedNw {
    #[serde(rename(deserialize = "IPAMConfig"))]
    pub ipam_config: IPAMConfig,
    #[serde(rename(deserialize = "Links"), default)]
    pub links: Vec<String>,
    #[serde(rename(deserialize = "Aliases"), default)]
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IPAMConfig {
    #[serde(rename(deserialize = "IPv4Address"), default)]
    pub ipv4_address: String,
    #[serde(rename(deserialize = "IPv6Address"), default)]
    pub ipv6_address: String,
    #[serde(rename(deserialize = "LinkLocalIPs"), default)]
    pub link_local_ips: Vec<String>,
}