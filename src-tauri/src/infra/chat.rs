use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_trait::async_trait;

use crate::common::errors::ApplicationError;
use crate::domain::chat::{AIChat, ChatSettings};
use crate::infra::core::openai::AIClient;

#[derive(Clone, Debug)]
pub struct OpenAIChat<T>
where
    T: AIClient,
{
    client: Arc<T>,
}

#[async_trait]
impl<T> AIChat for OpenAIChat<T>
where
    T: AIClient,
{
    async fn do_chat(&self, settings: &ChatSettings) -> Result<String, ApplicationError> {
        let mut req = CreateChatCompletionRequestArgs::default();

        req.model(&settings.model)
            .temperature(settings.temperature)
            .messages(self.build_messages(settings.clone()));

        if let Some(max_tokens) = &settings.max_tokens {
            req.max_tokens(*max_tokens);
        }
        // if let Some(response_format) = &settings.response_format {
        //     req.response_format(response_format);
        // }

        let req = req.build().unwrap();

        match self.client.create_chat(req).await {
            Ok(response) => {
                if response.choices.is_empty() || response.choices[0].message.content.is_none() {
                    Err(ApplicationError::EmptyResult)
                } else {
                    Ok(response.choices[0].message.content.clone().unwrap())
                }
            }
            Err(err) => {
                println!("OpenAI chat error: {}", err);
                Err(ApplicationError::OpenAPIError(err.to_string()))
            }
        }
    }
}

impl<T> OpenAIChat<T>
where
    T: AIClient,
{
    pub fn new(client: Arc<T>) -> Self {
        OpenAIChat { client }
    }

    fn build_messages(&self, settings: ChatSettings) -> Vec<ChatCompletionRequestMessage> {
        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(settings.system_prompt)
            .build()
            .unwrap();

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(settings.user_prompt)
            .build()
            .unwrap();

        let messages: Vec<ChatCompletionRequestMessage> = vec![
            ChatCompletionRequestMessage::System(system_message),
            ChatCompletionRequestMessage::User(user_message),
        ];
        messages
    }
}

#[cfg(test)]
mod tests {
    use async_openai::error::{ApiError, OpenAIError};
    use async_openai::types::{
        ChatChoice, ChatCompletionResponseMessage, CompletionUsage, CreateChatCompletionRequest,
        CreateChatCompletionResponse, FinishReason, Role,
    };
    use async_trait::async_trait;

    use crate::domain::chat::ChatSettings;
    use crate::infra::core::openai::AIClient;

    use super::*;

    #[tokio::test]
    async fn test_do_chat() {
        struct MockOpenAIClient {}

        #[async_trait]
        impl AIClient for MockOpenAIClient {
            async fn create_chat(
                &self,
                _req: CreateChatCompletionRequest,
            ) -> Result<CreateChatCompletionResponse, OpenAIError> {
                Ok(CreateChatCompletionResponse {
                    id: "test".to_string(),
                    object: "chat.completion".to_string(),
                    created: 0,
                    model: "gpt-4-1106-preview".to_string(),
                    usage: Option::from(CompletionUsage {
                        prompt_tokens: 0,
                        completion_tokens: 0,
                        total_tokens: 0,
                    }),
                    choices: vec![ChatChoice {
                        message: ChatCompletionResponseMessage {
                            role: Role::System,
                            content: Some("Test message".to_string()),
                            tool_calls: None,
                            function_call: None, // NOTE: function_callが完全に廃止されたら削除する
                        },
                        finish_reason: Option::from(FinishReason::Stop),
                        index: 0,
                    }],
                    system_fingerprint: None,
                })
            }
        }

        let mock_chat = OpenAIChat {
            client: Arc::new(MockOpenAIClient {}),
        };
        let settings = ChatSettings {
            id: 0,
            system_prompt: "System prompt".to_string(),
            user_prompt: "User prompt".to_string(),
            model: "gpt-4-1106-preview".to_string(),
            temperature: 0.0,
            max_tokens: None,
            response_format: None,
        };
        let result = mock_chat.do_chat(&settings).await;
        assert_eq!(result.unwrap(), "Test message");
    }

    #[tokio::test]
    async fn test_do_chat_error() {
        struct MockOpenAIClient;

        #[async_trait]
        impl AIClient for MockOpenAIClient {
            async fn create_chat(
                &self,
                _req: CreateChatCompletionRequest,
            ) -> Result<CreateChatCompletionResponse, OpenAIError> {
                // モックのレスポンスを返す
                Err(OpenAIError::ApiError(ApiError {
                    message: "Internal Server Error".to_string(),
                    r#type: None,
                    param: None,
                    code: None,
                }))
            }
        }

        let mock_chat = OpenAIChat {
            client: Arc::new(MockOpenAIClient),
        };
        let settings = ChatSettings {
            id: 0,
            system_prompt: "System prompt".to_string(),
            user_prompt: "User prompt".to_string(),
            model: "gpt-4-1106-preview".to_string(),
            temperature: 0.0,
            max_tokens: None,
            response_format: None,
        };
        let result = mock_chat.do_chat(&settings).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApplicationError::OpenAPIError(
                OpenAIError::ApiError(ApiError {
                    message: "Internal Server Error".to_string(),
                    r#type: None,
                    param: None,
                    code: None,
                })
                .to_string()
            )
        );
    }
}
