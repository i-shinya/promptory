use sea_orm::DbErr;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ApplicationError {
    #[error("unknown error. detail: {0}")]
    UnknownError(String),
    #[error("result is empty")]
    EmptyResult,
    #[error("openai api error: {0}")]
    OpenAPIError(String),
    #[error("db error: {0}")]
    DBError(#[from] DbErr),
    #[error("entity error: {0}")]
    DBEntityError(String),
}
