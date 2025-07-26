use axum::{Json, Router, extract::State, http::StatusCode, response::Html, routing::post};
use conversa_openai_client::{
    client::CreateChatCompletionResponse, types::CreateChatCompletionRequest,
};

use crate::{core::client::LlmGatewayClient, web_interface::result::WebinterfaceResult};

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
    State(gateway_client): State<LlmGatewayClient>,
    Json(body): Json<CreateChatCompletionRequest>,
) -> WebinterfaceResult<Json<CreateChatCompletionResponse>> {
    Ok(gateway_client
        .create_chat_completion_request(body)
        .await?
        .into())
}
