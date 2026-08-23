# Agentic-AI 🤖

A sophisticated Rust application for intelligent document processing and agentic chat powered by open-source LLMs. This project combines document parsing, summarization, and AI-driven conversation capabilities.

## 🎯 Features

### Document Processing
- **Document Parsing**: Split documents into sentences for structured analysis
- **Keyword Extraction**: Automatically identify the most important keywords using stop-word filtering
- **Text Summarization**: Generate concise summaries by selecting the most relevant sentences
- **Compression Metrics**: Calculate compression ratio to understand summary effectiveness
- **Document Analysis**: Intelligent analysis with AI-generated insights

### Agentic Chat
- **LLM Integration**: Support for multiple open-source LLM providers
- **Conversation Memory**: Maintains conversation history for context-aware responses
- **Document-Aware Chat**: Ask questions about uploaded documents
- **Multi-Provider Support**: Switch between different LLM backends
- **Interactive CLI**: User-friendly command-line interface

## 📁 Project Structure

```
Agentic-AI-/
├── Cargo.toml           # Project manifest and dependencies
├── README.md            # This file
└── src/
    ├── lib.rs           # Core document processing library
    ├── main.rs          # Simple summarization example
    ├── agent_main.rs    # Interactive agentic chat binary
    ├── llm.rs           # LLM provider abstraction
    ├── agent.rs         # DocumentAgent for intelligent processing
    └── agent/
        └── (agent module exports)
```

## 🏗️ Architecture

### Core Modules

#### `Document` (src/lib.rs)
Represents a parsed document with methods for analysis:
- `new()` - Create a new document
- `parse_sentences()` - Extract sentences from content
- `extract_keywords()` - Identify top keywords
- `summarize()` - Generate a summary with compression metrics

#### `LLMProvider` (src/llm.rs)
Trait for LLM integration with implementations:
- **OpenAILLM** - OpenAI-compatible API client
  - `default_openai()` - Use OpenAI API
  - `ollama()` - Local Ollama instance
  - `lm_studio()` - Local LM Studio instance
  - `chat()` - Send messages to LLM
  - `stream_chat()` - Streaming responses (extensible)

#### `DocumentAgent` (src/agent.rs)
Intelligent agent for document processing:
- `new()` - Create agent with LLM provider
- `process_document()` - Analyze document with user query
- `chat()` - Maintain conversation with history
- `analyze_document()` - Generate AI-powered insights
- `summarize_multiple()` - Process multiple documents
- `get_history()` / `clear_history()` - Manage conversation

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ (Edition 2021)
- One of the following LLM providers:
  - **Ollama** (free, local): https://ollama.ai/
  - **LM Studio** (free, local): https://lmstudio.ai/
  - **OpenAI API** (requires API key): https://openai.com/

### Installation

```bash
# Clone the repository
git clone https://github.com/Saurabh182128/Agentic-AI-.git
cd Agentic-AI-

# Build the project
cargo build --release
```

### Running the Agentic Chat

```bash
# Run the interactive agent
cargo run --bin agent --release

# Or run the simple summarizer example
cargo run --bin summarizer
```

## 💬 Usage Guide

### Interactive Agent Chat

```bash
$ cargo run --bin agent

╔════════════════════════════════════════════════════════════╗
║        Agentic AI - Document Processing Assistant          ║
║           Powered by OpenAI-Compatible LLMs               ║
╚════════════════════════════════════════════════════════════╝

🔧 Select LLM Provider:
   1. Ollama (local, free)
   2. LM Studio (local, free)
   3. OpenAI API (requires API key)
```

#### Commands
- `doc` - Upload and analyze a document
- `help` - Show help message
- `clear` - Clear conversation history
- `quit` - Exit the program
- Or just type any question for general chat

### Code Examples

#### Using DocumentAgent with Local LLM

```rust
use agentic_ai::agent::DocumentAgent;
use agentic_ai::llm::OpenAILLM;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create LLM provider (local Ollama)
    let llm = Arc::new(OpenAILLM::ollama("mistral".to_string()));
    
    // Create agent
    let mut agent = DocumentAgent::new(llm);
    
    // Chat with the agent
    let response = agent.chat("What is Rust?").await.unwrap();
    println!("Response: {}", response);
}
```

#### Analyzing a Document

