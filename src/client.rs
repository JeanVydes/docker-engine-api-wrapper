use hyper::{Client as HyperClient, Method, Body, Request};
use hyperlocal::{UnixClientExt, UnixConnector};

pub struct Client {
    pub url: String,
    pub client: HyperClient<UnixConnector>,
    pub runtime: tokio::runtime::Runtime,
}

pub trait ClientTrait {
    fn new(url: String) -> Self;
    fn ping(&self) -> Result<(), Box<dyn std::error::Error>>;
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
            let uri = format!("{}/_ping", self.url);
            let uri = uri.parse::<hyper::Uri>()?;
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
}

// Compare this snippet from docker-engine-api/src/container.rs: