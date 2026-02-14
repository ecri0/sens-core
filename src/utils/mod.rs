//! Shared utilities and error types for sens-core.

use thiserror::Error;

/// The main error type for sens-core operations.
#[derive(Error, Debug)]
pub enum SensCoreError {
    /// PDF parsing or extraction failed.
    #[error("PDF extraction error: {0}")]
    PdfError(String),

    /// Text sanitization failed.
    #[error("Sanitization error: {0}")]
    SanitizationError(String),

    /// USID generation or parsing failed.
    #[error("USID error: {0}")]
    UsidError(String),

    /// IO error during file operations.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid input or configuration.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Serialization or deserialization error.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Generic error.
    #[error("{0}")]
    Other(String),
}

/// Result type alias for sens-core operations.
pub type Result<T> = std::result::Result<T, SensCoreError>;

/// Configuration for text chunking operations.
#[derive(Debug, Clone)]
pub struct ChunkConfig {
    /// Maximum tokens per chunk (approximate).
    pub max_tokens: usize,
    /// Number of tokens to overlap between chunks.
    pub overlap_tokens: usize,
    /// Text separator for chunking (e.g., "\n" or ". ").
    pub separator: String,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            overlap_tokens: 50,
            separator: "\n".to_string(),
        }
    }
}

/// Metadata associated with a text chunk.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkMetadata {
    /// Source document identifier.
    pub source_id: Option<String>,
    /// Page number if applicable.
    pub page_number: Option<usize>,
    /// Creation timestamp (ISO 8601).
    pub created_at: Option<String>,
    /// Additional custom metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl Default for ChunkMetadata {
    fn default() -> Self {
        Self {
            source_id: None,
            page_number: None,
            created_at: None,
            custom: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_config_default() {
        let config = ChunkConfig::default();
        assert_eq!(config.max_tokens, 512);
        assert_eq!(config.overlap_tokens, 50);
    }
}
