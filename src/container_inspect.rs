use std::{collections::HashMap};

use serde::{Serialize, Deserialize};

use crate::{container_structs::Mount, container_network::NetworkSettingsNet};

#[derive(Serialize, Deserialize, Debug)]
pub struct InspectedContainer {
    #[serde(rename = "AppArmorProfile", default)]
    pub app_armor_profile: String,
    #[serde(rename = "Args", default)]
    pub args: Vec<String>,
    #[serde(rename = "Config", default)]
    pub config: Config,
    #[serde(rename = "Created", default)]
    pub created: String,
    #[serde(rename = "Driver", default)]
    pub driver: String,
    #[serde(rename = "ExecIDs", default)]
    pub exec_ids: Vec<String>,
    #[serde(rename = "HostConfig", default)]
    pub host_config: HostConfig,
    #[serde(rename = "HostnamePath", default)]
    pub hostname_path: String,
    #[serde(rename = "HostsPath", default)]
    pub hosts_path: String,
    #[serde(rename = "Id", default)]
    pub id: String,
    #[serde(rename = "Image", default)]
    pub image: String,
    #[serde(rename = "LogPath", default)]
    pub log_path: String,
    #[serde(rename = "MountLabel", default)]
    pub mount_label: String,
    #[serde(rename = "Mounts", default)]
    pub mounts: Vec<Mount>,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "NetworkSettings", default)]
    pub network_settings: NetworkSettingsNet,
    #[serde(rename = "Path", default)]
    pub path: String,
    #[serde(rename = "ProcessLabel", default)]
    pub process_label: String,
    #[serde(rename = "ResolvConfPath", default)]
    pub resolv_conf_path: String,
    #[serde(rename = "RestartCount", default)]
    pub restart_count: u64,
    #[serde(rename = "State", default)]
    pub state: State,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(rename = "AttachStderr", default)]
    pub attach_stderr: bool,
    #[serde(rename = "AttachStdin", default)]
    pub attach_stdin: bool,
    #[serde(rename = "AttachStdout", default)]
    pub attach_stdout: bool,
    #[serde(rename = "Cmd", default)]
    pub cmd: Vec<String>,
    #[serde(rename = "Domainname", default)]
    pub domainname: String,
    #[serde(rename = "Entrypoint", default)]
    pub entrypoint: Vec<String>,
    #[serde(rename = "Env", default)]
    pub env: Vec<String>,
    #[serde(rename = "ExposedPorts", default)]
    pub exposed_ports: Vec<String>,
    #[serde(rename = "Hostname", default)]
    pub hostname: String,
    #[serde(rename = "Image", default)]
    pub image: String,
    #[serde(rename = "Labels", default)]
    pub labels: Vec<String>,
    #[serde(rename = "OnBuild", default)]
    pub on_build: Vec<String>,
    #[serde(rename = "OpenStdin", default)]
    pub open_stdin: bool,
    #[serde(rename = "StdinOnce", default)]
    pub stdin_once: bool,
    #[serde(rename = "Tty", default)]
    pub tty: bool,
    #[serde(rename = "User", default)]
    pub user: String,
    #[serde(rename = "Volumes", default)]
    pub volumes: Vec<String>,
    #[serde(rename = "WorkingDir", default)]
    pub working_dir: String,
    #[serde(rename = "StopSignal", default)]
    pub stop_signal: String,
    #[serde(rename = "StopTimeout", default)]
    pub stop_timeout: u64,
    #[serde(rename = "HealthCheck", default)]
    pub health_check: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct State {
    #[serde(rename = "Error", default)]
    pub error: String,
    #[serde(rename = "ExitCode", default)]
    pub exit_code: u64,
    #[serde(rename = "FinishedAt", default)]
    pub finished_at: String,
    #[serde(rename = "OOMKilled", default)]
    pub oom_killed: bool,
    #[serde(rename = "Paused", default)]
    pub paused: bool,
    #[serde(rename = "Pid", default)]
    pub pid: u64,
    #[serde(rename = "Restarting", default)]
    pub restarting: bool,
    #[serde(rename = "Running", default)]
    pub running: bool,
    #[serde(rename = "StartedAt", default)]
    pub started_at: String,
    #[serde(rename = "Status", default)]
    pub status: StateHealth,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StateHealth {
    #[serde(rename = "FailingStreak", default)]
    pub failing_streak: u64,
    #[serde(rename = "Log", default)]
    pub log: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename = "Status", default)]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HostConfig {
    #[serde(rename = "MaximumIops", default)] 
    pub maximum_iops: i64,
    #[serde(rename = "MaximumIOBps", default)]
    pub maximum_io_bps: i64,
    #[serde(rename = "BlkioWeight", default)]
    pub blkio_weight: i16,
    #[serde(rename = "BlkioWeightDevice", default)]
    pub blkio_weight_device: Vec<HashMap<String, String>>,
    #[serde(rename = "BlkioDeviceReadBps", default)]
    pub blkio_device_read_bps: Vec<HashMap<String, String>>,
    #[serde(rename = "BlkioDeviceWriteBps", default)]
    pub blkio_device_write_bps: Vec<HashMap<String, String>>,
    #[serde(rename = "BlkioDeviceReadIOps", default)]
    pub blkio_device_read_iops: Vec<HashMap<String, String>>,
    #[serde(rename = "BlkioDeviceWriteIOps", default)]
    pub blkio_device_write_iops: Vec<HashMap<String, String>>,
    #[serde(rename = "ContainerIDFile", default)]
    pub container_id_file: String,
    #[serde(rename = "CpusetCpus", default)]
    pub cpuset_cpus: String,
    #[serde(rename = "CpusetMems", default)]
    pub cpuset_mems: String,
    #[serde(rename = "CpuPercent", default)]
    pub cpu_percent: u64,
    #[serde(rename = "CpuShares", default)]
    pub cpu_shares: u64,
    #[serde(rename = "CpuPeriod", default)]
    pub cpu_period: i64,
    #[serde(rename = "CpuRealtimePeriod", default)]
    pub cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime", default)]
    pub cpu_realtime_runtime: i64,
    #[serde(rename = "Devices", default)]
    pub devices: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename = "DeviceRequests", default)]
    pub device_requests: Vec<DeviceRequest>,
    #[serde(rename = "IpcMode", default)]
    pub ipc_mode: String,
    #[serde(rename = "Memory", default)]
    pub memory: i64,
    #[serde(rename = "MemorySwap", default)]
    pub memory_swap: i64,
    #[serde(rename = "MemoryReservation", default)]
    pub memory_reservation: i64,
    #[serde(rename = "KernelMemory", default)]
    pub kernel_memory: i64,
    #[serde(rename = "OomKillDisable", default)]
    pub oom_kill_disable: bool,
    #[serde(rename = "OomScoreAdj", default)]
    pub oom_score_adj: i16,
    #[serde(rename = "NetworkMode", default)]
    pub network_mode: String,
    #[serde(rename = "PidMode", default)]
    pub pid_mode: String,
    #[serde(rename = "PortBindings", default)]
    pub port_bindings: HashMap<String, Vec<HashMap<String, String>>>,
    #[serde(rename = "Privileged", default)]
    pub privileged: bool,
    #[serde(rename = "ReadonlyRootfs", default)]
    pub readonly_rootfs: bool,
    #[serde(rename = "PublishAllPorts", default)]
    pub publish_all_ports: bool,
    #[serde(rename = "RestartPolicy", default)]
    pub restart_policy: HashMap<String, String>,
    #[serde(rename = "LogConfig", default)]
    pub log_config: HashMap<String, String>,
    #[serde(rename = "Sysctls", default)]
    pub sysctls: HashMap<String, String>,
    #[serde(rename = "Ulimits", default)]
    pub ulimits: Vec<HashMap<String, String>>,
    #[serde(rename = "VolumeDriver", default)]
    pub volume_driver: String,
    #[serde(rename = "ShmSize", default)]
    pub shm_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceRequest {
    #[serde(rename = "Driver", default)]
    pub driver: String,
    #[serde(rename = "Count", default)]
    pub count: u64,
    #[serde(rename = "DeviceIDs", default)]
    pub device_ids: Vec<String>,
    #[serde(rename = "Capabilities", default)]
    pub capabilities: Vec<Vec<String>>,
    #[serde(rename = "Options", default)]
    pub options: HashMap<String, String>,
}