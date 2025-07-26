use axum::{Json, http::StatusCode};

use crate::types::{
    CreateChatCompletionRequest, CreateChatCompletionResponse, CreateChatCompletionResponseObject,
};

pub async fn create_chat_completion(
    _body: String,
) -> Result<Json<CreateChatCompletionResponse>, StatusCode> {
    Ok(CreateChatCompletionResponse {
        id: String::from("abcd"),
        choices: vec![],
        created: 123456879,
        model: String::from("GPT"),
        service_tier: None,
        system_fingerprint: None,
        object: CreateChatCompletionResponseObject::ChatCompletion,
        usage: None,
    }
    .into())
}
