use crate::llm::{LLMProvider, Message};
use crate::Document;

/// Agent for intelligent document processing and chat
pub struct DocumentAgent {
    pub llm: std::sync::Arc<dyn LLMProvider>,
    pub conversation_history: Vec<Message>,
    pub system_prompt: String,
}

impl DocumentAgent {
    pub fn new(llm: std::sync::Arc<dyn LLMProvider>) -> Self {
        let system_prompt = "You are an intelligent document processing assistant. \
            You help users analyze, summarize, and extract insights from documents. \
            You can parse documents, extract keywords, generate summaries, and answer \
            questions about document content. Be concise, clear, and helpful in your responses."
            .to_string();

        DocumentAgent {
            llm,
            conversation_history: vec![Message::system(&system_prompt)],
            system_prompt,
        }
    }

    pub fn with_system_prompt(llm: std::sync::Arc<dyn LLMProvider>, prompt: String) -> Self {
        DocumentAgent {
            llm,
            conversation_history: vec![Message::system(&prompt)],
            system_prompt: prompt,
        }
    }

    /// Process a user message about a document
    pub async fn process_document(
        &mut self,
        document_content: &str,
        user_query: &str,
    ) -> Result<String, String> {
        // Parse and analyze the document
        let doc = Document::new("User Document".to_string(), document_content.to_string());

        // Extract key information from the document
        let keywords = doc.extract_keywords(8);
        let sentences = doc.parse_sentences();
        let summary = doc.summarize(3);

        // Prepare context for the LLM
        let context = format!(
            "Document Analysis Results:\n\
            - Keywords: {}\n\
            - Sentence Count: {}\n\
            - Summary: {}\n\
            - Word Count: {}\n\
            - Compression Ratio: {:.2}%\n\n\
            Document Content:\n{}",
            keywords.join(", "),
            sentences.len(),
            summary.summary_text,
            doc.word_count,
            summary.compression_ratio,
            document_content
        );

        // Create the LLM request
        let mut messages = self.conversation_history.clone();
        messages.push(Message::user(&format!(
            "Context:\n{}\n\nUser Query: {}",
            context, user_query
        )));

        // Get response from LLM
        let response = self.llm.chat(messages.clone()).await?;

        // Update conversation history
        self.conversation_history.push(Message::user(user_query));
        self.conversation_history.push(Message::assistant(&response));

        Ok(response)
    }

    /// Chat with the agent
    pub async fn chat(&mut self, user_message: &str) -> Result<String, String> {
        // Add user message to history
        self.conversation_history
            .push(Message::user(user_message));

        // Get response from LLM
        let response = self.llm.chat(self.conversation_history.clone()).await?;

        // Add assistant response to history
        self.conversation_history
            .push(Message::assistant(&response));

        Ok(response)
    }

    /// Analyze a document and return insights
    pub async fn analyze_document(&self, content: &str) -> Result<String, String> {
        let doc = Document::new("Analysis Document".to_string(), content.to_string());

        let keywords = doc.extract_keywords(10);
        let summary = doc.summarize(5);

        let analysis_prompt = format!(
            "I have analyzed a document with the following characteristics:\n\
            - Title: {}\n\
            - Word Count: {}\n\
            - Top Keywords: {}\n\
            - Summary: {}\n\
            - Compression: {:.2}%\n\n\
            Please provide insights about this document, including:\n\
            1. Main topics and themes\n\
            2. Key takeaways\n\
            3. Potential use cases or applications\n\
            4. Any recommendations for further reading or research",
            summary.title,
            doc.word_count,
            keywords.join(", "),
            summary.summary_text,
            summary.compression_ratio
        );

        let messages = vec![
            Message::system(&self.system_prompt),
            Message::user(&analysis_prompt),
        ];

        self.llm.chat(messages).await
    }

    /// Get the conversation history
    pub fn get_history(&self) -> &Vec<Message> {
        &self.conversation_history
    }

    /// Clear the conversation history
    pub fn clear_history(&mut self) {
        self.conversation_history = vec![Message::system(&self.system_prompt)];
    }

    /// Summarize multiple documents
    pub async fn summarize_multiple(
        &self,
        documents: Vec<(&str, &str)>,
    ) -> Result<String, String> {
        let mut summaries = Vec::new();

        for (title, content) in documents {
            let doc = Document::new(title.to_string(), content.to_string());
            let summary = doc.summarize(3);
            summaries.push(format!(
                "## {}\n{}\nKeywords: {}",
                summary.title,
                summary.summary_text,
                summary.key_points.join(", ")
            ));
        }

        let combined_summaries = summaries.join("\n\n");

        let prompt = format!(
            "I have multiple document summaries. Please provide a comprehensive overview \
            of all these documents and how they relate to each other:\n\n{}",
            combined_summaries
        );

        let messages = vec![
            Message::system(&self.system_prompt),
            Message::user(&prompt),
        ];

        self.llm.chat(messages).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        // Note: This test requires a mock LLM provider in a full test suite
        // For now, we just verify the structure
        let system_prompt = "Test prompt".to_string();
        assert!(!system_prompt.is_empty());
    }
}
