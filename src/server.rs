//! Server module

mod error;
mod validation;

use crate::db::Db;
use async_once::AsyncOnce;
use bytes::Bytes;
use error::ServerError;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use validation::validate;

lazy_static! {
    static ref DB: AsyncOnce<Db> = AsyncOnce::new(async { Db::from_env().await });
}

/// Our UDP server
#[derive(Debug)]
pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    client: Client,
    res: Result<String, ServerError>,
}

// Remote client
#[derive(Debug)]
struct Client {
    len: usize,
    addr: SocketAddr,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            len: 0,
            addr: "0.0.0.0:0".parse().unwrap(),
        }
    }
}

impl Client {
    fn from(len: usize, addr: SocketAddr) -> Self {
        Client { len, addr }
    }
}

impl Server {
    pub async fn new(addr: SocketAddr, max_datagram_size: usize) -> anyhow::Result<Self> {
        // Opens socket for listening
        let socket = UdpSocket::bind(&addr).await?;

        // Prints info to the console
        log::info!("Listening on: {}", &addr);

        let client = Client::default();

        Ok(Server {
            socket,
            buf: vec![0; max_datagram_size],
            client,
            res: Ok(String::new()),
        })
    }

    /// Receives the request
    pub async fn recv(&mut self) -> anyhow::Result<()> {
        let (len, addr) = self.socket.recv_from(&mut self.buf).await?;
        self.client = Client::from(len, addr);
        log::info!("Received {:?} bytes from {:?}", len, addr);

        Ok(())
    }

    /// Processes the request
    pub async fn process(&mut self) -> anyhow::Result<()> {
        let id = match validate(&self.buf[..self.client.len]).await {
            Ok(num) => num,
            Err(e) => {
                self.res = Err(ServerError::ReqError(e.to_string()));
                return Ok(());
            }
        };

        match DB.get().await.get_by_id(id).await {
            Ok(item) => self.res = Ok(item.text),
            Err(e) => {
                self.res = Err(ServerError::DbError(e.to_string()));
            }
        };

        Ok(())
    }

    /// Sends the responce
    pub async fn send(&mut self) -> anyhow::Result<()> {
        // In this case, we send the client a response or an error.
        let buf = self.res.as_ref().map_or_else(
            // converts response to bytes 
            |s| Bytes::from(format!("{}\n", s)),
            // converts error to bytes
            |e| Bytes::from(format!("{}\n", e)),
        );

        let amt = self.socket.send_to(&buf[..], &self.client.addr).await?;
        log::info!(
            "Sended {}/{} bytes to {}",
            amt,
            buf.len(),
            &self.client.addr
        );

        Ok(())
    }
}
