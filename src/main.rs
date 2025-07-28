pub mod core;
pub mod result;
pub mod web_interface;

use std::net::SocketAddr;

use clap::Parser;
use conversa_openai_client::OpenAIClientBuilder;

use crate::{core::gateway::LlmGatewayBuilder, web_interface::router::llm_gateway_router};

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

    let mut gateway_builder = LlmGatewayBuilder::new();
    let client = OpenAIClientBuilder::new(
        "https://api.openai.com/v1".to_string(),
        std::env::var("OPENAI_API_KEY").unwrap(),
    )
    .build()
    .map_err(|e| format!("Failed to create AI client with error {:?}", e))?;
    gateway_builder.add_llm_provider(client);
    let gateway_client = gateway_builder.build();
    let router = llm_gateway_router(gateway_client).into_make_service();

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));

    axum_server::bind(addr)
        .serve(router)
        .await
        .map_err(|e| format!("Server error: {}", e))
}

#[cfg(test)]
mod main_tests {
    use super::*;

    use axum_test::{TestServer, TestServerConfig, Transport};
    use conversa_openai_client::OpenAIClientBuilder;

    #[tokio::test]
    async fn create_chat_completion_from_client_should_return_ok() {
        let gateway_client = LlmGatewayBuilder::new().build();
        let config = TestServerConfig {
            transport: Some(Transport::HttpRandomPort),
            ..TestServerConfig::default()
        };

        let server =
            TestServer::new_with_config(llm_gateway_router(gateway_client), config).unwrap();
        let mut server_url = server.server_address().unwrap().to_string();
        // Remove the trailing / from the address string
        server_url.pop().unwrap();
        println!("server url: {}", server_url);
        let client = OpenAIClientBuilder::new(server_url, String::new())
            .build()
            .unwrap();
        let request_body = conversa_openai_client::types::CreateChatCompletionRequest {
            create_model_response_properties:
                conversa_openai_client::types::CreateModelResponseProperties {
                    model_response_properties:
                        conversa_openai_client::types::ModelResponseProperties {
                            metadata: None,
                            top_logprobs: None,
                            temperature: None,
                            top_p: None,
                            user: None,
                            service_tier: None,
                        },
                    object: conversa_openai_client::types::CreateModelResponsePropertiesObject {
                        top_logprobs: None,
                    },
                },
            object: conversa_openai_client::types::CreateChatCompletionRequestObject {
                messages: vec![
                    conversa_openai_client::types::ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(
                        conversa_openai_client::types::ChatCompletionRequestUserMessage {
                            content: conversa_openai_client::types::ChatCompletionRequestUserMessageContent::String(String::from("What is the capital of France?")),
                            role: conversa_openai_client::types::ChatCompletionRequestUserMessageRole::User,
                            name: None
                        }
                    )
                ],
                model: conversa_openai_client::types::ModelIdsShared::String(String::from("Qwen/Qwen3-8B")),
                modalities: None,
                reasoning_effort: None,
                max_completion_tokens: None,
                frequency_penalty: None,
                presence_penalty: None,
                web_search_options: None,
                top_logprobs: None,
                response_format: None,
                audio: None,
                store: None,
                stream: None,
                stop: None,
                logit_bias: None,
                logprobs: None,
                max_tokens: None,
                n: None,
                prediction: None,
                seed: None,
                stream_options: None,
                tools: None,
                tool_choice: None,
                parallel_tool_calls: None,
                function_call: None,
                functions: None,
            },
        };
        let reply = client.create_chat_completion(request_body).await.unwrap();

        match reply {
            conversa_openai_client::client::CreateChatCompletionResponse::ApplicationJson(
                create_chat_completion_response,
            ) => {
                assert_eq!(create_chat_completion_response.choices.len(), 0); // TODO: This is going to fail when using a real provider
            }
            conversa_openai_client::client::CreateChatCompletionResponse::TextEventStream(_) => {
                panic!("Invalid reply type")
            }
        }
    }
}
