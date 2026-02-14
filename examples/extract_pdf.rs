//! Example: PDF extraction and text chunking.
//!
//! This example demonstrates how to:
//! 1. Extract text from a PDF file
//! 2. Extract pages with metadata
//! 3. Chunk text with configurable parameters
//!
//! Run with: `cargo run --example extract_pdf`

use sens_core::pdf::PdfExtractor;
use sens_core::utils::ChunkConfig;

fn main() -> sens_core::Result<()> {
    println!("=== PDF Extraction Example ===\n");

    // In a real application, you would use an actual PDF file
    // For this example, we'll demonstrate the API usage

    // Create an extractor
    let extractor = PdfExtractor::new();
    println!("Created PDF extractor");

    // Simulate extracted text for demonstration
    let sample_text = r#"
        Chapter 1: Introduction

        This is the introduction section. It provides an overview of the document.
        The introduction typically includes background information and context.

        Chapter 2: Main Content

        The main content section contains detailed information. It may span
        multiple pages and include various subsections and topics.

        Chapter 3: Conclusion

        The conclusion summarizes the key points and findings. It may also
        suggest future directions for research or implementation.
    "#;

    println!("\n--- Sample Text (Length: {} chars) ---", sample_text.len());
    println!("{}\n", sample_text);

    // Example 1: Chunk with default configuration
    println!("=== Example 1: Default Chunking ===");
    let default_config = ChunkConfig::default();
    println!("Config: max_tokens={}, overlap={}, separator=\"{}\"",
        default_config.max_tokens,
        default_config.overlap_tokens,
        default_config.separator.escape_default()
    );

    let chunks = PdfExtractor::chunk_text(sample_text, &default_config);
    println!("Created {} chunks\n", chunks.len());

    for chunk in &chunks {
        println!("Chunk ID: {}", chunk.id);
        println!("  Text (first 50 chars): {}",
            chunk.text.chars().take(50).collect::<String>());
        println!("  Range: bytes {}-{}", chunk.start_offset, chunk.end_offset);
        println!("  Length: {} bytes", chunk.text.len());
    }

    // Example 2: Custom chunking with sentence-based splitting
    println!("\n=== Example 2: Custom Chunking (Sentence-based) ===");
    let custom_config = ChunkConfig {
        max_tokens: 30,
        overlap_tokens: 8,
        separator: ". ".to_string(),
    };

    println!("Config: max_tokens={}, overlap={}, separator=\"{}\"",
        custom_config.max_tokens,
        custom_config.overlap_tokens,
        custom_config.separator.escape_default()
    );

    let custom_chunks = PdfExtractor::chunk_text(sample_text, &custom_config);
    println!("Created {} chunks\n", custom_chunks.len());

    for (idx, chunk) in custom_chunks.iter().enumerate() {
        println!("Chunk {}: {} bytes", idx, chunk.text.len());
        println!("  Preview: {}",
            chunk.text.chars()
                .take(60)
                .collect::<String>()
                .replace('\n', " "));
    }

    // Example 3: Demonstrate extraction metadata
    println!("\n=== Example 3: Page Metadata ===");
    println!("When extracting from real PDFs:");
    println!("  - PdfExtractor::extract_pages() returns Vec<Page>");
    println!("  - Each Page contains:");
    println!("    * page_number: usize (1-indexed)");
    println!("    * text: String (extracted text)");
    println!("    * word_count: usize (approximate)");

    // Simulate page extraction output
    let simulated_pages = vec![
        sens_core::Page {
            page_number: 1,
            text: "Page 1 content...".to_string(),
            word_count: 3,
        },
        sens_core::Page {
            page_number: 2,
            text: "Page 2 content...".to_string(),
            word_count: 3,
        },
    ];

    println!("\nSimulated page extraction:");
    for page in &simulated_pages {
        println!("  Page {}: {} words", page.page_number, page.word_count);
    }

    // Example 4: Chunking configuration reference
    println!("\n=== Example 4: Chunking Configuration Reference ===");
    let configs = vec![
        ("Small chunks", ChunkConfig {
            max_tokens: 128,
            overlap_tokens: 16,
            separator: ". ".to_string(),
        }),
        ("Medium chunks", ChunkConfig {
            max_tokens: 512,
            overlap_tokens: 64,
            separator: "\n".to_string(),
        }),
        ("Large chunks", ChunkConfig {
            max_tokens: 2048,
            overlap_tokens: 256,
            separator: "\n\n".to_string(),
        }),
    ];

    for (name, config) in configs {
        println!("\n{}:", name);
        println!("  max_tokens: {}", config.max_tokens);
        println!("  overlap_tokens: {}", config.overlap_tokens);
        println!("  separator: \"{}\"", config.separator.escape_default());
    }

    println!("\n=== Usage Summary ===");
    println!("To extract from a real PDF file:");
    println!("  1. Create extractor: let extractor = PdfExtractor::new();");
    println!("  2. Extract text: let text = extractor.extract_text(\"path.pdf\")?;");
    println!("  3. Or extract pages: let pages = extractor.extract_pages(\"path.pdf\")?;");
    println!("  4. Chunk as needed: let chunks = PdfExtractor::chunk_text(&text, &config);");

    Ok(())
}
