//! lib.rs

mod db;
mod logging;
mod server;

use server::Server;
use std::env;
use std::net::SocketAddr;
// use tokio::net::UdpSocket;

// RFC 791
// https://tools.ietf.org/html/rfc791
// const MAX_DATAGRAM_SIZE: usize = 508;
const MAX_DATAGRAM_SIZE: usize = 1024;

pub async fn run() -> anyhow::Result<()> {
    // Enables logging
    enable_logging!();

    // Creates socket address
    let addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string())
        .parse()?;

    // Creates new server
    let mut server = Server::new(addr, MAX_DATAGRAM_SIZE).await?;

    // Starts server loop
    loop {
        server.recv().await?;
        server.process().await?;
        server.send().await?;
    }
}
