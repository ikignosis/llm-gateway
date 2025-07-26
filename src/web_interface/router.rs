use crate::{
    core::client::LlmGatewayClient,
    types::{CreateChatCompletionResponse, CreateChatCompletionResponseObject},
};
use axum::{Json, Router, http::StatusCode, response::Html, routing::post};

pub fn llm_gateway_router(gateway_client: LlmGatewayClient) -> Router {
    let gateway_router = Router::new()
        .route("/chat/completions", post(create_chat_completion))
        .with_state(gateway_client);

    Router::new().nest("/v1", gateway_router).fallback(fallback)
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

pub async fn create_chat_completion(
    _body: String,
) -> Result<Json<CreateChatCompletionResponse>, StatusCode> {
    todo!()
    // .into())
}
