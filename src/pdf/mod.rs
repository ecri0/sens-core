//! PDF text extraction and chunking module.
//!
//! Provides high-performance PDF processing using the lopdf library.
//! Includes text extraction, per-page processing, and configurable text chunking.

use crate::utils::{ChunkConfig, ChunkMetadata, Result, SensCoreError};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Information about a single page from a PDF document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Page number (1-indexed).
    pub page_number: usize,
    /// Extracted text content.
    pub text: String,
    /// Approximate word count.
    pub word_count: usize,
}

/// A chunk of text with metadata and positioning information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique identifier for this chunk.
    pub id: String,
    /// The text content of the chunk.
    pub text: String,
    /// Starting byte offset in the original text.
    pub start_offset: usize,
    /// Ending byte offset in the original text.
    pub end_offset: usize,
    /// Optional metadata.
    pub metadata: ChunkMetadata,
}

/// High-performance PDF text extractor and processor.
///
/// Uses the lopdf library to efficiently extract text from PDF documents.
/// Supports per-page extraction and flexible text chunking with overlap.
///
/// # Examples
///
/// ```no_run
/// use sens_core::pdf::PdfExtractor;
/// use std::path::Path;
///
/// let extractor = PdfExtractor::new();
/// let text = extractor.extract_text(Path::new("document.pdf"))?;
/// println!("Extracted text: {}", text);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct PdfExtractor;

impl PdfExtractor {
    /// Creates a new PDF extractor instance.
    pub fn new() -> Self {
        PdfExtractor
    }

    /// Extracts all text from a PDF file.
    ///
    /// Reads the PDF document and concatenates text from all pages into a single string.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the PDF file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the extracted text, or an error if extraction fails.
    ///
    /// # Errors
    ///
    /// Returns a `SensCoreError::PdfError` if the file cannot be read or parsed.
    pub fn extract_text<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        let document = lopdf::Document::load(path).map_err(|e| {
            SensCoreError::PdfError(format!("Failed to load PDF: {}", e))
        })?;

        extract_text_from_document(&document)
    }

    /// Extracts text from a PDF file on a per-page basis.
    ///
    /// Returns a vector of `Page` structures, each containing the text from a single page.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the PDF file.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Page` objects, or an error if extraction fails.
    ///
    /// # Errors
    ///
    /// Returns a `SensCoreError::PdfError` if the file cannot be read or parsed.
    pub fn extract_pages<P: AsRef<Path>>(path: P) -> Result<Vec<Page>> {
        let path = path.as_ref();
        let document = lopdf::Document::load(path).map_err(|e| {
            SensCoreError::PdfError(format!("Failed to load PDF: {}", e))
        })?;

        extract_pages_from_document(&document)
    }

    /// Chunks text into overlapping segments with configurable parameters.
    ///
    /// Splits text into chunks based on the provided configuration, with optional
    /// overlap between consecutive chunks to preserve context.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to chunk.
    /// * `config` - Chunking configuration including max tokens, overlap, and separator.
    ///
    /// # Returns
    ///
    /// A vector of `Chunk` objects with positioning and metadata.
    ///
    /// # Example
    ///
    /// ```
    /// use sens_core::pdf::PdfExtractor;
    /// use sens_core::utils::ChunkConfig;
    ///
    /// let text = "First sentence. Second sentence. Third sentence.";
    /// let config = ChunkConfig {
    ///     max_tokens: 20,
    ///     overlap_tokens: 5,
    ///     separator: ". ".to_string(),
    /// };
    ///
    /// let chunks = PdfExtractor::chunk_text(text, &config);
    /// assert!(!chunks.is_empty());
    /// ```
    pub fn chunk_text(text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return vec![];
        }

        let sentences: Vec<&str> = text.split(&config.separator).collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut start_offset = 0;

        for (idx, sentence) in sentences.iter().enumerate() {
            let sentence_with_sep = if idx == 0 {
                sentence.to_string()
            } else {
                format!("{}{}", config.separator, sentence)
            };

            let token_estimate = estimate_tokens(&current_chunk);

            if token_estimate > 0 && token_estimate + estimate_tokens(&sentence_with_sep) > config.max_tokens {
                let chunk_text = current_chunk.trim().to_string();
                if !chunk_text.is_empty() {
                    let end_offset = start_offset + chunk_text.len();
                    chunks.push(Chunk {
                        id: format!("chunk_{}", chunks.len()),
                        text: chunk_text.clone(),
                        start_offset,
                        end_offset,
                        metadata: ChunkMetadata::default(),
                    });

                    let overlap_chars = (config.overlap_tokens as f32 * 4.0) as usize;
                    if chunk_text.len() > overlap_chars {
                        let overlap_text = &chunk_text[chunk_text.len() - overlap_chars..];
                        current_chunk = format!("{}{}", overlap_text, config.separator);
                        start_offset = end_offset - overlap_chars;
                    } else {
                        current_chunk.clear();
                        start_offset = end_offset;
                    }
                }
            }

            current_chunk.push_str(&sentence_with_sep);
        }

        if !current_chunk.trim().is_empty() {
            let chunk_text = current_chunk.trim().to_string();
            let end_offset = start_offset + chunk_text.len();
            chunks.push(Chunk {
                id: format!("chunk_{}", chunks.len()),
                text: chunk_text,
                start_offset,
                end_offset,
                metadata: ChunkMetadata::default(),
            });
        }

        chunks
    }
}

