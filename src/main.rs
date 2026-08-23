use agentic_ai::Document;

fn main() {
    // Example document about Rust
    let content = "Rust is a systems programming language that runs blazingly fast and prevents segfaults. \
        It guarantees memory safety without using garbage collection. Rust is designed to be concurrent, \
        practical, and safe. The language empowers developers to build reliable and efficient software. \
        Rust has strong static typing and pattern matching. It provides zero-cost abstractions and \
        minimal runtime overhead. Rust is used for building systems, web servers, embedded systems, and more. \
        The Rust community is welcoming and provides excellent documentation. Cargo is Rust's package manager \
        and build system. Rust code is fast, memory-safe, and thread-safe.";

    let doc = Document::new("Introduction to Rust".to_string(), content.to_string());

    println!("=== Document Information ===");
    println!("Title: {}", doc.title);
    println!("Word Count: {}", doc.word_count);
    println!();

    println!("=== Parsed Sentences ===");
    let sentences = doc.parse_sentences();
    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}. {}", i + 1, sentence);
    }
    println!();

    println!("=== Top Keywords ===");
    let keywords = doc.extract_keywords(8);
    for (i, keyword) in keywords.iter().enumerate() {
        println!("{}. {}", i + 1, keyword);
    }
    println!();

    println!("=== Document Summary ===");
    let summary = doc.summarize(3);
    println!("Summary: {}", summary.summary_text);
    println!("Key Points: {:?}", summary.key_points);
    println!("Compression Ratio: {:.2}%", summary.compression_ratio);
    println!();

    // Another example with different content
    let content2 = "Artificial Intelligence is transforming industries worldwide. \
        Machine learning models are becoming more sophisticated. Deep learning networks can process \
        vast amounts of data. AI assistants help users with various tasks. Natural language processing \
        enables machines to understand human language. Computer vision allows machines to interpret images. \
        AI ethics is crucial for responsible development. Data privacy is essential in AI systems. \
        Automation powered by AI increases efficiency. AI research continues to advance rapidly.";

    let doc2 = Document::new("AI and Machine Learning".to_string(), content2.to_string());
    println!("=== Second Document Summary ===");
    let summary2 = doc2.summarize(2);
    println!("Title: {}", summary2.title);
    println!("Summary: {}", summary2.summary_text);
    println!("Compression Ratio: {:.2}%", summary2.compression_ratio);
}
