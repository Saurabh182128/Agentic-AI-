# Agentic-AI

A Rust application for parsing and summarizing documents efficiently. This project provides a simple yet powerful framework for extracting key information from text documents.

## Features

- **Document Parsing**: Split documents into sentences for structured analysis
- **Keyword Extraction**: Automatically identify the most important keywords in a document
- **Text Summarization**: Generate concise summaries by selecting the most relevant sentences
- **Compression Metrics**: Calculate compression ratio to understand summary effectiveness
- **Stop Word Filtering**: Intelligent filtering of common words to focus on meaningful content

## Project Structure

```
Agentic-AI-/
├── Cargo.toml        # Project manifest and dependencies
├── src/
│   ├── lib.rs       # Core library with Document and Summary structs
│   └── main.rs      # Example usage and demonstrations
└── README.md        # This file
```

## Core Components

### Document Struct
Represents a parsed document with methods for analysis:
- `new()` - Create a new document
- `parse_sentences()` - Extract sentences from content
- `extract_keywords()` - Identify top keywords
- `summarize()` - Generate a summary

### Summary Struct
Contains the summarization results:
- `title` - Document title
- `key_points` - Extracted keywords
- `summary_text` - The summarized content
- `compression_ratio` - Percentage of content reduction

## Usage

### Basic Example

```rust
use agentic_ai::Document;

let doc = Document::new(
    "My Document".to_string(),
    "Your document content here...".to_string()
);

// Extract keywords
let keywords = doc.extract_keywords(5);
println!("Keywords: {:?}", keywords);

// Generate summary
let summary = doc.summarize(3);
println!("Summary: {}", summary.summary_text);
println!("Compression: {:.2}%", summary.compression_ratio);
```

### Running the Example

```bash
cargo run --bin summarizer
```

This will demonstrate the library with sample documents about Rust and AI/ML.

## Dependencies

- **regex** - Pattern matching for sentence parsing
- **serde** & **serde_json** - Serialization support
- **Edition 2021** - Modern Rust syntax and features

## Algorithm Overview

### Summarization Process
1. Parse document into sentences using regex
2. Extract top keywords (excluding stop words)
3. Score each sentence based on keyword frequency
4. Select top-scoring sentences while preserving order
5. Calculate compression ratio

### Stop Words
The algorithm filters out common English words (articles, prepositions, pronouns, etc.) to focus on meaningful content.

## Testing

Run the included unit tests:

```bash
cargo test
```

Tests cover:
- Document creation and word counting
- Sentence parsing
- Keyword extraction

## Future Enhancements

- [ ] Support for multiple languages
- [ ] TF-IDF scoring for better keyword extraction
- [ ] File I/O for document loading
- [ ] JSON export for summaries
- [ ] Web API interface
- [ ] Machine learning-based summarization

## License

This project is currently unlicensed. See LICENSE for more details.

## Author

Saurabh182128

---

**Note**: This is a foundational implementation designed for educational purposes and can be extended with more sophisticated NLP techniques.
