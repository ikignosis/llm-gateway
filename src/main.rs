pub mod chat_completions;
pub mod multipart;
pub mod result;
pub mod types;

use std::net::SocketAddr;

use axum::{Router, http::StatusCode, response::Html, routing::post};
use clap::Parser;

use crate::chat_completions::create_chat_completion;

fn router() -> Router {
    Router::new()
        .route("/chat/completions", post(create_chat_completion))
        .fallback(fallback)
}

// Default route
async fn fallback(uri: axum::http::Uri) -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        Html(format!(
            "<h1> 404 - Not Found</h1> <p>No route for {uri}</p>"
        )),
    )
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
