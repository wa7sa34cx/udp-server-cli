//! Main module

use udp_server_cli::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
