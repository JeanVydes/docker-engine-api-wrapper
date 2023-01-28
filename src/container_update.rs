use serde::{Serialize, Deserialize};
use crate::{container_structs::GenericDevice, container_inspect::DeviceRequest};

/// UpdateContainer is the struct used to update a container
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateContainerWith {
    /// The number of CPUs. The default is 0.0 which means no limit.
    #[serde(rename(serialize = "CpuShares"), skip_serializing_if = "Option::is_none", default)]
    pub cpu_shares: Option<i64>,

    /// Memory limit (in bytes).
    #[serde(rename(serialize = "Memory"), skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<i64>,

    /// Path to cgroups under which the container's cgroup is created. 
    /// If the path is not absolute, the path is considered to be relative to the cgroups path of the init process. 
    /// Cgroups are created if they do not already exist.
    #[serde(rename(serialize = "CgroupParent"), skip_serializing_if = "Option::is_none", default)]
    pub cgroup_parent: Option<String>,

    /// Block IO weight (relative weight) accepts a weight value between 0 and 1000.
    #[serde(rename(serialize = "BlkioWeight"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_weight: Option<i16>,

    /// Block IO weight (relative device weight) as `ThreadDeviceWeight`
    #[serde(rename(serialize = "BlkioWeightDevice"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_weight_device: Option<Vec<ThrottleDeviceWeight>>,

    /// Block IO read rate limit (bytes per second) as `ThrottleDeviceRate`
    #[serde(rename(serialize = "BlkioDeviceReadBps"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_device_read_bps: Option<Vec<ThrottleDeviceRate>>,

    /// Block IO write rate limit (bytes per second) as `ThrottleDeviceRate`
    #[serde(rename(serialize = "BlkioDeviceWriteBps"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_device_write_bps: Option<Vec<ThrottleDeviceRate>>,

    /// Block IO read rate limit (IO per second) as `ThrottleDeviceRate`
    #[serde(rename(serialize = "BlkioDeviceReadIOps"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_device_read_iops: Option<Vec<ThrottleDeviceRate>>,

    /// Block IO write rate limit (IO per second) as `ThrottleDeviceRate`
    #[serde(rename(serialize = "BlkioDeviceWriteIOps"), skip_serializing_if = "Option::is_none", default)]
    pub blkio_device_write_iops: Option<Vec<ThrottleDeviceRate>>,

    /// CPU period to be used for hardcapping (in usecs). 0 to use system default. (in microseconds)
    #[serde(rename(serialize = "CpuPeriod"), skip_serializing_if = "Option::is_none", default)]
    pub cpu_period: Option<i64>,

    /// CPU quota to be used for hardcapping (in usecs). 0 to use system default. (in microseconds)
    #[serde(rename(serialize = "CpuQuota"), skip_serializing_if = "Option::is_none", default)]
    pub cpu_quota: Option<i64>,

    /// CPU real-time period in microseconds.
    #[serde(rename(serialize = "CpuRealtimePeriod"), skip_serializing_if = "Option::is_none", default)]
    pub cpu_realtime_period: Option<i64>,

    /// CPU real-time runtime in microseconds.
    #[serde(rename(serialize = "CpuRealtimeRuntime"), skip_serializing_if = "Option::is_none", default)]
    pub cpu_realtime_runtime: Option<i64>,

    /// CPUs in which to allow execution (0-3, 0,1).
    #[serde(rename(serialize = "CpusetCpus"), skip_serializing_if = "Option::is_none", default)]
    pub cpuset_cpus: Option<String>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename(serialize = "CpusetMems"), skip_serializing_if = "Option::is_none", default)]
    pub cpuset_mems: Option<String>,

    /// A list of devices to add to the container.
    #[serde(rename(serialize = "Devices"), skip_serializing_if = "Option::is_none", default)]
    pub devices: Option<Vec<GenericDevice>>,

    /// a list of cgroup rules to apply to the container
    #[serde(rename(serialize = "DeviceCgroupRules"), skip_serializing_if = "Option::is_none", default)]
    pub device_cgroup_rules: Option<Vec<String>>,

    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename(serialize = "DeviceRequests"), skip_serializing_if = "Option::is_none", default)]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Kernel memory limit (in bytes).
    #[serde(rename(serialize = "KernelMemory"), skip_serializing_if = "Option::is_none", default)]
    pub kernel_memory: Option<i64>,

    #[serde(rename(serialize = "KernelMemoryTCP"), skip_serializing_if = "Option::is_none", default)]
    pub kernel_memory_tcp: Option<i64>,

    /// Memory soft limit (in bytes).
    #[serde(rename(serialize = "MemoryReservation"), skip_serializing_if = "Option::is_none", default)]
    pub memory_reservation: Option<i64>,

    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename(serialize = "MemorySwap"), skip_serializing_if = "Option::is_none", default)]
    pub memory_swap: Option<i64>,

    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename(serialize = "MemorySwappiness"), skip_serializing_if = "Option::is_none", default)]
    pub memory_swappiness: Option<i16>,

    /// CPU quota in units of 10^-9 CPUs.
    #[serde(rename(serialize = "NanosCpus"), skip_serializing_if = "Option::is_none", default)]
    pub nanos_cpus: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThrottleDeviceWeight {
    /// Path to the device file, relative to the container's cgroup.
    #[serde(rename(serialize = "Path"), skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,

    /// Rate (in bytes per second) at which the device is limited.
    #[serde(rename(serialize = "Weight"), skip_serializing_if = "Option::is_none", default)]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThrottleDeviceRate {
    /// Path to the device file, relative to the container's cgroup.
    #[serde(rename(serialize = "Path"), skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,

    /// Rate (in bytes per second) at which the device is limited.
    #[serde(rename(serialize = "Rate"), skip_serializing_if = "Option::is_none", default)]
    pub rate: Option<i64>,
}