use std::net::SocketAddr;

use axum::Router;
use clap::Parser;

fn router() -> Router {
    Router::new()
}

#[derive(Parser)]
struct Cli {
    /// Port number on which the server listens for incoming connections
    #[clap(default_value_t = 3456)]
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));

    let router = router().into_make_service();
    axum_server::bind(addr)
        .serve(router)
        .await
        .map_err(|e| format!("Server error: {}", e))
}
