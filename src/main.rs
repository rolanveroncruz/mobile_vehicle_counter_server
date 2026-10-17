mod server;
mod handlers;
mod entities;

use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run_server().await?;
    Ok(())
}
