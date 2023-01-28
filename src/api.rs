use hyper::Method;
use crate::container_inspect::InspectedContainer;
use crate::container_structs::Container;
use crate::container_create::CreateContainerResponseFromAPI;
use crate::container_procceses::ContainerProcessesResponse;
use crate::container_stats::Stats;
use serde::Deserialize;

//pub type NoImplementedYet = Bytes;
pub type EmptyOk = ();

pub const LIST_CONTAINERS: &str = "/containers/json";
pub const LIST_CONTAINERS_METHOD: Method = Method::GET;
pub type ListContainersReturn = Vec<Container>;

pub const CREATE_CONTAINER: &str = "/containers/create";
pub const CREATE_CONTAINER_METHOD: Method = Method::POST;
pub type CreateContainerReturn = CreateContainerResponseFromAPI;

pub const INSPECT_CONTAINER_START: &str = "/containers/";
pub const INSPECT_CONTAINER_END: &str = "/json";
pub const INSPECT_CONTAINER_METHOD: Method = Method::GET;
pub type InspectContainerReturn = InspectedContainer;

pub const LIST_PROCESSES_START: &str = "/containers/";
pub const LIST_PROCESSES_END: &str = "/top";
pub const LIST_PROCESSES_METHOD: Method = Method::GET;
pub type ListProcessesReturn = ContainerProcessesResponse;

pub const GET_CONTAINER_LOGS_START: &str = "/containers/";
pub const GET_CONTAINER_LOGS_END: &str = "/logs";
pub const GET_CONTAINER_LOGS_METHOD: Method = Method::GET;
pub type GetContainerLogsReturn = String;

//pub const GET_CHANGES_CONTAINER: &str = "/containers/{id}/changes";
//pub const EXPORT_CONTAINER: &str = "/containers/{id}/export";

pub const GET_STATS_CONTAINER_START: &str = "/containers/";
pub const GET_STATS_CONTAINER_END: &str = "/stats";
pub const GET_STATS_CONTAINER_METHOD: Method = Method::GET;
pub type GetContainerStatsReturn = Stats;
//
pub const RESIZE_CONTAINER: &str = "/containers/{id}/resize";

pub const START_CONTAINER_START: &str = "/containers/";
pub const START_CONTAINER_END: &str = "/start";
pub const START_CONTAINER_METHOD: Method = Method::POST;

pub const STOP_CONTAINER_START: &str = "/containers/";
pub const STOP_CONTAINER_END: &str = "/stop";
pub const STOP_CONTAINER_METHOD: Method = Method::POST;

pub const RESTART_CONTAINER_START: &str = "/containers/";
pub const RESTART_CONTAINER_END: &str = "/restart";
pub const RESTART_CONTAINER_METHOD: Method = Method::POST;

pub const KILL_CONTAINER_START: &str = "/containers/";
pub const KILL_CONTAINER_END: &str = "/kill";
pub const KILL_CONTAINER_METHOD: Method = Method::POST;

pub const UPDATE_CONTAINER_START: &str = "/containers/";
pub const UPDATE_CONTAINER_END: &str = "/update";
pub const UPDATE_CONTAINER_METHOD: Method = Method::POST;

pub const RENAME_CONTAINER_START: &str = "/containers/";
pub const RENAME_CONTAINER_END: &str = "/rename";
pub const RENAME_CONTAINER_METHOD: Method = Method::POST;

pub const PAUSE_CONTAINER_START: &str = "/containers/";
pub const PAUSE_CONTAINER_END: &str = "/pause";
pub const PAUSE_CONTAINER_METHOD: Method = Method::POST;

pub const UNPAUSE_CONTAINER_START: &str = "/containers/";
pub const UNPAUSE_CONTAINER_END: &str = "/unpause";
pub const UNPAUSE_CONTAINER_METHOD: Method = Method::POST;

pub const WAIT_CONTAINER_START: &str = "/containers/";
pub const WAIT_CONTAINER_END: &str = "/wait";
pub const WAIT_CONTAINER_METHOD: Method = Method::POST;

pub const REMOVE_CONTAINER: &str = "/containers/";
pub const REMOVE_CONTAINER_METHOD: Method = Method::DELETE;

pub const RESIZE_CONTAINER_TTY_START: &str = "/containers/";
pub const RESIZE_CONTAINER_TTY_END: &str = "/resize";
pub const RESIZE_CONTAINER_TTY_METHOD: Method = Method::POST;

pub const DELETE_STOPPED_CONTAINERS: &str = "/containers/prune";
pub const DELETE_STOPPED_CONTAINERS_METHOD: Method = Method::POST;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ErrorMessage {
    #[serde(rename(deserialize = "message"), default)]
    pub message: String
}

// Path: docker-engine-api/src/api.rs