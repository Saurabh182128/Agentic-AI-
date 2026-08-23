pub mod agent;
pub mod llm;

use regex::Regex;
use std::collections::HashMap;

/// Represents a parsed document
#[derive(Debug, Clone)]
pub struct Document {
    pub title: String,
    pub content: String,
    pub word_count: usize,
}

/// Represents a summary of a document
#[derive(Debug, Clone)]
pub struct Summary {
    pub title: String,
    pub key_points: Vec<String>,
    pub summary_text: String,
    pub compression_ratio: f32,
}

impl Document {
    /// Create a new document from raw text
    pub fn new(title: String, content: String) -> Self {
        let word_count = content.split_whitespace().count();
        Document {
            title,
            content,
            word_count,
        }
    }

    /// Parse document into sentences
    pub fn parse_sentences(&self) -> Vec<String> {
        let sentence_regex = Regex::new(r"[.!?]+").unwrap();
        sentence_regex
            .split(&self.content)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Extract keywords from the document
    pub fn extract_keywords(&self, top_n: usize) -> Vec<String> {
        let words: Vec<&str> = self.content.to_lowercase().split_whitespace().collect();
        let mut word_freq: HashMap<String, usize> = HashMap::new();

        // Common stop words to filter out
        let stop_words = vec![
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had",
            "do", "does", "did", "will", "would", "could", "should", "may", "might", "can",
            "this", "that", "these", "those", "i", "you", "he", "she", "it", "we", "they",
        ];

        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !stop_words.contains(&clean_word) && clean_word.len() > 2 {
                *word_freq.entry(clean_word.to_string()).or_insert(0) += 1;
            }
        }

        let mut keywords: Vec<_> = word_freq.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));

        keywords
            .into_iter()
            .take(top_n)
            .map(|(word, _)| word)
            .collect()
    }

    /// Summarize the document
    pub fn summarize(&self, summary_length: usize) -> Summary {
        let sentences = self.parse_sentences();
        let keywords = self.extract_keywords(10);

        // Score sentences based on keyword frequency
        let mut scored_sentences: Vec<(usize, String, f32)> = sentences
            .iter()
            .enumerate()
            .map(|(idx, sentence)| {
                let mut score = 0.0;
                for keyword in &keywords {
                    if sentence.to_lowercase().contains(&keyword.to_lowercase()) {
                        score += 1.0;
                    }
                }
                (idx, sentence.clone(), score)
            })
            .collect();

        // Select top sentences by score
        scored_sentences.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let mut top_sentences: Vec<_> = scored_sentences
            .iter()
            .take(summary_length)
            .collect();

        // Sort back to original order
        top_sentences.sort_by_key(|a| a.0);

        let summary_text = top_sentences
            .iter()
            .map(|(_, s, _)| s.clone())
            .collect::<Vec<_>>()
            .join(" ");

        let original_words = self.word_count;
        let summary_words = summary_text.split_whitespace().count();
        let compression_ratio = (1.0 - (summary_words as f32 / original_words as f32)) * 100.0;

        Summary {
            title: self.title.clone(),
            key_points: keywords,
            summary_text,
            compression_ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "Test".to_string(),
            "This is a test document.".to_string(),
        );
        assert_eq!(doc.word_count, 5);
    }

    #[test]
    fn test_parse_sentences() {
        let doc = Document::new(
            "Test".to_string(),
            "First sentence. Second sentence! Third sentence?".to_string(),
        );
        let sentences = doc.parse_sentences();
        assert_eq!(sentences.len(), 3);
    }

    #[test]
    fn test_extract_keywords() {
        let doc = Document::new(
            "Test".to_string(),
            "rust rust programming rust code programming".to_string(),
        );
        let keywords = doc.extract_keywords(2);
        assert_eq!(keywords.len(), 2);
    }
}
