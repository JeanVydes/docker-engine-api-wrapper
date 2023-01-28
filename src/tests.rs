use crate::client::{ClientTrait as _, Client};
use crate::container_create::{CreateContainerFrom};
#[allow(unused_imports)]
use crate::{container_network::{HostConfig, NetworkingConfig}};
use crate::containers_service::ContainersServiceTrait;

#[test]
fn test() {
    assert_eq!(0, 0);
}

#[test]
fn get_containers() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    match client.containers.list_containers(false, 0, false, "".to_string()) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn create_container() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    // use Option<>
    let options = CreateContainerFrom {
        exposed_ports: None,
        hostname: None,
        domainname: None,
        user: None,
        attach_stdin: None,
        attach_stdout: None,
        attach_stderr: None,
        tty: None,
        open_stdin: None,
        stdin_once: None,
        env: None,
        cmd: Some(vec!["/bin/true".to_string()]),
        image: Some("alpine:latest".to_string()),
        labels: None,
        volumes: None,
        working_dir: None,
        entrypoint: None,
        network_disabled: None,
        mac_address: None,
        stop_signal: None,
        stop_timeout: None,
        host_config: None,
        networking_config: None,
    };

    let response = match client.containers.create_container("test", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    assert!(response.id.len() > 0);
}

#[test]
fn create_container_short() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerFrom::default();
    options.image = Some("alpine:latest".to_string());
    options.cmd = Some(vec!["/bin/true".to_string()]);

    let response = match client.containers.create_container("test2", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    assert!(response.id.len() > 0);
}

#[test]
fn create_and_remove_container() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerFrom::default();
    options.image = Some("alpine:latest".to_string());
    options.cmd = Some(vec!["/bin/true".to_string()]);

    let response = match client.containers.create_container("test3", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    match client.containers.remove_container(&response.id, false, true, false) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn create_start_getstats_stop_remove() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    let mut options = CreateContainerFrom::default();
    options.image = Some("alpine:latest".to_string());
    options.cmd = Some(vec!["/bin/true".to_string()]);

    let response = match client.containers.create_container("test4", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    match client.containers.start_container(&response.id) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };

    match client.containers.get_stats_container(&response.id, true, false) {
        Ok(stats) => {
            println!("Container CPU Usage (this mean that the container is running): {:?}", stats.cpu_stats);
        },
        Err(e) => panic!("Error: {}", e)
    };

    match client.containers.stop_container(&response.id, 0) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };

    match client.containers.remove_container(&response.id, false, true, false) {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn get_stats() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    // get first a container id
    let containers = match client.containers.list_containers(true, 0, false, "".to_string()) {
        Ok(containers) => containers,
        Err(e) => panic!("Error: {}", e)
    };

    let id = containers[0].id.clone();

    match client.containers.get_stats_container(&id.to_string(), true, false) {
        Ok(stats) => stats,
        Err(e) => panic!("Error: {}", e)
    };
}

#[test]
fn inspect_container() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    // get first a container id
    let containers = match client.containers.list_containers(true, 0, false, "".to_string()) {
        Ok(containers) => containers,
        Err(e) => panic!("Error: {}", e)
    };

    let id = containers[0].id.clone();

    match client.containers.inspect_container(&id.to_string(), true) {
        Ok(container) => container,
        Err(e) => panic!("Error: {}", e)
    };
}