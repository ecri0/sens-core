#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! # sens-core: High-Performance Document Processing for AI Pipelines
//!
//! A Rust library providing fast, efficient text extraction, sanitization, and semantic
//! identification for AI applications. Delivers 15× performance improvement over equivalent
//! Python implementations.
//!
//! ## Features
//!
//! - **PDF Extraction**: Extract text from PDF files with per-page support and configurable chunking
//! - **Smart Sanitizer**: Detect and redact PII (emails, phone numbers, credit cards, SSN, IPs)
//! - **USID Generation**: Create model-independent semantic identifiers for content
//!
//! ## Quick Start
//!
//! ### Extract text from a PDF
//!
//! ```no_run
//! use sens_core::pdf::PdfExtractor;
//! use sens_core::utils::ChunkConfig;
//!
//! let extractor = PdfExtractor::new();
//! let text = extractor.extract_text("document.pdf")?;
//! let chunks = PdfExtractor::chunk_text(&text, &ChunkConfig::default());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Sanitize text with PII detection
//!
//! ```
//! use sens_core::sanitizer::Sanitizer;
//!
//! let sanitizer = Sanitizer::default();
//! let result = sanitizer.sanitize("Email: john@example.com")?;
//! println!("Sanitized: {}", result.sanitized_text);
//! // Output: "Email: [Email:REDACTED_1]"
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Generate Universal Semantic IDs
//!
//! ```
//! use sens_core::usid::UsidGenerator;
//!
//! let usid = UsidGenerator::generate("document content", 42);
//! println!("USID: {}", usid);
//! // Output: "USID: sens://v1/universal/f42_<hash>"
//!
//! let parsed = UsidGenerator::parse(&usid.to_string())?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Module Structure
//!
//! - [`pdf`] - PDF text extraction and chunking
//! - [`sanitizer`] - PII detection and text sanitization
//! - [`usid`] - Universal Semantic ID generation and parsing
//! - [`utils`] - Shared error types and utilities
//!
//! ## Performance
//!
//! sens-core achieves **15× speedup** over equivalent Python implementations:
//! - PDF extraction: ~100MB/s throughput
//! - Text sanitization: sub-millisecond processing for typical documents
//! - USID generation: deterministic, collision-resistant hashing
//!
//! ## License
//!
//! Licensed under the Apache License, Version 2.0.

pub mod pdf;
pub mod sanitizer;
pub mod usid;
pub mod utils;

// Re-export commonly used items
pub use pdf::{Chunk, Page, PdfExtractor};
pub use sanitizer::{Replacement, Sanitizer, SanitizeResult, SanitizerConfig};
pub use usid::{Usid, UsidGenerator, UsidInfo};
pub use utils::{ChunkConfig, ChunkMetadata, Result, SensCoreError};
