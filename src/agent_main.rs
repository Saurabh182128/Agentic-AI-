use agentic_ai::agent::DocumentAgent;
use agentic_ai::llm::OpenAILLM;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Agentic AI - Document Processing Assistant          ║");
    println!("║           Powered by OpenAI-Compatible LLMs               ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize LLM provider
    let llm_provider = select_llm_provider().await;

    // Create agent
    let mut agent = DocumentAgent::new(Arc::new(llm_provider));

    println!("\n📚 Welcome! I can help you with:");
    println!("   1. Chat about documents (type 'doc' to paste a document)");
    println!("   2. Analyze a local text file (type 'file')");
    println!("   3. Summarize a folder of documents (type 'batch')");
    println!("   4. Ask questions about document content");
    println!("   5. General conversation");
    println!("\nCommands:");
    println!("   'doc'   - Paste a document for analysis");
    println!("   'file'  - Analyze a local .txt/.md/.rst file");
    println!("   'batch' - Summarize all supported files in a folder");
    println!("   'help'  - Show help");
    println!("   'quit'  - Exit the program");
    println!("   'clear' - Clear conversation history");
    println!();

    loop {
        print!("\n🤖 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("\n👋 Goodbye! Thanks for using Agentic AI.");
                break;
            }
            "help" => {
                show_help();
            }
            "clear" => {
                agent.clear_history();
                println!("✓ Conversation history cleared.");
            }
            "doc" => {
                if let Ok(document_content) = read_document_input() {
                    process_document_analysis(&mut agent, &document_content).await;
                }
            }
            "file" => {
                match read_path_input("Enter the text file path to analyze: ") {
                    Ok(path) => {
                        match fs::read_to_string(&path) {
                            Ok(document_content) => {
                                println!("\n📖 Processing file: {}", path);
                                process_document_analysis(&mut agent, &document_content).await;
                            }
                            Err(e) => println!("❌ Error reading file '{}': {}", path, e),
                        }
                    }
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
            "batch" => {
                match read_path_input("Enter a folder path containing text files (.txt/.md/.rst): ") {
                    Ok(path) => {
                        match collect_document_files(&path) {
                            Ok(documents) => {
                                println!("\n📚 Found {} supported files in {}", documents.len(), path);
                                let batches: Vec<(&str, &str)> = documents
                                    .iter()
                                    .map(|(title, content)| (title.as_str(), content.as_str()))
                                    .collect();
                                match agent.summarize_multiple(batches).await {
                                    Ok(summary) => {
                                        println!("\n🔍 Batch Summary:\n{}", summary);
                                    }
                                    Err(e) => println!("❌ Error summarizing batch: {}", e),
                                }
                            }
                            Err(e) => println!("❌ Error reading batch: {}", e),
                        }
                    }
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
            "" => continue,
            user_message => {
                print!("\n🤖 Assistant: ");
                io::stdout().flush().unwrap();

                match agent.chat(user_message).await {
                    Ok(response) => {
                        println!("{}", response);
                    }
                    Err(e) => {
                        println!("❌ Error: {}", e);
                    }
                }
            }
        }
    }
}

async fn select_llm_provider() -> OpenAILLM {
    println!("\n🔧 Select LLM Provider:");
    println!("   1. Ollama (local, free)");
    println!("   2. LM Studio (local, free)");
    println!("   3. OpenAI API (requires API key)");
    println!();

    print!("Choose provider (1-3) [default: 1]: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "2" => {
            print!("Enter model name (e.g., 'neural-chat', 'mistral'): ");
            io::stdout().flush().unwrap();

            let mut model = String::new();
            io::stdin().read_line(&mut model).unwrap();
            let model = model.trim().to_string();

            println!("✓ Using LM Studio with model: {}", model);
            OpenAILLM::lm_studio(model)
        }
        "3" => {
            print!("Enter your OpenAI API key: ");
            io::stdout().flush().unwrap();

            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key).unwrap();
            let api_key = api_key.trim().to_string();

            println!("✓ Using OpenAI API");
            OpenAILLM::default_openai(api_key)
        }
        _ => {
            print!("Enter Ollama model name (e.g., 'mistral', 'neural-chat') [default: mistral]: ");
            io::stdout().flush().unwrap();

            let mut model = String::new();
            io::stdin().read_line(&mut model).unwrap();
            let model = model.trim().to_string();
            let model = if model.is_empty() {
                "mistral".to_string()
            } else {
                model
            };

            println!("✓ Using Ollama with model: {}", model);
            OpenAILLM::ollama(model)
        }
    }
}

fn read_document_input() -> io::Result<String> {
    println!("\n📝 Enter your document (type 'END' on a new line when finished):");
    println!("─────────────────────────────────────────");

    let mut document = String::new();
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;

        if line.trim() == "END" {
            break;
        }

        document.push_str(&line);
    }

    if document.is_empty() {
        println!("⚠️  Empty document. Please provide some content.");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty document",
        ));
    }

    Ok(document)
}

