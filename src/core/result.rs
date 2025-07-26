use crate::core::message::GatewayMail;

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug)]
pub enum GatewayError {
    /// This error occurs when the gateway has been dropped or panicked while
    /// the clients are still working
    GatewayChannelClosed,
    /// This error occurs when an operation for which we expect a reply
    /// terminates without sending a response
    NoOperationReplyReceived,
    /// This error occurs when the client returns an error message after a request
    LlmClientError(String),
}

impl From<tokio::sync::mpsc::error::SendError<GatewayMail>> for GatewayError {
    fn from(_: tokio::sync::mpsc::error::SendError<GatewayMail>) -> Self {
        GatewayError::GatewayChannelClosed
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for GatewayError {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        GatewayError::NoOperationReplyReceived
    }
}

pub fn convert_client_error<T>(
    value: conversa_openai_client::ConversaResult<T>,
) -> GatewayResult<T> {
    value.map_err(|e| GatewayError::LlmClientError(e.to_string()))
}
