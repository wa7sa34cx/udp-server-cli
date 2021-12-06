//! Main module
//! 
//! Pattern is often predictable, and anything predictable can be hacked.

use udp_server_cli::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
