use agentic_ai::agent::DocumentAgent;
use agentic_ai::llm::OpenAILLM;
use std::io::{self, Write};
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
    println!("   2. Ask questions about document content");
    println!("   3. Get summaries and key insights");
    println!("   4. General conversation");
    println!("\nCommands:");
    println!("   'doc'   - Paste a document for analysis");
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
                    println!("\n📖 Processing document...");
                    match agent.analyze_document(&document_content).await {
                        Ok(analysis) => {
                            println!("\n🔍 Analysis:\n{}", analysis);

                            // Ask for follow-up questions
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

                                match agent
                                    .process_document(&document_content, question)
                                    .await
                                {
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

fn show_help() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                         HELP                              ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║ Commands:                                                  ║");
    println!("║   doc   - Upload and analyze a document                   ║");
    println!("║   help  - Show this help message                          ║");
    println!("║   clear - Clear conversation history                      ║");
    println!("║   quit  - Exit the program                                ║");
    println!("║                                                            ║");
    println!("║ Features:                                                  ║");
    println!("║   • Analyze documents and extract key insights            ║");
    println!("║   • Generate summaries with compression ratios            ║");
    println!("║   • Extract keywords and important terms                  ║");
    println!("║   • Answer questions about document content               ║");
    println!("║   • Maintain conversation context                         ║");
    println!("║                                                            ║");
    println!("║ Supported LLM Providers:                                   ║");
    println!("║   • Ollama (http://localhost:11434)                       ║");
    println!("║   • LM Studio (http://localhost:1234)                     ║");
    println!("║   • OpenAI API (https://api.openai.com)                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
