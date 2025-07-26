use conversa_openai_client::OpenAIClient;

use crate::core::result::convert_client_error;

use super::{client::LlmGatewayClient, message::GatewayMail};

const GATEWAY_CHANNEL_BUFFER_SIZE: usize = 32;

pub struct LlmGateway;
pub struct LlmGatewayBuilder {
    llm_provider_list: Vec<OpenAIClient>,
}

impl LlmGatewayBuilder {
    pub fn new() -> Self {
        Self {
            llm_provider_list: Vec::new(),
        }
    }

    pub fn add_llm_provider(&mut self, provider: OpenAIClient) {
        self.llm_provider_list.push(provider);
    }

    pub fn build(self) -> LlmGatewayClient {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(GATEWAY_CHANNEL_BUFFER_SIZE);
        tokio::spawn(async move {
            while let Some(m) = receiver.recv().await {
                match m {
                    GatewayMail::CreateChatCompletion {
                        request,
                        reply_sender: response,
                    } => {
                        let reply = self.llm_provider_list[0]
                            .create_chat_completion(request)
                            .await;
                        response.send(convert_client_error(reply)).ok();
                    }
                }
            }
        });

        LlmGatewayClient::new(sender)
    }
}
