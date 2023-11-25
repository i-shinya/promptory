use async_openai::error::OpenAIError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("result is empty")]
    EmptyResult,
    #[error("http error: {0}")]
    OpenAPIError(#[from] OpenAIError),
}
