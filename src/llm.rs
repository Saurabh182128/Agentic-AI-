use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Trait for LLM providers
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, messages: Vec<Message>) -> Result<String, String>;
    async fn stream_chat(&self, messages: Vec<Message>) -> Result<String, String>;
}

/// Message structure for LLM communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Message {
            role: role.to_string(),
            content: content.to_string(),
        }
    }

    pub fn system(content: &str) -> Self {
        Message::new("system", content)
    }

    pub fn user(content: &str) -> Self {
        Message::new("user", content)
    }

    pub fn assistant(content: &str) -> Self {
        Message::new("assistant", content)
    }
}

/// OpenAI-compatible LLM implementation
pub struct OpenAILLM {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: Message,
}

impl OpenAILLM {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        OpenAILLM {
            base_url,
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }

    /// Create with default OpenAI API
    pub fn default_openai(api_key: String) -> Self {
        OpenAILLM::new(
            "https://api.openai.com/v1".to_string(),
            api_key,
            "gpt-3.5-turbo".to_string(),
        )
    }

    /// Create with local Ollama instance
    pub fn ollama(model: String) -> Self {
        OpenAILLM::new(
            "http://localhost:11434/v1".to_string(),
            "ollama".to_string(),
            model,
        )
    }

    /// Create with LM Studio local instance
    pub fn lm_studio(model: String) -> Self {
        OpenAILLM::new(
            "http://localhost:1234/v1".to_string(),
            "lm-studio".to_string(),
            model,
        )
    }
}

#[async_trait]
impl LLMProvider for OpenAILLM {
    async fn chat(&self, messages: Vec<Message>) -> Result<String, String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 1024,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let body: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        body.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| "No response from model".to_string())
    }

    async fn stream_chat(&self, messages: Vec<Message>) -> Result<String, String> {
        // For now, same as chat. Can be extended for streaming
        self.chat(messages).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_system_message() {
        let msg = Message::system("You are a helpful assistant");
        assert_eq!(msg.role, "system");
    }

    #[test]
    fn test_openai_llm_creation() {
        let llm = OpenAILLM::default_openai("test-key".to_string());
        assert_eq!(llm.model, "gpt-3.5-turbo");
        assert_eq!(llm.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn test_ollama_llm_creation() {
        let llm = OpenAILLM::ollama("mistral".to_string());
        assert_eq!(llm.model, "mistral");
        assert_eq!(llm.base_url, "http://localhost:11434/v1");
    }

    #[test]
    fn test_lm_studio_llm_creation() {
        let llm = OpenAILLM::lm_studio("neural-chat".to_string());
        assert_eq!(llm.model, "neural-chat");
        assert_eq!(llm.base_url, "http://localhost:1234/v1");
    }
}
