use std::collections::HashMap;
use crate::client::{ClientTrait as _, Client};
use crate::container_create::{CreateContainerBody};
#[allow(unused_imports)]
use crate::{container_network::{HostConfig, NetworkingConfig}};

#[test]
fn test() {
    assert_eq!(0, 0);
}

#[test]
fn get_containers() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    match client.list_containers(false, 0, false, "".to_string()) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn create_container() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let options = CreateContainerBody {
        hostname: "localhost".to_string(),
        domainname: "localhost".to_string(),
        user: "".to_string(),
        attach_stdin: true,
        attach_stdout: true,
        attach_stderr: true,
        tty: true,
        open_stdin: true,
        stdin_once: false,
        env: vec![],
        cmd: vec!["echo test'".to_string()],
        image: "alpine:latest".to_string(),
        labels: HashMap::new(),
        volumes: HashMap::new(),
        working_dir: "".to_string(),
        entrypoint: "".to_string(),
        network_disabled: false,
        mac_address: "".to_string(),
        stop_signal: "".to_string(),
        stop_timeout: 0,
        host_config: HostConfig::default(),
        networking_config: NetworkingConfig::default(),
        exposed_ports: HashMap::new(),
    };

    let response = match client.create_container("test", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    assert!(response.id.len() > 0);
}

#[test]
fn create_container_short() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerBody::default();
    options.image = "alpine:latest".to_string();
    options.cmd = vec!["/bin/true".to_string()];

    let response = match client.create_container("test2", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    assert!(response.id.len() > 0);
}

#[test]
fn create_and_remove_container() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerBody::default();
    options.image = "alpine:latest".to_string();
    options.cmd = vec!["/bin/true".to_string()];

    let response = match client.create_container("test3", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    match client.remove_container(&response.id, false, true, false) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn create_start_getstats_stop_remove() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerBody::default();
    options.image = "alpine:latest".to_string();
    options.cmd = vec!["/bin/true".to_string()];

    let response = match client.create_container("test4", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    match client.start_container(&response.id) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };

    match client.get_stats_container(&response.id, true, false) {
        Ok(stats) => {
            println!("Container CPU Usage (this mean that the container is running): {:?}", stats.cpu_stats.cpu_usage.total_usage);
        },
        Err(e) => panic!("Error: {}", e)
    };

    match client.stop_container(&response.id, 0) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };

    match client.remove_container(&response.id, false, true, false) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn get_stats() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    // get first a container id
    let containers = match client.list_containers(true, 0, false, "".to_string()) {
        Ok(containers) => containers,
        Err(e) => panic!("Error: {}", e)
    };

    let id = containers[0].id.clone();

    match client.get_stats_container(&id.to_string(), true, false) {
        Ok(stats) => stats,
        Err(e) => panic!("Error: {}", e)
    };
}