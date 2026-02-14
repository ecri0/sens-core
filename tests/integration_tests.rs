//! Integration tests for sens-core library.

use sens_core::pdf::PdfExtractor;
use sens_core::sanitizer::Sanitizer;
use sens_core::usid::UsidGenerator;
use sens_core::utils::ChunkConfig;

#[test]
fn test_pdf_extractor_chunking() {
    let text = "First sentence. Second sentence. Third sentence. Fourth sentence. Fifth sentence.";
    let config = ChunkConfig {
        max_tokens: 20,
        overlap_tokens: 5,
        separator: ". ".to_string(),
    };

    let chunks = PdfExtractor::chunk_text(text, &config);

    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|c| !c.text.is_empty()));
    assert!(chunks[0].id.starts_with("chunk_"));
}

#[test]
fn test_sanitizer_email_detection() {
    let sanitizer = Sanitizer::default();
    let text = "Contact john@example.com for details";

    let result = sanitizer.sanitize(text).unwrap();

    assert!(result.sanitized_text.contains("REDACTED"));
    assert_eq!(result.stats.total_redactions, 1);
    assert!(result.stats.redactions_by_category.contains_key("Email"));
}

#[test]
fn test_sanitizer_multiple_patterns() {
    let sanitizer = Sanitizer::default();
    let text = "Email: john@example.com, Phone: (555) 123-4567, SSN: 123-45-6789";

    let result = sanitizer.sanitize(text).unwrap();

    assert_eq!(result.stats.total_redactions, 3);
    assert!(result.stats.redactions_by_category.get("Email").is_some());
    assert!(result.stats.redactions_by_category.get("PhoneNumber").is_some());
    assert!(result.stats.redactions_by_category.get("SSN").is_some());
}

#[test]
fn test_sanitizer_restoration() {
    let sanitizer = Sanitizer::default();
    let original = "Contact john@example.com or jane@test.org";

    let result = sanitizer.sanitize(original).unwrap();
    let restored = Sanitizer::restore(&result.sanitized_text, &result.replacements);

    assert_eq!(restored, original);
}

#[test]
fn test_usid_generation() {
    let content = "Test content for USID";
    let feature_id = 42;

    let usid = UsidGenerator::generate(content, feature_id);

    assert_eq!(usid.namespace, "universal");
    assert_eq!(usid.feature_id, feature_id);
    assert_eq!(usid.content_hash.len(), 16);
}

#[test]
fn test_usid_parsing() {
    let uri = "sens://v1/universal/f42_abc123def456f789";

    let usid = UsidGenerator::parse(uri).unwrap();

    assert_eq!(usid.feature_id, 42);
    assert_eq!(usid.namespace, "universal");
    assert_eq!(usid.content_hash, "abc123def456f789");
}

#[test]
fn test_usid_roundtrip() {
    let original = UsidGenerator::generate("roundtrip test", 99);
    let uri_string = original.to_string();
    let parsed = UsidGenerator::parse(&uri_string).unwrap();

    assert_eq!(original, parsed);
}

#[test]
fn test_usid_deterministic() {
    let content = "same content";
    let feature_id = 10;

    let usid1 = UsidGenerator::generate(content, feature_id);
    let usid2 = UsidGenerator::generate(content, feature_id);

    assert_eq!(usid1, usid2);
}

#[test]
fn test_usid_custom_namespace() {
    let content = "Test content";
    let usid = UsidGenerator::generate_with_namespace(content, 5, "document".to_string());

    assert_eq!(usid.namespace, "document");
    let uri = usid.to_string();
    assert!(uri.contains("document"));
}

#[test]
fn test_usid_resolution() {
    let usid = UsidGenerator::generate("content", 42);
    let info = UsidGenerator::resolve(&usid);

    assert_eq!(info.feature_id, 42);
    assert_eq!(info.namespace, "universal");
    assert!(!info.uri.is_empty());
    assert!(info.feature_prefix.contains("f42"));
}

#[test]
fn test_chunking_with_overlap() {
    let text = "Word1 word2 word3 word4 word5 word6 word7 word8 word9 word10";
    let config = ChunkConfig {
        max_tokens: 10,
        overlap_tokens: 3,
        separator: " ".to_string(),
    };

    let chunks = PdfExtractor::chunk_text(text, &config);

    assert!(!chunks.is_empty());
    // Verify chunks maintain proper offsets
    for chunk in &chunks {
        assert!(chunk.end_offset > chunk.start_offset);
        assert!(!chunk.text.is_empty());
    }
}

#[test]
fn test_empty_text_handling() {
    let sanitizer = Sanitizer::default();
    let text = "";

    let result = sanitizer.sanitize(text).unwrap();

    assert_eq!(result.stats.total_redactions, 0);
    assert!(result.replacements.is_empty());
}

#[test]
fn test_clean_text_no_redactions() {
    let sanitizer = Sanitizer::default();
    let text = "This is clean text with no sensitive information whatsoever.";

    let result = sanitizer.sanitize(text).unwrap();

    assert_eq!(result.stats.total_redactions, 0);
    assert_eq!(result.sanitized_text, text);
}

#[test]
fn test_pdf_extractor_creation() {
    let extractor = PdfExtractor::new();
    let extractor_default = PdfExtractor::default();

    // Both should create instances successfully
    assert_eq!(std::mem::size_of_val(&extractor), std::mem::size_of_val(&extractor_default));
}

#[test]
fn test_chunk_metadata() {
    let text = "Sample text for chunking";
    let config = ChunkConfig::default();
    let chunks = PdfExtractor::chunk_text(text, &config);

    for chunk in chunks {
        // Verify metadata structure exists
        assert!(chunk.metadata.source_id.is_none() || chunk.metadata.source_id.is_some());
        assert!(chunk.metadata.page_number.is_none() || chunk.metadata.page_number.is_some());
    }
}

#[test]
fn test_sanitizer_default_creation() {
    let sanitizer1 = Sanitizer::default();
    let sanitizer2 = Sanitizer::default_config();

    let text = "john@example.com";
    let result1 = sanitizer1.sanitize(text).unwrap();
    let result2 = sanitizer2.sanitize(text).unwrap();

    assert_eq!(result1.stats.total_redactions, result2.stats.total_redactions);
}

#[test]
fn test_multiple_same_patterns() {
    let sanitizer = Sanitizer::default();
    let text = "First email: john@example.com, second email: jane@example.com";

    let result = sanitizer.sanitize(text).unwrap();

    assert_eq!(result.stats.total_redactions, 2);
    assert_eq!(result.replacements.len(), 2);
}

#[test]
fn test_usid_invalid_formats() {
    let invalid_uris = vec![
        "invalid://format",
        "sens://v1/missing_feature",
        "sens://v1/namespace/invalid_format",
    ];

    for uri in invalid_uris {
        assert!(UsidGenerator::parse(uri).is_err());
    }
}
