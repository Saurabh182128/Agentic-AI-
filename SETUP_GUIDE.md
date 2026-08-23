# Agentic AI - Setup Guide

## 🎯 Prerequisites

Before you start, make sure you have:
- **Rust 1.70+** installed (https://rustup.rs/)
- **One LLM Provider** installed:
  - Ollama (recommended for beginners)
  - LM Studio
  - Or OpenAI API key

## 🛠️ Installation Steps

### Step 1: Clone the Repository

```bash
git clone https://github.com/Saurabh182128/Agentic-AI-.git
cd Agentic-AI-
```

### Step 2: Choose and Setup Your LLM Provider

#### Option A: Ollama (Recommended - Free & Local)

1. **Download & Install Ollama**
   - Visit https://ollama.ai/
   - Download for your OS (Windows, macOS, Linux)
   - Install and run

2. **Pull a Model**
   ```bash
   # Download Mistral (recommended, ~4GB)
   ollama pull mistral
   
   # Or try other models
   ollama pull neural-chat      # Smaller, faster
   ollama pull llama2           # Meta's LLaMA 2
   ollama pull dolphin-mixtral  # Powerful alternative
   ```

3. **Verify Installation**
   ```bash
   # Start Ollama server (usually runs in background)
   ollama serve
   
   # In another terminal, test it
   ollama run mistral "Hello, what is Rust?"
   ```

#### Option B: LM Studio (Free & Local)

1. **Download & Install LM Studio**
   - Visit https://lmstudio.ai/
   - Download for your OS
   - Install and run

2. **Load a Model**
   - Open LM Studio
   - Search for a model (e.g., "mistral", "neural-chat")
   - Download and load it
   - Go to "Local Server" tab
   - Click "Start Server"
   - Server will run on `http://localhost:1234`

#### Option C: OpenAI API

1. **Get API Key**
   - Visit https://platform.openai.com/api/keys
   - Create a new secret key
   - Copy and save it securely

2. **Set Environment Variable** (optional)
   ```bash
   # Linux/macOS
   export OPENAI_API_KEY="sk-..."
   
   # Windows (PowerShell)
   $env:OPENAI_API_KEY="sk-..."
   ```

### Step 3: Build the Project

```bash
# Build in release mode (optimized)
cargo build --release

# Or build in debug mode (faster compilation)
cargo build
```

### Step 4: Run the Interactive Agent

```bash
# Run the agentic chat
cargo run --bin agent --release

# Follow the on-screen prompts to select your LLM provider
```

## 🎮 Using the Agent

### First Run

When you run the agent for the first time:

```bash
$ cargo run --bin agent --release

╔════════════════════════════════════════════════════════════╗
║        Agentic AI - Document Processing Assistant          ║
║           Powered by OpenAI-Compatible LLMs               ║
╚════════════════════════════════════════════════════════════╝

🔧 Select LLM Provider:
   1. Ollama (local, free)
   2. LM Studio (local, free)
   3. OpenAI API (requires API key)

Choose provider (1-3) [default: 1]:
```

### Example Interaction

```
Choose provider (1-3) [default: 1]: 1
Enter Ollama model name (e.g., 'mistral', 'neural-chat') [default: mistral]: mistral
✓ Using Ollama with model: mistral

📚 Welcome! I can help you with:
   1. Chat about documents (type 'doc' to paste a document)
   2. Ask questions about document content
   3. Get summaries and key insights
   4. General conversation

Commands:
   'doc'   - Paste a document for analysis
   'help'  - Show help
   'quit'  - Exit the program
   'clear' - Clear conversation history

🤖 You: What is machine learning?
🤖 Assistant: Machine learning is a subset of artificial intelligence that enables systems to learn and improve from experience without being explicitly programmed...

🤖 You: doc
📝 Enter your document (type 'END' on a new line when finished):
─────────────────────────────────────────
Machine learning is a powerful technique in AI. It enables systems to learn patterns from data.
Supervised learning uses labeled data. Unsupervised learning finds patterns without labels.
END
✓ Document received (87 words)

📖 Processing document...

🔍 Analysis:
The document discusses machine learning fundamentals...

❓ Ask a question about this document (or 'done' to finish): What are the main topics?
```

## 📋 Troubleshooting

### Problem: Connection refused to Ollama

**Solution:**
```bash
# Make sure Ollama is running
ollama serve

# In another terminal, verify connection
curl http://localhost:11434/api/tags
```

### Problem: Model not found in Ollama

**Solution:**
```bash
# List available models
ollama list

# Pull the model
ollama pull mistral

# Verify
ollama run mistral "test"
```

### Problem: LM Studio server not responding

**Solution:**
1. Open LM Studio
2. Go to "Local Server" tab
3. Click "Start Server"
4. Verify it says "Server is running on http://localhost:1234"

### Problem: OpenAI API key invalid

**Solution:**
1. Verify your key from https://platform.openai.com/api/keys
2. Make sure there are no extra spaces
3. Check that your account has available credits
4. Try creating a new key

### Problem: Cargo build fails

**Solution:**
```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build --release

# Check Rust version
rustc --version  # Should be 1.70+
```

## 📊 Performance Tips

### Optimize for Speed
```bash
# Use faster models
ollama pull neural-chat  # ~5GB, faster responses

# Run in release mode
cargo run --bin agent --release
```

### Optimize for Quality
```bash
# Use larger models
ollama pull mistral         # ~7GB
ollama pull dolphin-mixtral # ~26GB, most capable

# Set temperature lower for focused responses
# (Configure in src/llm.rs if needed)
```

## 🔧 Environment Variables (Optional)

Create a `.env` file in the project root:

```env
# Ollama configuration
OLLAMA_MODEL=mistral
OLLAMA_BASE_URL=http://localhost:11434/v1

# LM Studio configuration
LM_STUDIO_MODEL=neural-chat
LM_STUDIO_BASE_URL=http://localhost:1234/v1

# OpenAI configuration
OPENAI_API_KEY=sk-your-key-here
OPENAI_MODEL=gpt-3.5-turbo
```

## 🧪 Testing the Setup

### Test 1: Simple Chat
```bash
cargo run --bin agent --release
# Type: "Hello, introduce yourself"
# Expected: Agent responds with introduction
```

### Test 2: Document Analysis
```bash
cargo run --bin agent --release
# Type: "doc"
# Paste: "Rust is a systems programming language."
# Type: "What is the main topic?"
# Expected: Agent analyzes and responds
```

### Test 3: Unit Tests
```bash
cargo test
# All tests should pass
```

## 📚 Next Steps

1. **Explore Document Analysis**
   - Try uploading different document types
   - Experiment with various queries
   - Notice how summaries work

2. **Learn the Codebase**
   - Read `src/lib.rs` - Document processing
   - Read `src/llm.rs` - LLM integration
   - Read `src/agent.rs` - Agent logic

3. **Customize for Your Needs**
   - Modify system prompts in `src/agent.rs`
   - Add custom commands in `src/agent_main.rs`
   - Extend with new features

4. **Integrate with Your Project**
   - Use as a library: `cargo add agentic-ai`
   - Build custom agents
   - Combine with other tools

## 🎓 Learning Resources

### Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### LLMs & AI
- [Ollama Documentation](https://github.com/ollama/ollama)
- [LM Studio Docs](https://lmstudio.ai/docs/)
- [OpenAI API Documentation](https://platform.openai.com/docs/)

### Project-Specific
- Check `examples/` directory for sample code
- Read inline code comments
- Explore test cases in `src/`

## 🆘 Getting Help

### If Something Goes Wrong

1. **Check the logs**
   ```bash
   cargo run --bin agent -- --verbose
   ```

2. **Clean and rebuild**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Verify dependencies**
   ```bash
   cargo update
   cargo check
   ```

4. **Check GitHub Issues**
   - Look for similar problems
   - Create a new issue with details

## ✅ Verification Checklist

After setup, verify everything works:

- [ ] Rust is installed (`rustc --version`)
- [ ] LLM provider is running
- [ ] Project builds (`cargo build --release`)
- [ ] Unit tests pass (`cargo test`)
- [ ] Agent runs (`cargo run --bin agent`)
- [ ] Can chat with agent
- [ ] Can upload and analyze documents
- [ ] Conversation history works

---

**You're all set! Enjoy using Agentic AI! 🚀**