fn read_path_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let cleaned = input.trim();

    if cleaned.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "A file or directory path is required",
        ));
    }

    Ok(cleaned.to_string())
}

fn is_supported_document_extension(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(ext.as_str(), "txt" | "md" | "markdown" | "rst")
}

fn collect_document_files(path: &str) -> io::Result<Vec<(String, String)>> {
    let root = Path::new(path);

    if root.is_file() {
        if !is_supported_document_extension(root) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file type. Use .txt, .md, or .rst",
            ));
        }

        let title = root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("document")
            .to_string();
        let content = fs::read_to_string(root)?;
        return Ok(vec![(title, content)]);
    }

    if root.is_dir() {
        let mut docs = Vec::new();
        let mut entries = fs::read_dir(root)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let candidate = entry.path();
            if !candidate.is_file() || !is_supported_document_extension(&candidate) {
                continue;
            }

            let title = candidate
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("document")
                .to_string();
            let content = fs::read_to_string(&candidate)?;
            docs.push((title, content));
        }

        if docs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No supported text files were found in that directory",
            ));
        }

        return Ok(docs);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Path '{}' does not exist or is not a file/directory", path),
    ))
}

async fn process_document_analysis(agent: &mut DocumentAgent, document_content: &str) {
    println!("\n📖 Processing document...");
    match agent.analyze_document(document_content).await {
        Ok(analysis) => {
            println!("\n🔍 Analysis:\n{}", analysis);

            loop {
                print!("\n❓ Ask a question about this document (or 'done' to finish): ");
                io::stdout().flush().unwrap();

                let mut question = String::new();
                if io::stdin().read_line(&mut question).is_err() {
                    println!("Error reading input");
                    continue;
                }

                let question = question.trim();

                if question.to_lowercase() == "done" {
                    break;
                }

                match agent.process_document(document_content, question).await {
                    Ok(response) => {
                        println!("\n🤖 Assistant: {}", response);
                    }
                    Err(e) => {
                        println!("❌ Error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Error analyzing document: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::collect_document_files;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_collect_document_files_collects_supported_text_files() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("agentic_ai_batch_{unique}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("alpha.md"), "Alpha doc").unwrap();
        fs::write(dir.join("beta.txt"), "Beta doc").unwrap();
        fs::write(dir.join("ignore.bin"), [0u8; 4]).unwrap();

        let files = collect_document_files(dir.to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|(title, _)| title.contains("alpha")));
        assert!(files.iter().any(|(title, _)| title.contains("beta")));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_collect_document_files_rejects_empty_directory() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("agentic_ai_empty_{unique}"));
        fs::create_dir_all(&dir).unwrap();

        let files = collect_document_files(dir.to_str().unwrap());
        assert!(files.is_err());

        fs::remove_dir_all(dir).unwrap();
    }
}

fn show_help() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                         HELP                              ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║ Commands:                                                  ║");
    println!("║   doc   - Upload and analyze a document                   ║");
    println!("║   file  - Analyze a local .txt/.md/.rst file               ║");
    println!("║   batch - Summarize all supported files in a folder        ║");
    println!("║   help  - Show this help message                          ║");
    println!("║   clear - Clear conversation history                      ║");
    println!("║   quit  - Exit the program                                ║");
    println!("║                                                            ║");
    println!("║ Features:                                                  ║");
    println!("║   • Analyze documents and extract key insights            ║");
    println!("║   • Generate summaries with compression ratios            ║");
    println!("║   • Extract keywords and important terms                  ║");
    println!("║   • Answer questions about document content               ║");
    println!("║   • Analyze files or folders in bulk                      ║");
    println!("║   • Maintain conversation context                         ║");
    println!("║                                                            ║");
    println!("║ Supported LLM Providers:                                   ║");
    println!("║   • Ollama (http://localhost:11434)                       ║");
    println!("║   • LM Studio (http://localhost:1234)                     ║");
    println!("║   • OpenAI API (https://api.openai.com)                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
