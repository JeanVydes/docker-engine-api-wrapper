use hyper::{Client as HyperClient, Method, Body, Request, body::{Bytes, HttpBody}};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use tokio::io::{self, AsyncWriteExt as _};
use crate::{api::{
    NoImplementedYet,

    LIST_CONTAINERS,
    LIST_CONTAINERS_METHOD,
    LIST_CONTAINERS_RETURN,

    CREATE_CONTAINER,
    CREATE_CONTAINER_METHOD,
    CREATE_CONTAINER_RETURN,

    INSPECT_CONTAINER_START,
    INSPECT_CONTAINER_END,
    INSPECT_CONTAINER_METHOD,

    START_CONTAINER_START,
    START_CONTAINER_END,
    START_CONTAINER_METHOD,

    STOP_CONTAINER_START,
    STOP_CONTAINER_END,
    STOP_CONTAINER_METHOD,

    RESTART_CONTAINER_START,
    RESTART_CONTAINER_END,
    RESTART_CONTAINER_METHOD,

    KILL_CONTAINER_START,
    KILL_CONTAINER_END,
    KILL_CONTAINER_METHOD,

    GET_CONTAINER_LOGS_START,
    GET_CONTAINER_LOGS_END,
    GET_CONTAINER_LOGS_METHOD,
    GET_CONTAINER_LOGS_RETURN,

    LIST_PROCESSES_START,
    LIST_PROCESSES_END,
    LIST_PROCESSES_METHOD,
    LIST_PROCESSES_RETURN,

    GET_STATS_CONTAINER_START,
    GET_STATS_CONTAINER_END,
    GET_STATS_CONTAINER_METHOD,
    GET_STATS_CONTAINER_RETURN,

    REMOVE_CONTAINER,
    REMOVE_CONTAINER_METHOD,
}, container_create::{CreateContainerBody}};

pub struct Client {
    pub url: String,
    pub client: HyperClient<UnixConnector>,
    pub runtime: tokio::runtime::Runtime,
}

pub trait ClientTrait {
    fn new(url: String) -> Self;
    fn ping(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn list_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<LIST_CONTAINERS_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn create_container(&mut self, name: &str, image: &str, more: &CreateContainerBody<String>) -> Result<CREATE_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn inspect_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn start_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn stop_container(&mut self, id: &str, timeout: i32) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn restart_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn kill_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn remove_container(&mut self, id: &str, remove_associated_volumes: bool, force: bool, remove_specified_linked: bool) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>>;
    fn get_container_logs(&mut self, id: &str) -> Result<GET_CONTAINER_LOGS_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn list_processes(&mut self, id: &str) -> Result<LIST_PROCESSES_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn get_stats_container(&mut self, id: &str, stream: bool, oneshot: bool) -> Result<GET_STATS_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>>;
}

impl ClientTrait for Client {
    fn new(url: String) -> Self {
        let client = HyperClient::unix();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        Client {
            url,
            client,
            runtime,
        }
    }

    fn ping(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.runtime.block_on(async {
            let uri = Uri::new(self.url.clone(), "/ping");
            let req = Request::builder()
                .method(Method::GET)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let response = self.client.request(req).await?;

            if response.status().is_success() {
                return Ok(());
            }

            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No response")).into());
        })
    }
    
    /// Gets a list of containers.
    ///
    /// # Arguments
    ///
    /// * `all` - A boolean value indicating whether to show all containers or only running containers.
    /// * `limit` - An integer value indicating the maximum number of containers to return.
    /// * `size` - A boolean value indicating whether to show the container size in human readable format.
    /// * `filters` - A string value containing filters to apply to the container list.
    /// 
    ///  See the [More information](https://docs.docker.com/engine/api/v1.41/#tag/Container) for more information.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Container` structs on success, or an error of type `Box<dyn std::error::Error + Send + Sync>` on failure.
    fn list_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<LIST_CONTAINERS_RETURN, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}?all={}&limit={}&size={}&filters={}", LIST_CONTAINERS, all, limit, size, filters);
            let uri = Uri::new(self.url.clone(), &url);
            
            let req = Request::builder()
                .method(LIST_CONTAINERS_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;
                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let containers: LIST_CONTAINERS_RETURN = serde_json::from_slice(&chunk)?;

                return Ok(containers);
            }

