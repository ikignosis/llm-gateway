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
