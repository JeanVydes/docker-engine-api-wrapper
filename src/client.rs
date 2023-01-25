use hyper::{Client as HyperClient};
use hyperlocal::{UnixClientExt, UnixConnector};


pub struct Client {
    pub url: String,
    pub client: HyperClient<UnixConnector>,
    pub runtime: tokio::runtime::Runtime,
}

pub trait ClientTrait {
    fn new(url: String) -> Self;
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
}

// Compare this snippet from docker-engine-api/src/container.rs: