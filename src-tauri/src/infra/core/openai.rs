use async_openai::config::OpenAIConfig;
use async_openai::Client;

pub fn new_client() -> Client<OpenAIConfig> {
    Client::new()
}
