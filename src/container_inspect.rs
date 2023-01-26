use std::{collections::HashMap};

use serde::{Serialize, Deserialize};

use crate::{container_structs::Mount, container_network::NetworkSettingsNet};

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectedContainer {
    #[serde(rename(deserialize = "AppArmorProfile"), default)]
    pub app_armor_profile: String,
    #[serde(rename(deserialize = "Args"), default)]
    pub args: Vec<String>,
    #[serde(rename(deserialize = "Config"))]
    pub config: Config,
    #[serde(rename(deserialize = "Created"), default)]
    pub created: String,
    #[serde(rename(deserialize = "Driver"), default)]
    pub driver: String,
    #[serde(rename(deserialize = "ExecIDs"), default)]
    pub exec_ids: Vec<String>,
    #[serde(rename(deserialize = "HostConfig"), default)]
    pub host_config: HostConfig,
    #[serde(rename(deserialize = "HostnamePath"), default)]
    pub hostname_path: String,
    #[serde(rename(deserialize = "HostsPath"), default)]
    pub hosts_path: String,
    #[serde(rename(deserialize = "Id"), default)]
    pub id: String,
    #[serde(rename(deserialize = "Image"), default)]
    pub image: String,
    #[serde(rename(deserialize = "LogPath"), default)]
    pub log_path: String,
    #[serde(rename(deserialize = "MountLabel"), default)]
    pub mount_label: String,
    #[serde(rename(deserialize = "Mounts"), default)]
    pub mounts: Vec<Mount>,
    #[serde(rename(deserialize = "Name"), default)]
    pub name: String,
    #[serde(rename(deserialize = "NetworkSettings"))]
    pub network_settings: NetworkSettingsNet,
    #[serde(rename(deserialize = "Path"), default)]
    pub path: String,
    #[serde(rename(deserialize = "ProcessLabel"), default)]
    pub process_label: String,
    #[serde(rename(deserialize = "ResolvConfPath"), default)]
    pub resolv_conf_path: String,
    #[serde(rename(deserialize = "RestartCount"), default)]
    pub restart_count: u64,
    #[serde(rename(deserialize = "State"))]
    pub state: State,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename(deserialize = "AttachStderr"))]
    pub attach_stderr: bool,
    #[serde(rename(deserialize = "AttachStdin"))]
    pub attach_stdin: bool,
    #[serde(rename(deserialize = "AttachStdout"))]
    pub attach_stdout: bool,
    #[serde(rename(deserialize = "Cmd"), default)]
    pub cmd: Vec<String>,
    #[serde(rename(deserialize = "Domainname"), default)]
    pub domainname: String,
    #[serde(rename(deserialize = "Entrypoint"), default)]
    pub entrypoint: Vec<String>,
    #[serde(rename(deserialize = "Env"), default)]
    pub env: Vec<String>,
    #[serde(rename(deserialize = "ExposedPorts"), default)]
    pub exposed_ports: Vec<String>,
    #[serde(rename(deserialize = "Hostname"), default)]
    pub hostname: String,
    #[serde(rename(deserialize = "Image"), default)]
    pub image: String,
    #[serde(rename(deserialize = "Labels"), default)]
    pub labels: Vec<String>,
    #[serde(rename(deserialize = "OnBuild"), default)]
    pub on_build: Vec<String>,
    #[serde(rename(deserialize = "OpenStdin"))]
    pub open_stdin: bool,
    #[serde(rename(deserialize = "StdinOnce"))]
    pub stdin_once: bool,
    #[serde(rename(deserialize = "Tty"))]
    pub tty: bool,
    #[serde(rename(deserialize = "User"), default)]
    pub user: String,
    #[serde(rename(deserialize = "Volumes"), default)]
    pub volumes: Vec<String>,
    #[serde(rename(deserialize = "WorkingDir"), default)]
    pub working_dir: String,
    #[serde(rename(deserialize = "StopSignal"), default)]
    pub stop_signal: String,
    #[serde(rename(deserialize = "StopTimeout"), default)]
    pub stop_timeout: u64,
    #[serde(rename(deserialize = "HealthCheck"), default)]
    pub health_check: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename(deserialize = "Error"), default)]
    pub error: String,
    #[serde(rename(deserialize = "ExitCode"), default)]
    pub exit_code: u64,
    #[serde(rename(deserialize = "FinishedAt"), default)]
    pub finished_at: String,
    #[serde(rename(deserialize = "OOMKilled"))]
    pub oom_killed: bool,
    #[serde(rename(deserialize = "Paused"))]
    pub paused: bool,
    #[serde(rename(deserialize = "Pid"), default)]
    pub pid: u64,
    #[serde(rename(deserialize = "Restarting"))]
    pub restarting: bool,
    #[serde(rename(deserialize = "Running"))]
    pub running: bool,
    #[serde(rename(deserialize = "StartedAt"), default)]
    pub started_at: String,
    #[serde(rename(deserialize = "Status"), default)]
    pub status: StateHealth,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StateHealth {
    #[serde(rename(deserialize = "FailingStreak"), default)]
    pub failing_streak: u64,
    #[serde(rename(deserialize = "Log"), default)]
    pub log: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename(deserialize = "Status"), default)]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HostConfig {
    #[serde(rename(deserialize = "MaximumIops"), default)] 
    pub maximum_iops: i64,
    #[serde(rename(deserialize = "MaximumIOBps"), default)]
    pub maximum_io_bps: i64,
    #[serde(rename(deserialize = "BlkioWeight"), default)]
    pub blkio_weight: i16,
    #[serde(rename(deserialize = "BlkioWeightDevice"), default)]
    pub blkio_weight_device: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "BlkioDeviceReadBps"), default)]
    pub blkio_device_read_bps: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "BlkioDeviceWriteBps"), default)]
    pub blkio_device_write_bps: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "BlkioDeviceReadIOps"), default)]
    pub blkio_device_read_iops: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "BlkioDeviceWriteIOps"), default)]
    pub blkio_device_write_iops: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "ContainerIDFile"), default)]
    pub container_id_file: String,
    #[serde(rename(deserialize = "CpusetCpus"), default)]
    pub cpuset_cpus: String,
    #[serde(rename(deserialize = "CpusetMems"), default)]
    pub cpuset_mems: String,
    #[serde(rename(deserialize = "CpuPercent"), default)]
    pub cpu_percent: u64,
    #[serde(rename(deserialize = "CpuShares"), default)]
    pub cpu_shares: u64,
    #[serde(rename(deserialize = "CpuPeriod"), default)]
    pub cpu_period: i64,
    #[serde(rename(deserialize = "CpuRealtimePeriod"), default)]
    pub cpu_realtime_period: i64,
    #[serde(rename(deserialize = "CpuRealtimeRuntime"), default)]
    pub cpu_realtime_runtime: i64,
    #[serde(rename(deserialize = "Devices"), default)]
    pub devices: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename(deserialize = "DeviceRequests"), default)]
    pub device_requests: Vec<DeviceRequest>,
    #[serde(rename(deserialize = "IpcMode"), default)]
    pub ipc_mode: String,
    #[serde(rename(deserialize = "Memory"), default)]
    pub memory: i64,
    #[serde(rename(deserialize = "MemorySwap"), default)]
    pub memory_swap: i64,
    #[serde(rename(deserialize = "MemoryReservation"), default)]
    pub memory_reservation: i64,
    #[serde(rename(deserialize = "KernelMemory"), default)]
    pub kernel_memory: i64,
    #[serde(rename(deserialize = "OomKillDisable"), default)]
    pub oom_kill_disable: bool,
    #[serde(rename(deserialize = "OomScoreAdj"), default)]
    pub oom_score_adj: i16,
    #[serde(rename(deserialize = "NetworkMode"), default)]
    pub network_mode: String,
    #[serde(rename(deserialize = "PidMode"), default)]
    pub pid_mode: String,
    #[serde(rename(deserialize = "PortBindings"), default)]
    pub port_bindings: HashMap<String, Vec<HashMap<String, String>>>,
    #[serde(rename(deserialize = "Privileged"), default)]
    pub privileged: bool,
    #[serde(rename(deserialize = "ReadonlyRootfs"), default)]
    pub readonly_rootfs: bool,
    #[serde(rename(deserialize = "PublishAllPorts"), default)]
    pub publish_all_ports: bool,
    #[serde(rename(deserialize = "RestartPolicy"), default)]
    pub restart_policy: HashMap<String, String>,
    #[serde(rename(deserialize = "LogConfig"), default)]
    pub log_config: HashMap<String, String>,
    #[serde(rename(deserialize = "Sysctls"), default)]
    pub sysctls: HashMap<String, String>,
    #[serde(rename(deserialize = "Ulimits"), default)]
    pub ulimits: Vec<HashMap<String, String>>,
    #[serde(rename(deserialize = "VolumeDriver"), default)]
    pub volume_driver: String,
    #[serde(rename(deserialize = "ShmSize"), default)]
    pub shm_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceRequest {
    #[serde(rename(deserialize = "Driver"), default)]
    pub driver: String,
    #[serde(rename(deserialize = "Count"), default)]
    pub count: u64,
    #[serde(rename(deserialize = "DeviceIDs"), default)]
    pub device_ids: Vec<String>,
    #[serde(rename(deserialize = "Capabilities"), default)]
    pub capabilities: Vec<Vec<String>>,
    #[serde(rename(deserialize = "Options"), default)]
    pub options: HashMap<String, String>,
}