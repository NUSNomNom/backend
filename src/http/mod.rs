pub mod config;

use anyhow::Result;
use config::Config;
use tokio::net::TcpListener;

pub async fn serve(config: Config, listener: TcpListener) -> Result<()> {
    unimplemented!()
}