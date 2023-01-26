use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerProcessesResponse {
    #[serde(rename(deserialize = "Titles"), default)]
    pub titles: Vec<String>,
    #[serde(rename(deserialize = "Processes"), default)]
    pub processes: Vec<Vec<String>>,
}
