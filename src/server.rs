//! Server module

// use std::io;
use std::net::SocketAddr;
// use std::cell::Cell;
use anyhow::Result;
use tokio::net::UdpSocket;

/// Our UDP server
#[derive(Debug)]
pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    client: Client,
    data: Option<String>,
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
    pub async fn new(addr: &SocketAddr, max_datagram_size: usize) -> Result<Self> {
        // Opens socket for listening
        let socket = UdpSocket::bind(addr).await?;

        // Prints info to the console
        log::info!("Listening on: {}", &addr);

        let client = Client::default();

        Ok(Server {
            socket,
            buf: vec![0; max_datagram_size],
            client,
            data: String::new(),
        })
    }

    pub async fn recv(&mut self) -> Result<()> {
        let (len, addr) = self.socket.recv_from(&mut self.buf).await?;
        self.client = Client::from(len, addr);
        log::info!("Received {:?} bytes from {:?}", len, addr);

        Ok(())
    }

    pub async fn process(&mut self) -> Result<()> {
        // self.data = String::from_utf8(self.buf[..self.client.len].collect())?;
        // self.data = self.buf[..self.client.len].to_string();
        let data = match String::from_utf8(self.buf[..self.client.len].to_owned()) {
            Ok(data) => data,
            Err(_) => {
                log::info!("Received invalid UTF-8");
                String::new()
            },
        };

        Ok(())
    }

    pub async fn send(&mut self) -> Result<()> {
        let Client { len, addr } = self.client;
        let amt = self.socket.send_to(&self.buf[..len], &addr).await?;
        log::info!("Sended {}/{} bytes to {}", amt, len, addr);

        Ok(())
    }
}
