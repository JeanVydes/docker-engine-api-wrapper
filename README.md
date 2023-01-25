# Docker Engine API

A simple way to interact with the Docker Engine API

## Getting Start

First you should to have Docker installed locally; then you should run library test with the commnad `cargo test`; if everything goes ok, congratulations you can start to code.

### Fetch Containers

To fetch containers, provide you with methods to fetch multiple containers, the most primitive and unadapted part is the filters that should be provided as a string, anyways you can check the docs [filters](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList).

What are those arguments in the function? [Check Docker Engine API documentation](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerList)

```rust
fn get_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<Vec<Container>, Box<dyn std::error::Error + Send + Sync>>
```

```rust
let mut client = crate::client::Client::new("/var/run/docker.sock".to_string());
    let containers =  match client.get_containers(true, 0, false, "".to_string()) {
        Ok(containers) => containers,
        Err(e) => panic!("Error: {}", e)
    };
```

### Create New Container

For the following example you've already nginx:latest image in your system. For know open a cmd and type `docker image ls`, otherwise you can edit the image and use what you want...

[More Information](https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate)

In a simple way:

```rust
let mut client = Client::new("/var/run/docker.sock".to_string());
let mut options = CreateContainerBody::default();
options.image = "nginx:latest".to_string();
options.cmd = vec!["nginx -g 'daemon off;'".to_string()];

let response = match client.create_container("test2", "linux", &options) {
    Ok(response) => response,
    Err(e) => panic!("Error: {}", e)
};
```

Max customization options:

```rust
let mut client = crate::client::Client::new("/var/run/docker.sock".to_string());
let options = crate::apicontainer::CreateContainerBody {
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
    image: "nginx".to_string(),
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

# Contributors

Would be a pleasure to get you here...