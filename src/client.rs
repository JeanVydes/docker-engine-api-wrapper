use hyper::{Client as HyperClient, Method};
use hyperlocal::{UnixClientExt, UnixConnector};
use crate::{api::{
    EmptyOk,

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

    RESIZE_CONTAINER_TTY_START,
    RESIZE_CONTAINER_TTY_END,
    RESIZE_CONTAINER_TTY_METHOD,

    PAUSE_CONTAINER_START,
    PAUSE_CONTAINER_END,
    PAUSE_CONTAINER_METHOD,

    UNPAUSE_CONTAINER_START,
    UNPAUSE_CONTAINER_END,
    UNPAUSE_CONTAINER_METHOD,

    WAIT_CONTAINER_START,
    WAIT_CONTAINER_END,
    WAIT_CONTAINER_METHOD,
}, container_create::{CreateContainerBody}, request::request, error::err, container_inspect::InspectedContainer};

pub struct Client {
    pub url: String,
    pub client: HyperClient<UnixConnector>,
    pub runtime: tokio::runtime::Runtime,
}

pub trait ClientTrait {
    fn new(url: String) -> Self;
    fn ping(&self) -> Result<(), Box<dyn std::error::Error>>;

    // Containers
    fn list_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<LIST_CONTAINERS_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn create_container(&mut self, name: &str, image: &str, more: &CreateContainerBody<String>) -> Result<CREATE_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn inspect_container(&mut self, id: &str, size: bool) -> Result<InspectedContainer, Box<dyn std::error::Error + Send + Sync>>;
    fn start_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn stop_container(&mut self, id: &str, timeout: i32) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn restart_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn kill_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn remove_container(&mut self, id: &str, remove_associated_volumes: bool, force: bool, remove_specified_linked: bool) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn get_container_logs(&mut self, id: &str) -> Result<GET_CONTAINER_LOGS_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn list_processes(&mut self, id: &str) -> Result<LIST_PROCESSES_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn get_stats_container(&mut self, id: &str, stream: bool, oneshot: bool) -> Result<GET_STATS_CONTAINER_RETURN, Box<dyn std::error::Error + Send + Sync>>;
    fn resize_container_tty(&mut self, id: &str, height: i32, width: i32) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn pause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn unpause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
    fn wait_container(&mut self, id: &str, condition: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>>;
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
        let url = "/containers/json";
        let response = request(&self.client, self.url.clone(), url.to_string(), Method::GET, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&r.body)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
        let url = format!("{}?all={}&limit={}&size={}&filters={}", LIST_CONTAINERS, all, limit, size, filters);
        let response = request(&self.client, self.url.clone(), url, LIST_CONTAINERS_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                let containers: LIST_CONTAINERS_RETURN = serde_json::from_slice(&r.body)?;

                return Ok(containers)
            },
            Err(e) => Err(e),
        }
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
        let url = format!("{}?name={}&platform={}", CREATE_CONTAINER, name, platform);
        let body = serde_json::to_string(&more)?;
        let response = request(&self.client, self.url.clone(), url, CREATE_CONTAINER_METHOD, body, &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 201 {
                    return Err(err(&r.body)?);
                }

                let container = serde_json::from_slice(&r.body)?;

                return Ok(container);
            },
            Err(e) => Err(e),
        }
    }

    /// Inspects a container.
    ///
    /// # Arguments
    ///
    /// * id - A string value indicating the id of the container.
    /// * size - A boolean value indicating whether to show the container size in human readable format.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the details of the inspected container in bytes on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    ///
    /// See the Docker API reference[https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerInspect] for more information.
    fn inspect_container(&mut self, id: &str, size: bool) -> Result<InspectedContainer, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}?size={}", INSPECT_CONTAINER_START, id, INSPECT_CONTAINER_END, size);
        let response = request(&self.client, self.url.clone(), url, INSPECT_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                let container: InspectedContainer = serde_json::from_slice(&r.body)?;

                return Ok(container);
            },
            Err(e) => Err(e),
        }
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
    fn start_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}", START_CONTAINER_START, id, START_CONTAINER_END);
        let response = request(&self.client, self.url.clone(), url, START_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
    fn stop_container(&mut self, id: &str, timeout: i32) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}?t={}", STOP_CONTAINER_START, id, STOP_CONTAINER_END, timeout);
        let response = request(&self.client, self.url.clone(), url, STOP_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
    fn restart_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}", RESTART_CONTAINER_START, id, RESTART_CONTAINER_END);
        let response = request(&self.client, self.url.clone(), url, RESTART_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
    fn kill_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}", KILL_CONTAINER_START, id, KILL_CONTAINER_END);
        let response = request(&self.client, self.url.clone(), url, KILL_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
    fn remove_container(&mut self, id: &str, remove_associated_volumes: bool, force: bool, remove_specified_linked: bool) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}?v={}&force={}&link={}",  REMOVE_CONTAINER, id, remove_associated_volumes, force, remove_specified_linked);
        let response = request(&self.client, self.url.clone(), url,REMOVE_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
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
        let url = format!("{}{}{}", GET_CONTAINER_LOGS_START, id, GET_CONTAINER_LOGS_END);
        let response = request(&self.client, self.url.clone(), url, GET_CONTAINER_LOGS_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                let logs = String::from_utf8(r.body.to_vec())?;

                return Ok(logs)
            },
            Err(e) => Err(e),
        }
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
        let url = format!("{}{}{}", LIST_PROCESSES_START, id, LIST_PROCESSES_END);
        let response = request(&self.client, self.url.clone(), url, LIST_PROCESSES_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                let processes: LIST_PROCESSES_RETURN = serde_json::from_slice(&r.body)?;

                return Ok(processes)
            },
            Err(e) => Err(e),
        }
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
        let url = format!("{}{}{}?stream={}&one-shot={}", GET_STATS_CONTAINER_START, id, GET_STATS_CONTAINER_END, stream, oneshot);
        let response = request(&self.client, self.url.clone(), url, GET_STATS_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                let stats: GET_STATS_CONTAINER_RETURN = serde_json::from_slice(&r.body)?;

                return Ok(stats)
            },
            Err(e) => Err(e),
        }
    }

    /// This function resizes the tty of a container.
    ///     
    /// # Arguments
    /// 
    /// * id - The ID of the container
    /// * height - The height of the tty
    /// * width - The width of the tty
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing an EmptyOk struct on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    /// 
    /// See the Docker API reference [https://docs.docker.com/engine/api/v1.41/#operation/ContainerResize] for more information.
    fn resize_container_tty(&mut self, id: &str, height: i32, width: i32) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}?h={}&w={}", RESIZE_CONTAINER_TTY_START, id, RESIZE_CONTAINER_TTY_END, height, width);
        let response = request(&self.client, self.url.clone(), url, RESIZE_CONTAINER_TTY_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// This function pause a container
    /// 
    /// # Arguments
    /// 
    /// * id - The ID of the container
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing an EmptyOk struct on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    /// 
    /// See the Docker API reference [https://docs.docker.com/engine/api/v1.41/#operation/ContainerPause] for more information.
    fn pause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}", PAUSE_CONTAINER_START, id, PAUSE_CONTAINER_END);
        let response = request(&self.client, self.url.clone(), url, PAUSE_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// This function unpause a container
    /// 
    /// # Arguments
    /// 
    /// * id - The ID of the container
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing an EmptyOk struct on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    /// 
    /// See the Docker API reference [https://docs.docker.com/engine/api/v1.41/#operation/ContainerUnpause] for more information.
    fn unpause_container(&mut self, id: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}", UNPAUSE_CONTAINER_START, id, UNPAUSE_CONTAINER_END);
        let response = request(&self.client, self.url.clone(), url, UNPAUSE_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 204 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// Block until a container stops, then returns the exit code.
    /// 
    /// # Arguments
    /// 
    /// * id - The ID of the container
    /// * condition - The condition to wait for
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing an EmptyOk struct on success, or an error of type Box<dyn std::error::Error + Send + Sync> on failure.
    /// 
    /// See the Docker API reference [https://docs.docker.com/engine/api/v1.41/#operation/ContainerWait] for more information.
    fn wait_container(&mut self, id: &str, condition: &str) -> Result<EmptyOk, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}{}?condition={}", WAIT_CONTAINER_START, id, WAIT_CONTAINER_END, condition);
        let response = request(&self.client, self.url.clone(), url, WAIT_CONTAINER_METHOD, "".to_string(), &self.runtime);
        match response {
            Ok(r) => {
                if r.status != 200 {
                    return Err(err(&r.body)?);
                }

                return Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
