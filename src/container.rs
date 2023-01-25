use hyper::{Request, Method, Body};
use crate::client::{Client};
use crate::api::{
    LIST_CONTAINERS,
    CREATE_CONTAINER
};
use hyper::{body::HttpBody};
use hyperlocal::Uri;
use tokio::io::{self, AsyncWriteExt as _};
use crate::containerstructs::Container;
use crate::apicontainer::{CreateContainerBody, CreateContainerResponse};

pub trait ContainerTrait {
    fn get_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<Vec<Container>, Box<dyn std::error::Error + Send + Sync>>;
    fn create_container(&mut self, name: &str, image: &str, more: &CreateContainerBody<String>) -> Result<CreateContainerResponse, Box<dyn std::error::Error + Send + Sync>>;
}

impl ContainerTrait for Client {
    /// Gets a list of containers from the API.
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
    fn get_containers(&mut self, all: bool, limit: i32, size: bool, filters: String) -> Result<Vec<Container>, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}?all={}&limit={}&size={}&filters={}", LIST_CONTAINERS, all, limit, size, filters);
            let uri = Uri::new(self.url.clone(), &url).into();
            let mut response = self.client.get(uri).await?;

            while let Some(next) = response.data().await {
                let chunk = next?;
                io::stdout().write_all(&chunk).await?;
                let status = response.status();
                if status != 200 {
                    let err_message: serde_json::Value = serde_json::from_slice(&chunk)?;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server error: {}", err_message))).into());
                }

                let containers: Vec<Container> = serde_json::from_slice(&chunk)?;

                return Ok(containers);
            }

            Ok(vec![])
        })
    }

    fn create_container(&mut self, name: &str, platform: &str, more: &CreateContainerBody<String>) -> Result<CreateContainerResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.runtime.block_on(async {
            let url = format!("{}?name={}&platform={}", CREATE_CONTAINER, name, platform);
            let uri = Uri::new(self.url.clone(), &url);

            let body = serde_json::to_string(&more)?;

            let req = Request::builder()
                .method(Method::POST)
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

            Ok(CreateContainerResponse::default())
        })
    }
}
