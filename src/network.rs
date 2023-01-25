use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Network {
    #[serde(rename(deserialize = "Name"), default)]
    pub name: String,
    #[serde(rename(deserialize = "Id"), default)]
    pub id: String,
    #[serde(rename(deserialize = "Created"), default)]
    pub created: String,
    #[serde(rename(deserialize = "Scope"), default)]
    pub scope: String,
    #[serde(rename(deserialize = "Driver"), default)]
    pub driver: String,
    #[serde(rename(deserialize = "EnableIPv6"), default)]
    pub enable_ipv6: bool,
    #[serde(rename(deserialize = "Internal"), default)]
    pub internal: bool,
    #[serde(rename(deserialize = "Attachable"), default)]
    pub attachable: bool,
    #[serde(rename(deserialize = "Ingress"), default)]
    pub ingress: bool,
    #[serde(rename(deserialize = "IPAM"))]
    pub ipam: IPAM,
    #[serde(rename(deserialize = "Options"), default)]
    pub options: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct IPAM {
    #[serde(rename(deserialize = "Driver"), default)]
    pub driver: String,
    #[serde(rename(deserialize = "Config"), default)]
    pub config: Vec<String>,
}