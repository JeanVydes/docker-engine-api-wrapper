use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stats {
    #[serde(rename(deserialize = "read"), default)]
    pub read: String,
    #[serde(rename(deserialize = "pids_stats"), default)]
    pub pids_stats: PidsStats,
    #[serde(rename(deserialize = "networks"), default)]
    pub networks: HashMap<String, NetworkStats>,
    #[serde(rename(deserialize = "memory_stats"), default)]
    pub memory_stats: MemoryStats,
    #[serde(rename(deserialize = "blkio_stats"), default)]
    pub blkio_stats: BlkioStats,
    #[serde(rename(deserialize = "cpu_stats"), default)]
    pub cpu_stats: CpuStats,
    #[serde(rename(deserialize = "precpu_stats"), default)]
    pub precpu_stats: CpuStats,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PidsStats {
    #[serde(rename(deserialize = "current"), default)]
    pub current: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkStats {
    #[serde(rename(deserialize = "rx_bytes"), default)]
    pub rx_bytes: u64,
    #[serde(rename(deserialize = "rx_packets"), default)]
    pub rx_packets: u64,
    #[serde(rename(deserialize = "rx_errors"), default)]
    pub rx_errors: u64,
    #[serde(rename(deserialize = "rx_dropped"), default)]
    pub rx_dropped: u64,
    #[serde(rename(deserialize = "tx_bytes"), default)]
    pub tx_bytes: u64,
    #[serde(rename(deserialize = "tx_packets"), default)]
    pub tx_packets: u64,
    #[serde(rename(deserialize = "tx_errors"), default)]
    pub tx_errors: u64,
    #[serde(rename(deserialize = "tx_dropped"), default)]
    pub tx_dropped: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemoryStats {
    #[serde(rename(deserialize = "usage"), default)]
    pub usage: u64,
    #[serde(rename(deserialize = "max_usage"), default)]
    pub max_usage: u64,
    #[serde(rename(deserialize = "stats"), default)]
    pub stats: HashMap<String, u64>,
    #[serde(rename(deserialize = "limit"), default)]
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlkioStats {
    #[serde(rename(deserialize = "io_service_bytes_recursive"), default)]
    pub io_service_bytes_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_serviced_recursive"), default)]
    pub io_serviced_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_queue_recursive"), default)]
    pub io_queue_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_service_time_recursive"), default)]
    pub io_service_time_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_wait_time_recursive"), default)]
    pub io_wait_time_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_merged_recursive"), default)]
    pub io_merged_recursive: serde_json::Value,
    #[serde(rename(deserialize = "io_time_recursive"), default)]
    pub io_time_recursive: serde_json::Value,
    #[serde(rename(deserialize = "sectors_recursive"), default)]
    pub sectors_recursive: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlkioStatsEntry {
    #[serde(rename(deserialize = "major"), default)]
    pub major: u64,
    #[serde(rename(deserialize = "minor"), default)]
    pub minor: u64,
    #[serde(rename(deserialize = "op"), default)]
    pub op: String,
    #[serde(rename(deserialize = "value"), default)]
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CpuStats {
    #[serde(rename(deserialize = "cpu_usage"), default)]
    pub cpu_usage: CpuUsage,
    #[serde(rename(deserialize = "system_cpu_usage"), default)]
    pub system_cpu_usage: u64,
    #[serde(rename(deserialize = "online_cpus"), default)]
    pub online_cpus: u64,
    #[serde(rename(deserialize = "throttling_data"), default)]
    pub throttling_data: ThrottlingData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CpuUsage {
    #[serde(rename(deserialize = "total_usage"), default)]
    pub total_usage: u64,
    #[serde(rename(deserialize = "percpu_usage"), default)]
    pub percpu_usage: Vec<u64>,
    #[serde(rename(deserialize = "usage_in_kernelmode"), default)]
    pub usage_in_kernelmode: u64,
    #[serde(rename(deserialize = "usage_in_usermode"), default)]
    pub usage_in_usermode: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ThrottlingData {
    #[serde(rename(deserialize = "periods"), default)]
    pub periods: u64,
    #[serde(rename(deserialize = "throttled_periods"), default)]
    pub throttled_periods: u64,
    #[serde(rename(deserialize = "throttled_time"), default)]
    pub throttled_time: u64,
}