pub mod api;
pub mod client;
pub mod network;
pub mod container_network;
pub mod container_create;
pub mod container_procceses;
pub mod container_stats;
pub mod container_inspect;
pub mod container_structs;
pub mod request;
pub mod error;

#[cfg(test)]
mod tests;

use crate::client::ClientTrait;
pub fn new(url: String) -> client::Client {
    return client::Client::new(url);
}