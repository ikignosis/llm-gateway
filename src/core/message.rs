use tokio::sync::oneshot;

use super::result::GatewayResult;
use crate::types::{CreateChatCompletionRequest, CreateChatCompletionResponse};

pub enum GatewayMail {
    CreateChatCompletion {
        request: CreateChatCompletionRequest,
        reply_sender: oneshot::Sender<GatewayResult<CreateChatCompletionResponse>>,
    },
}