impl Default for PdfExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts text from a parsed PDF document using lopdf's page API.
fn extract_text_from_document(document: &lopdf::Document) -> Result<String> {
    let mut text = String::new();
    let page_ids = document.get_pages();

    if page_ids.is_empty() {
        return Err(SensCoreError::PdfError(
            "No pages found in PDF".to_string(),
        ));
    }

    // Iterate pages in page-number order
    let mut sorted_pages: Vec<_> = page_ids.iter().collect();
    sorted_pages.sort_by_key(|(page_num, _)| *page_num);

    for (_page_num, &object_id) in &sorted_pages {
        if let Ok(content_bytes) = document.get_page_content(object_id) {
            extract_text_from_content(&content_bytes, &mut text);
        }
    }

    if text.is_empty() {
        return Err(SensCoreError::PdfError(
            "No text content extracted from PDF".to_string(),
        ));
    }

    Ok(text)
}

/// Extracts pages from a parsed PDF document.
fn extract_pages_from_document(document: &lopdf::Document) -> Result<Vec<Page>> {
    let mut pages = Vec::new();
    let page_ids = document.get_pages();

    if page_ids.is_empty() {
        return Err(SensCoreError::PdfError("No pages extracted".to_string()));
    }

    let mut sorted_pages: Vec<_> = page_ids.iter().collect();
    sorted_pages.sort_by_key(|(page_num, _)| *page_num);

    for (&page_num, &object_id) in &sorted_pages {
        let mut page_text = String::new();
        if let Ok(content_bytes) = document.get_page_content(object_id) {
            extract_text_from_content(&content_bytes, &mut page_text);
        }

        let word_count = page_text.split_whitespace().count();
        pages.push(Page {
            page_number: page_num as usize,
            text: page_text,
            word_count,
        });
    }

    if pages.is_empty() {
        return Err(SensCoreError::PdfError("No pages extracted".to_string()));
    }

    Ok(pages)
}

/// Extracts text from PDF content stream bytes.
///
/// Parses a raw PDF content stream to find text objects (BT...ET blocks)
/// and extracts text strings enclosed in parentheses.
fn extract_text_from_content(content: &[u8], text: &mut String) {
    let mut in_text_object = false;
    let mut i = 0;
    let bytes = content;

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'B' && bytes[i + 1] == b'T' {
            in_text_object = true;
            i += 2;
        } else if i + 1 < bytes.len() && bytes[i] == b'E' && bytes[i + 1] == b'T' {
            in_text_object = false;
            i += 2;
        } else if in_text_object && bytes[i] == b'(' {
            i += 1;
            let mut text_content = Vec::new();
            while i < bytes.len() && bytes[i] != b')' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                } else {
                    text_content.push(bytes[i]);
                    i += 1;
                }
            }
            if let Ok(extracted) = String::from_utf8(text_content) {
                text.push_str(&extracted);
                text.push(' ');
            }
            i += 1;
        } else {
            i += 1;
        }
    }
}

/// Estimates token count from text (rough approximation: 1 token ≈ 4 characters).
fn estimate_tokens(text: &str) -> usize {
    (text.len() as f32 / 4.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text_basic() {
        let text = "First. Second. Third. Fourth.";
        let config = ChunkConfig {
            max_tokens: 20,
            overlap_tokens: 5,
            separator: ". ".to_string(),
        };

        let chunks = PdfExtractor::chunk_text(text, &config);
        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| !c.text.is_empty()));
    }

    #[test]
    fn test_chunk_text_empty() {
        let text = "";
        let config = ChunkConfig::default();
        let chunks = PdfExtractor::chunk_text(text, &config);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_page_struct() {
        let page = Page {
            page_number: 1,
            text: "Sample text".to_string(),
            word_count: 2,
        };
        assert_eq!(page.page_number, 1);
        assert_eq!(page.word_count, 2);
    }

    #[test]
    fn test_chunk_struct() {
        let chunk = Chunk {
            id: "test_chunk".to_string(),
            text: "Test content".to_string(),
            start_offset: 0,
            end_offset: 12,
            metadata: ChunkMetadata::default(),
        };
        assert_eq!(chunk.text, "Test content");
        assert_eq!(chunk.end_offset - chunk.start_offset, 12);
    }
}
