pub enum LlmGatewayError {
    IoError(String),
}

impl From<std::io::Error> for LlmGatewayError {
    fn from(value: std::io::Error) -> Self {
        LlmGatewayError::IoError(value.to_string())
    }
}
