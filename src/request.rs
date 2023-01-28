use hyper::body::Bytes;
use hyper::{Client as HyperClient, Method, Request, Body, body::HttpBody};
use hyperlocal::{UnixConnector, Uri};
use tokio::runtime::Runtime;

pub struct SimpleResponse {
    pub status: u16,
    pub body: Bytes,
}

impl Default for SimpleResponse {
    fn default() -> Self {
        SimpleResponse {
            status: 0,
            body: Bytes::new(),
        }
    }
}

pub fn request(client: &HyperClient<UnixConnector>, socket: String, url: String, method: Method, body: String, runtime: &Runtime) -> Result<SimpleResponse, Box<dyn std::error::Error + Send + Sync>> {
    runtime.block_on(async {
        let uri = Uri::new(&socket, &url);
        let body = Body::from(body);

        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(body)?
            .into();

        let mut response = client.request(req).await?; 
        while let Some(next) = response.data().await {
            let chunk = next?;

            let simple = SimpleResponse {
                status: response.status().as_u16(),
                body: chunk,
            };

            return Ok(simple);
        }

        Ok(SimpleResponse {
            status: response.status().as_u16().into(),
            body: Bytes::new(),
        })
    })
}