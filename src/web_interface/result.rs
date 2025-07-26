use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::core::result::GatewayError;

pub type WebinterfaceResult<T> = Result<T, WebinterfaceError>;

pub struct WebinterfaceError {
    status_code: StatusCode,
    data: Vec<u8>,
}

impl IntoResponse for WebinterfaceError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.data).into_response()
    }
}

impl From<GatewayError> for WebinterfaceError {
    fn from(value: GatewayError) -> Self {
        match value {
            GatewayError::GatewayChannelClosed | GatewayError::NoOperationReplyReceived => {
                WebinterfaceError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    data: Vec::new(),
                }
            }
            GatewayError::LlmClientError(conversa_error) => match conversa_error {
                conversa_openai_client::ConversaError::ClientError(_) => todo!(),
                conversa_openai_client::ConversaError::InvalidData(_) => todo!(),
                conversa_openai_client::ConversaError::IoError(_) => todo!(),
                conversa_openai_client::ConversaError::UnexpectedStatusCode { code, response } => {
                    WebinterfaceError {
                        status_code: StatusCode::from_u16(code).expect("This conversion always suceeds because the reply comes from a web server"),
                        data: response.into_bytes(),
                    }
                }
                conversa_openai_client::ConversaError::UnexpectedContentType(_) => todo!(),
                conversa_openai_client::ConversaError::ErrorResponse(_) => {
                    WebinterfaceError {
                        status_code: StatusCode::BAD_REQUEST,
                        data: vec![],
                    }
                }
                conversa_openai_client::ConversaError::Error(_) => {
                    WebinterfaceError {
                        status_code: StatusCode::NOT_FOUND,
                        data: vec![]
                    }
                },
            },
        }
    }
}
