use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse};
use async_openai::Client;
use async_trait::async_trait;

/// open aiのclientのラッパーtrait
/// crate内の実装がstructなのでテストでモックを使うための対応
#[async_trait]
pub trait AIClient: Send + Sync {
    async fn create_chat(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError>;
}

#[derive(Clone, Debug)]
struct OpenAIClient {
    client: Client<OpenAIConfig>,
}

pub fn new_client() -> Box<dyn AIClient> {
    let client = Client::new();
    Box::new(OpenAIClient { client })
}

#[async_trait]
impl AIClient for OpenAIClient {
    async fn create_chat(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        self.client.chat().create(request).await
    }
}
