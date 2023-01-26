use hyper::Method;
use hyper::body::Bytes;
use crate::container_structs::Container;
use crate::container_create::CreateContainerResponse;
use crate::container_procceses::ContainerProcessesResponse;
use crate::container_stats::Stats;

pub type NoImplementedYet = Bytes;

pub const LIST_CONTAINERS: &str = "/containers/json";
pub const LIST_CONTAINERS_METHOD: Method = Method::GET;
pub type LIST_CONTAINERS_RETURN = Vec<Container>;

pub const CREATE_CONTAINER: &str = "/containers/create";
pub const CREATE_CONTAINER_METHOD: Method = Method::POST;
pub type CREATE_CONTAINER_RETURN = CreateContainerResponse;

pub const INSPECT_CONTAINER_START: &str = "/containers/";
pub const INSPECT_CONTAINER_END: &str = "/json";
pub const INSPECT_CONTAINER_METHOD: Method = Method::GET;

pub const LIST_PROCESSES_START: &str = "/containers/";
pub const LIST_PROCESSES_END: &str = "/top";
pub const LIST_PROCESSES_METHOD: Method = Method::GET;
pub type LIST_PROCESSES_RETURN = ContainerProcessesResponse;

pub const GET_CONTAINER_LOGS_START: &str = "/containers/";
pub const GET_CONTAINER_LOGS_END: &str = "/logs";
pub const GET_CONTAINER_LOGS_METHOD: Method = Method::GET;
pub type GET_CONTAINER_LOGS_RETURN = String;

pub const GET_CHANGES_CONTAINER: &str = "/containers/{id}/changes";
pub const EXPORT_CONTAINER: &str = "/containers/{id}/export";

pub const GET_STATS_CONTAINER_START: &str = "/containers/";
pub const GET_STATS_CONTAINER_END: &str = "/stats";
pub const GET_STATS_CONTAINER_METHOD: Method = Method::GET;
pub type GET_STATS_CONTAINER_RETURN = Stats;

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

pub const UPDATE_CONTAINER: &str = "/containers/{id}/update";
pub const RENAME_CONTAINER: &str = "/containers/{id}/rename";
pub const PAUSE_CONTAINER: &str = "/containers/{id}/pause";
pub const UNPAUSE_CONTAINER: &str = "/containers/{id}/unpause";
pub const ATTACH_CONTAINER: &str = "/containers/{id}/attach";
pub const WAIT_CONTAINER: &str = "/containers/{id}/wait";

pub const REMOVE_CONTAINER: &str = "/containers/";
pub const REMOVE_CONTAINER_METHOD: Method = Method::DELETE;

// Path: docker-engine-api/src/api.rs
