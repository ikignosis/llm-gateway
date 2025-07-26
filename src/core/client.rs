use conversa_openai_client::{
    client::CreateChatCompletionResponse, types::CreateChatCompletionRequest,
};
use tokio::sync::{mpsc::Sender, oneshot};

use crate::core::{message::GatewayMail, result::GatewayResult};

#[derive(Debug, Clone)]
pub struct LlmGatewayClient {
    sender: Sender<GatewayMail>,
}

impl LlmGatewayClient {
    pub fn new(sender: Sender<GatewayMail>) -> Self {
        Self { sender }
    }

    pub async fn create_chat_completion_request(
        &self,
        request: CreateChatCompletionRequest,
    ) -> GatewayResult<CreateChatCompletionResponse> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(GatewayMail::CreateChatCompletion {
                request,
                reply_sender: oneshot_sender,
            })
            .await?;
        oneshot_receiver.await?
    }
}
