use crate::types::{CreateChatCompletionResponse, CreateChatCompletionResponseObject};

use super::{client::LlmGatewayClient, message::GatewayMail};

const GATEWAY_CHANNEL_BUFFER_SIZE: usize = 32;

pub struct LlmGateway;

pub struct LlmGatewayBuilder;

impl LlmGatewayBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(self) -> LlmGatewayClient {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(GATEWAY_CHANNEL_BUFFER_SIZE);
        tokio::spawn(async move {
            while let Some(m) = receiver.recv().await {
                match m {
                    GatewayMail::CreateChatCompletion {
                        request: _,
                        reply_sender: response,
                    } => {
                        let output = Ok(CreateChatCompletionResponse {
                            id: String::from("abcd"),
                            choices: vec![],
                            created: 123456879,
                            model: String::from("GPT"),
                            service_tier: None,
                            system_fingerprint: None,
                            object: CreateChatCompletionResponseObject::ChatCompletion,
                            usage: None,
                        });
                        response.send(output).ok();
                    }
                }
            }
        });

        LlmGatewayClient::new(sender)
    }
}
