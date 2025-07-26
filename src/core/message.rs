use super::result::GatewayResult;

use conversa_openai_client::{
    client::CreateChatCompletionResponse, types::CreateChatCompletionRequest,
};
use tokio::sync::oneshot;

pub enum GatewayMail {
    CreateChatCompletion {
        request: CreateChatCompletionRequest,
        reply_sender: oneshot::Sender<GatewayResult<CreateChatCompletionResponse>>,
    },
}
