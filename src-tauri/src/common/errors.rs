use async_openai::error::OpenAIError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("unknown error. detail: {0}")]
    UnknownError(String),
    #[error("result is empty")]
    EmptyResult,
    #[error("http error: {0}")]
    OpenAPIError(#[from] OpenAIError),
}