            Ok(vec![])
        })
    }

    /// Creates a new container.
    ///
    /// # Arguments
    ///
    /// * name - A string value indicating the name of the container.
    /// * platform - A string value indicating the platform of the container.
    /// * more - A struct containing more information to be passed while creating the container.
    ///
    /// See the Docker API refenrece[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerCreate] for more information.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the created container on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    fn create_container(&mut self, name: &str, platform: &str, more: &CreateContainerBody<String>) -> Result<CREATE_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}?name={}&platform={}", CREATE_CONTAINER, name, platform);
            let uri = Uri::new(self.url.clone(), &url);

            let body = serde_json::to_string(&more)?;

            let req = Request::builder()
                .method(CREATE_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::from(body))?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 201 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let container = serde_json::from_slice(&chunk)?;

                return Ok(container);
            }

            Ok(CREATE_CONTAINER_RETURN::default())
        })
    }

    /// Inspects a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the inspected container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect] for more information.
    fn inspect_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", INSPECT_CONTAINER_START, id, INSPECT_CONTAINER_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(INSPECT_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Start a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the started container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerStart] for more information.
    fn start_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", START_CONTAINER_START, id, START_CONTAINER_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(START_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 204 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Stops a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    /// * timeout - An integer value indicating the timeout for the container to stop.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the stopped container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerStop]
    fn stop_container(&mut self, id: &str, timeout: i32) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}?t={}", STOP_CONTAINER_START, id, STOP_CONTAINER_END, timeout);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(STOP_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 204 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Restarts a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the restarted container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerRestart] for more information.
    fn restart_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", RESTART_CONTAINER_START, id, RESTART_CONTAINER_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(RESTART_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 204 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Kills a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the killed container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerKill] for more information.
    fn kill_container(&mut self, id: &str) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", KILL_CONTAINER_START, id, KILL_CONTAINER_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(KILL_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 204 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Removes a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    /// * remove_associated_volumes - A boolean value indicating whether to remove the associated volumes with the container.
    /// * force - A boolean value indicating whether to remove the container forcefully.
    /// * remove_specified_linked - A boolean value indicating whether to remove the specified linked containers.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the removed container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#operation/ContainerDelete] for more information.
    fn remove_container(&mut self, id: &str, remove_associated_volumes: bool, force: bool, remove_specified_linked: bool) -> Result<NoImplementedYet, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}?v={}&force={}&link={}", REMOVE_CONTAINER, id, remove_associated_volumes, force, remove_specified_linked);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(REMOVE_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 204 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(chunk);
            }

            Ok(Bytes::new())
        })
    }

    /// Retrieves the logs of a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the logs of the container as a string on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#operation/ContainerLogs] for more information.
    fn get_container_logs(&mut self, id: &str) -> Result<GET_CONTAINER_LOGS_RETURN, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", GET_CONTAINER_LOGS_START, id, GET_CONTAINER_LOGS_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(GET_CONTAINER_LOGS_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let logs: String = String::from_utf8(chunk.to_vec())?;

                // into the function type
                return Ok(logs);
            }

            Ok("".to_string().into())
        })
    }

    /// Lists the processes running inside a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    ///
    /// # Returns
    ///
    /// Returns a Result containing a list of processes running inside the container on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#operation/ContainerListProcesses] for more information.
    fn list_processes(&mut self, id: &str) -> Result<LIST_PROCESSES_RETURN, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}", LIST_PROCESSES_START, id, LIST_PROCESSES_END);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(LIST_PROCESSES_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let processes: LIST_PROCESSES_RETURN = serde_json::from_slice(&chunk)?;

                // into the function type
                return Ok(processes);
            }

            Ok(LIST_PROCESSES_RETURN::default())
        })
    }

    /// This function retrieves statistics for a specific container.
    ///
    /// # Arguments
    ///
    /// * id - The ID of the container
    /// * stream - A boolean value indicating whether to stream the output
    /// * oneshot - A boolean value indicating whether to retrieve only one set of statistics
    ///
    /// # Returns
    ///
    /// Returns a Result containing a GET_STATS_CONTAINER_RETURN
    /// ///
    /// # Example
    ///
    /// /// let mut client = Docker::connect("http://127.0.0.1:2375").unwrap(); /// let stats = client.get_stats_container("container_id", true, false).unwrap(); ///

    /// The struct returned by this function contains the following fields:
    ///
    /// * read - A string representing the time the stats were read
    /// * pids_stats - A struct containing PID statistics
    /// * networks - A HashMap of NetworkStats structs, keyed by the network name
    /// * memory_stats - A struct containing memory statistics
    /// * blkio_stats - A struct containing Block I/O statistics
    /// * cpu_stats - A struct containing CPU statistics
    /// * precpu_stats - A struct containing previous CPU statistic
    /// 
    /// See the Docker API reference [https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerStats] for more information.
    fn get_stats_container(&mut self, id: &str, stream: bool, oneshot: bool) -> Result<GET_STATS_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}{}{}?stream={}&one-shot={}", GET_STATS_CONTAINER_START, id, GET_STATS_CONTAINER_END, stream, oneshot);
            let uri = Uri::new(self.url.clone(), &url);

            let req = Request::builder()
                .method(GET_STATS_CONTAINER_METHOD)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::empty())?
                .into();

            let mut response = self.client.request(req).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;

                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let stats: GET_STATS_CONTAINER_RETURN = serde_json::from_slice(&chunk)?;

                // into the function type
                return Ok(stats);
            }

            Ok(GET_STATS_CONTAINER_RETURN::default())
        })
    }
}
