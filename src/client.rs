use hyper::{Client as HyperClient, Method};
use hyperlocal::{UnixClientExt, UnixConnector};

use crate::request::request;
use crate::{containers_service::ContainersService};

pub struct Client {
    pub url: String,
    pub client: HyperClient<UnixConnector>,
    pub runtime: tokio::runtime::Runtime,
    pub containers: ContainersService,
}

pub trait ClientTrait {
    fn new(url: String) -> Self;
    fn ping(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl ClientTrait for Client {
    fn new(url: String) -> Self {
        let client = HyperClient::unix();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let containers = ContainersService {
            url: url.clone(),
            client: client.clone(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        };

        Client {
            url,
            client,
            runtime,
            containers,
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
}