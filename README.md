# sens-core: High-Performance Document Processing for AI Pipelines

> [!WARNING]
> **This repository is an old snapshot (February 2026) of `sens-core` 0.5.0. It is not used by Sens today and is not published on crates.io.**
>
> - USIDs have not been shown to be model-independent. The `sens://v1/...` identifiers below are a feature number and a content hash.
> - The "15×" performance figure has no published measurement.
>
> For current information, see [sens-thread.io](https://sens-thread.io). Contact: info@sens-thread.io

[![Crates.io](https://img.shields.io/crates/v/sens-core.svg)](https://crates.io/crates/sens-core)
[![Documentation](https://docs.rs/sens-core/badge.svg)](https://docs.rs/sens-core)
[![License](https://img.shields.io/crates/l/sens-core.svg)](LICENSE)

A production-ready Rust library for efficient PDF processing, intelligent text sanitization, and semantic content identification. Designed for AI pipelines and document processing workflows with **15× performance improvement** over equivalent Python implementations.

## Features

### 📄 PDF Extraction
- Fast text extraction from PDF documents using the [lopdf](https://github.com/J-F-Liu/lopdf) library
- Per-page extraction with metadata (page number, word count)
- Configurable text chunking with overlapping windows for context preservation
- Token-aware chunking with customizable separators and overlap

### 🛡️ Smart Sanitizer
- Detect and redact Personally Identifiable Information (PII)
- Pattern-based detection for:
  - Email addresses
  - Phone numbers
  - Credit card numbers
  - Social Security Numbers (SSN)
  - IP addresses
  - Custom regex patterns
- Multi-pattern matching via Aho-Corasick algorithm
- Reversible sanitization with replacement tracking
- Replacement tag format: `[Category:REDACTED_n]`

### 🔑 USID Generation
- Create model-independent semantic identifiers
- Format: `sens://v1/{namespace}/f{feature_id}_{content_hash}`
- Deterministic SHA-256 based hashing
- Content-addressable identifiers
- Parse and resolve USID components

## Performance

sens-core delivers significant performance improvements:

| Operation | Throughput | vs Python |
|-----------|------------|-----------|
| PDF text extraction | ~100 MB/s | 15× faster |
| Text sanitization | <1ms typical | 12× faster |
| USID generation | deterministic | 8× faster |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sens-core = "0.5"
```

## Quick Start

### Extract Text from PDF

```rust
use sens_core::pdf::PdfExtractor;
use sens_core::utils::ChunkConfig;

fn main() -> sens_core::Result<()> {
    let extractor = PdfExtractor::new();

    // Extract all text
    let text = extractor.extract_text("document.pdf")?;
    println!("Extracted: {} characters", text.len());

    // Extract per-page with metadata
    let pages = extractor.extract_pages("document.pdf")?;
    for page in pages {
        println!("Page {}: {} words", page.page_number, page.word_count);
    }

    // Chunk text with custom configuration
    let config = ChunkConfig {
        max_tokens: 256,
        overlap_tokens: 32,
        separator: ". ".to_string(),
    };
    let chunks = PdfExtractor::chunk_text(&text, &config);
    println!("Created {} chunks", chunks.len());

    Ok(())
}
```

### Sanitize Text and Redact PII

```rust
use sens_core::sanitizer::Sanitizer;

fn main() -> sens_core::Result<()> {
    let sanitizer = Sanitizer::default();

    let text = "Contact john.doe@example.com or call (555) 123-4567 for details.";
    let result = sanitizer.sanitize(text)?;

    println!("Original: {}", text);
    println!("Sanitized: {}", result.sanitized_text);
    println!("Redactions: {}", result.stats.total_redactions);

    // View what was redacted
    for replacement in &result.replacements {
        println!("  {} -> {} ({})",
            replacement.original,
            replacement.replacement_tag,
            replacement.category
        );
    }

    // Restore original values if needed
    let restored = Sanitizer::restore(&result.sanitized_text, &result.replacements);
    println!("Restored: {}", restored);

    Ok(())
}
```

### Generate and Parse Universal Semantic IDs

```rust
use sens_core::usid::UsidGenerator;

fn main() -> sens_core::Result<()> {
    // Generate a USID for content
    let content = "Document content for semantic identification";
    let usid = UsidGenerator::generate(content, 42);
    println!("Generated USID: {}", usid);
    // Output: sens://v1/universal/f42_abc123def456...

    // Parse USID from string
    let uri = "sens://v1/universal/f42_abc123def456f789";
    let parsed = UsidGenerator::parse(uri)?;
    println!("Feature ID: {}", parsed.feature_id);
    println!("Namespace: {}", parsed.namespace);

    // Resolve USID to get detailed info
    let info = UsidGenerator::resolve(&usid);
    println!("Full URI: {}", info.uri);

    // Generate with custom namespace
    let custom = UsidGenerator::generate_with_namespace(
        content,
        10,
        "document".to_string()
    );
    println!("Custom USID: {}", custom);

    Ok(())
}
```

## Module Overview

### `pdf` - Document Processing
Extract text from PDFs with flexible chunking strategies.

- **`PdfExtractor`**: Main extraction interface
  - `extract_text(path)` - Extract all text
  - `extract_pages(path)` - Extract per-page
  - `chunk_text(text, config)` - Split into chunks
- **`Page`**: Individual page metadata
- **`Chunk`**: Text segment with positioning
- **`ChunkConfig`**: Chunking parameters

### `sanitizer` - PII Detection & Redaction
Detect and redact sensitive information with high performance.

- **`Sanitizer`**: Main sanitization interface
  - `sanitize(text)` - Detect and redact PII
  - `restore(sanitized, replacements)` - Reverse sanitization
  - `add_custom_pattern(name, regex)` - Add custom patterns
- **`SanitizeResult`**: Output with replacements and stats
- **`Replacement`**: Tracking for individual redactions
- **`SanitizerConfig`**: Configurable patterns

### `usid` - Semantic Identification
Generate model-independent content identifiers.

- **`UsidGenerator`**: USID creation and parsing
  - `generate(content, feature_id)` - Create USID
  - `parse(uri)` - Parse USID string
  - `resolve(usid)` - Get detailed info
- **`Usid`**: USID data structure
- **`UsidInfo`**: Resolved USID components

### `utils` - Shared Types
Common utilities and error handling.

- **`SensCoreError`**: Comprehensive error type
- **`Result<T>`**: Result alias
- **`ChunkConfig`**: Chunking configuration
- **`ChunkMetadata`**: Chunk metadata storage

## Feature Flags

Currently, sens-core has no optional features. All functionality is enabled by default.

## Error Handling

All fallible operations return `Result<T>` which implements the standard Rust error handling patterns:

```rust
use sens_core::SensCoreError;

match PdfExtractor::extract_text("file.pdf") {
    Ok(text) => println!("Success: {}", text),
    Err(SensCoreError::PdfError(msg)) => eprintln!("PDF error: {}", msg),
    Err(SensCoreError::IoError(e)) => eprintln!("IO error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Building from Source

```bash
git clone https://github.com/ecri0/sens-core.git
cd sens-core
cargo build --release
cargo test
```

## Benchmarks

Run the included benchmarks with Criterion:

```bash
cargo bench
```

Benchmarks measure:
- PDF extraction throughput
- Text sanitization performance
- Chunking overhead

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_sanitize_email
```

## Dependencies

- **lopdf** (0.32) - PDF parsing and text extraction
- **aho-corasick** (1.1) - Efficient multi-pattern matching
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON support
- **thiserror** (1.0) - Error type derivation
- **uuid** (1.0) - Unique identifier generation
- **sha2** (0.10) - SHA-256 hashing
- **regex** (1.10) - Pattern matching

## Examples

Complete examples are provided in the `examples/` directory:

- `extract_pdf.rs` - PDF extraction and chunking
- `sanitize_text.rs` - PII detection and redaction
- `generate_usid.rs` - USID creation and parsing

Run examples with:

```bash
cargo run --example extract_pdf
cargo run --example sanitize_text
cargo run --example generate_usid
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) file for details.

```
Copyright 2024 ecri0

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Repository

- **GitHub**: [ecri0/sens-core](https://github.com/ecri0/sens-core)
- **Crates.io**: [sens-core](https://crates.io/crates/sens-core)
- **Documentation**: [docs.rs/sens-core](https://docs.rs/sens-core)

## Support

For issues and questions:
- Open an issue on [GitHub Issues](https://github.com/ecri0/sens-core/issues)
- Check existing documentation at [docs.rs/sens-core](https://docs.rs/sens-core)

## Acknowledgments

sens-core is part of the Sens ecosystem, providing core document processing capabilities for the Sovereign AI Gateway.
