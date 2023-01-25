use std::collections::HashMap;
use crate::client::{ClientTrait as _, Client};
use crate::apicontainer::{CreateContainerBody};
use crate::{container::ContainerTrait as _, containerstructs::{HostConfig, NetworkingConfig}};

#[test]
fn test() {
    assert_eq!(0, 0);
}

//ignore test
#[test]
fn get_containers() {
    let mut client = Client::new("/var/run/docker.sock".to_string());
    match client.get_containers(true, 0, false, "".to_string()) {
        Ok(containers) => {
            println!("Containers: {:?}", containers.len());
        },
        Err(e) => panic!("Error: {}", e)
    };

    return;
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
        cmd: vec!["nginx -g 'daemon off;'".to_string()],
        image: "nginx:latest".to_string(),
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
    options.image = "nginx:latest".to_string();
    options.cmd = vec!["nginx -g 'daemon off;'".to_string()];

    let response = match client.create_container("test2", "linux", &options) {
        Ok(response) => response,
        Err(e) => panic!("Error: {}", e)
    };

    assert!(response.id.len() > 0);
}