# Docker Engine API Wrapper

A simple way to interact with the Docker Engine API

## Getting Start

First you should to have Docker installed locally; then you should run library test with the commnad `cargo test` (the tests uses the alpine image: `docker pull alpine`); if everything goes ok, congratulations you can start to code.

### Create a Client

A Client is the main way to connect to Docker Engine API, usually the socket is located in `/var/run/docker.sock`.

```rust
extern crate docker_engine_api;
use crate::docker_engine_api::client::ClientTrait;

fn main() {
    let mut client = docker_engine_api::new("/var/run/docker.sock".to_string());
    match client.ping() {
        Ok(_) => {println!("Pong!")},
        Err(e) => panic!("Error: {}", e)
    };
}
```

### Fetch Containers

To fetch containers, provide you with methods to fetch multiple containers, the most primitive and unadapted part is the filters that should be provided as a string, anyways you can check the docs [filters](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList).

```rust
fn get_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<Vec<Container>, Box<dyn std::error::Error + Send + Sync>>
```

What are those arguments in the function? [Check Docker Engine API documentation](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList)


```rust
match client.get_containers(false, 0, false, "".to_string()) {
    Ok(containers) => containers,
    Err(e) => panic!("Error: {}", e)
};
```

### Create New Container

For the following example you've already nginx:latest image in your system. For know open a cmd and type `docker image ls`, otherwise you can edit the image and use what you want...

[More Information](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate)

In a simple way:

```rust
use docker_engine_api::container_create::CreateContainerBody;

let mut options = CreateContainerBody::default();
options.image = "alpine:latest".to_string();
options.cmd = vec!["/bin/true".to_string()];

let response = match client.create_container("test2", "linux", &options) {
    Ok(response) => response,
    Err(e) => panic!("Error: {}", e)
};
```

Max customization options:

```rust
use docker_engine_api::container_create::CreateContainerBody;

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
```

### Get Stats

```rust
match client.get_stats_container(container_id, true, false) {
    Ok(stats) => {
        println!("{:?}", stats);
    },
    Err(e) => panic!("Error: {}", e)
};
```

### Another Methods

```rust
fn inspect_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn start_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn stop_container(&mut self, id: &str, timeout: i32) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn restart_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn remove_container(&mut self, id: &str, remove_associated_volumes: bool, force: bool, remove_specified_linked: bool) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn get_container_logs(&mut self, id: &str) -> Result<GET_CONTAINER_LOGS_RETURN, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn list_processes(&mut self, id: &str) -> Result<LIST_PROCESSES_RETURN, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn resize_container_tty(&mut self, id: &str, height: i32, width: i32) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn pause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn unpause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>
```

```rust
fn wait_container(&mut self, id: &str, condition: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>
```

# Contributors

Would be a pleasure to get you here...