```rust
use agentic_ai::agent::DocumentAgent;
use agentic_ai::llm::OpenAILLM;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let llm = Arc::new(OpenAILLM::ollama("neural-chat".to_string()));
    let agent = DocumentAgent::new(llm);
    
    let document = "Your document text here...";
    let analysis = agent.analyze_document(document).await.unwrap();
    println!("{}", analysis);
}
```

#### Processing Document with Query

```rust
#[tokio::main]
async fn main() {
    let llm = Arc::new(OpenAILLM::default_openai("sk-...".to_string()));
    let mut agent = DocumentAgent::new(llm);
    
    let response = agent
        .process_document(document_content, "What are the main topics?")
        .await
        .unwrap();
    
    println!("{}", response);
}
```

## 📦 Dependencies

```toml
# Core dependencies
regex = "1.10"                           # Pattern matching
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"                       # JSON support

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# HTTP client for LLM APIs
reqwest = { version = "0.11", features = ["json"] }

# Async trait support
async-trait = "0.1"
```

## 🔄 Workflow

### Document Analysis Flow
```
1. User uploads document
2. Document parsing (sentences, word count)
3. Keyword extraction (stop-word filtering)
4. Summary generation (sentence scoring)
5. AI analysis via LLM
6. Return insights to user
```

### Chat Interaction Flow
```
1. User message
2. Add to conversation history
3. Send to LLM with full context
4. Receive and display response
5. Store in history for context
6. Next iteration with updated context
```

## 🛠️ Supported LLM Providers

### Ollama (Recommended for Local Use)
- **Setup**: Download from https://ollama.ai/
- **Models**: mistral, neural-chat, llama2, etc.
- **Endpoint**: `http://localhost:11434/v1`
- **Cost**: Free, runs locally

### LM Studio
- **Setup**: Download from https://lmstudio.ai/
- **Models**: Extensive model library
- **Endpoint**: `http://localhost:1234/v1`
- **Cost**: Free, runs locally

### OpenAI API
- **Setup**: Get API key from https://platform.openai.com/
- **Models**: GPT-3.5 Turbo, GPT-4, etc.
- **Endpoint**: `https://api.openai.com/v1`
- **Cost**: Pay-per-token

## 📊 Algorithm Overview

### Summarization Process
1. **Parsing**: Split document into sentences using regex (`[.!?]+`)
2. **Keyword Extraction**: Count word frequencies, filter stop words
3. **Sentence Scoring**: Score based on keyword occurrences
4. **Selection**: Pick top-scoring sentences while preserving order
5. **Compression**: Calculate reduction percentage

### Stop Words
Common English words filtered: articles (a, the), prepositions (in, on, at), pronouns (I, you, he), auxiliary verbs (is, are, have), etc.

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_document_creation
```

### Test Coverage
- Document creation and word counting
- Sentence parsing
- Keyword extraction
- Message creation
- LLM provider initialization

## 🎓 Examples

### Example 1: Simple Document Summarization
```bash
cargo run --bin summarizer
```

### Example 2: Interactive Chat with Document Analysis
```bash
cargo run --bin agent

# In the agent:
# Type: doc
# Paste your document (end with "END")
# Ask questions about the document
```

### Example 3: Programmatic Usage
See `src/agent_main.rs` for the full interactive example.

## 🔮 Future Enhancements

- [ ] Support for multiple languages
- [ ] TF-IDF scoring for better keyword extraction
- [ ] PDF/DOCX file I/O
- [ ] JSON export for summaries
- [ ] Web API interface (Actix/Axum)
- [ ] Streaming response support
- [ ] Document caching
- [ ] Vector embeddings for semantic search
- [ ] Fine-tuning capabilities
- [ ] Multi-document comparison

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report issues
- Suggest features
- Submit pull requests
- Improve documentation

## 📝 License

This project is currently unlicensed. 

## 👨‍💻 Author

**Saurabh182128**

---

## 🚦 Getting Started Checklist

- [ ] Clone the repository
- [ ] Install Rust (https://rustup.rs/)
- [ ] Install an LLM provider (Ollama recommended)
- [ ] Run `cargo build`
- [ ] Start the agent: `cargo run --bin agent`
- [ ] Upload a document or start chatting
- [ ] Explore the codebase

## 📚 Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Async Runtime](https://tokio.rs/)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [LM Studio](https://lmstudio.ai/)
- [OpenAI API Docs](https://platform.openai.com/docs/)

---

**Note**: This is a sophisticated implementation combining document analysis with AI-powered agentic chat. It's designed for both learning and practical use cases.
