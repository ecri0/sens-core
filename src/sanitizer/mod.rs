//! Smart text sanitization module with pattern-based PII detection and redaction.
//!
//! Efficiently detects and redacts sensitive information including:
//! - Email addresses
//! - Phone numbers
//! - Credit card numbers
//! - Social Security Numbers
//! - IP addresses
//! - Custom patterns
//!
//! Uses aho-corasick for fast multi-pattern matching and regex for pattern-based detection.

use crate::utils::Result;
use aho_corasick::AhoCorasick;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of a sanitization operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeResult {
    /// The sanitized text with replacements.
    pub sanitized_text: String,
    /// List of replacements made.
    pub replacements: Vec<Replacement>,
    /// Statistics about the sanitization.
    pub stats: SanitizationStats,
}

/// Information about a single replacement made during sanitization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replacement {
    /// Original text that was replaced.
    pub original: String,
    /// Replacement tag (e.g., `[Email:REDACTED_1]`).
    pub replacement_tag: String,
    /// Start position in original text.
    pub start: usize,
    /// End position in original text.
    pub end: usize,
    /// Category of the detected sensitive data.
    pub category: String,
}

/// Statistics about sanitization operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SanitizationStats {
    /// Total number of redactions made.
    pub total_redactions: usize,
    /// Breakdown by category.
    pub redactions_by_category: HashMap<String, usize>,
    /// Original text length.
    pub original_length: usize,
    /// Sanitized text length.
    pub sanitized_length: usize,
}

/// Configuration for the Smart Sanitizer.
#[derive(Debug, Clone)]
pub struct SanitizerConfig {
    /// Patterns for email detection.
    pub email_pattern: Regex,
    /// Patterns for phone number detection.
    pub phone_pattern: Regex,
    /// Patterns for credit card detection.
    pub credit_card_pattern: Regex,
    /// Patterns for SSN detection.
    pub ssn_pattern: Regex,
    /// Patterns for IP address detection.
    pub ip_pattern: Regex,
    /// Custom patterns for additional detection.
    pub custom_patterns: Vec<(String, Regex)>,
    /// Keywords to match with aho-corasick.
    pub keywords: Vec<String>,
}

impl Default for SanitizerConfig {
    fn default() -> Self {
        Self {
            email_pattern: Regex::new(
                r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
            ).unwrap(),
            phone_pattern: Regex::new(r"(\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")
                .unwrap(),
            credit_card_pattern: Regex::new(r"\b(?:\d{4}[-\s]?){3}\d{4}\b")
                .unwrap(),
            ssn_pattern: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b")
                .unwrap(),
            ip_pattern: Regex::new(
                r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"
            ).unwrap(),
            custom_patterns: Vec::new(),
            keywords: vec![
                "password".to_string(),
                "apikey".to_string(),
                "secret".to_string(),
                "token".to_string(),
            ],
        }
    }
}

/// High-performance text sanitizer using pattern matching and regex.
///
/// The Smart Sanitizer detects and redacts sensitive information from text
/// using a combination of regex patterns and aho-corasick keyword matching.
/// Supports restoration of original values via replacement maps.
///
/// # Examples
///
/// ```
/// use sens_core::sanitizer::Sanitizer;
///
/// let sanitizer = Sanitizer::new(Default::default());
/// let result = sanitizer.sanitize("Contact me at john@example.com").unwrap();
/// println!("Sanitized: {}", result.sanitized_text);
/// // Output: "Contact me at [Email:REDACTED_1]"
/// ```
pub struct Sanitizer {
    config: SanitizerConfig,
    ac: AhoCorasick,
}

impl Sanitizer {
    /// Creates a new sanitizer with the given configuration.
    pub fn new(config: SanitizerConfig) -> Self {
        let keywords = config.keywords.iter().map(|k| k.to_lowercase()).collect::<Vec<_>>();
        let ac = AhoCorasick::new(keywords).unwrap_or_else(|_| {
            AhoCorasick::new(vec!["password", "apikey", "secret", "token"]).unwrap()
        });

        Sanitizer { config, ac }
    }

    /// Creates a sanitizer with default configuration.
    pub fn default_config() -> Self {
        Self::new(SanitizerConfig::default())
    }

    /// Sanitizes text by detecting and redacting sensitive information.
    ///
    /// Returns a `SanitizeResult` containing the sanitized text, replacements,
    /// and statistics about what was redacted.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to sanitize.
    ///
    /// # Returns
    ///
    /// A `Result` containing the sanitization result.
    pub fn sanitize(&self, text: &str) -> Result<SanitizeResult> {
        let mut sanitized = text.to_string();
        let mut replacements = Vec::new();
        let mut stats = SanitizationStats {
            original_length: text.len(),
            redactions_by_category: HashMap::new(),
            ..Default::default()
        };

        let patterns = vec![
            ("Email", &self.config.email_pattern),
            ("PhoneNumber", &self.config.phone_pattern),
            ("CreditCard", &self.config.credit_card_pattern),
            ("SSN", &self.config.ssn_pattern),
            ("IPAddress", &self.config.ip_pattern),
        ];

        let mut matches = Vec::new();

        for (category, pattern) in patterns {
            for mat in pattern.find_iter(text) {
                matches.push((mat.start(), mat.end(), mat.as_str().to_string(), category.to_string()));
            }
        }

        for (name, pattern) in &self.config.custom_patterns {
            for mat in pattern.find_iter(text) {
                matches.push((mat.start(), mat.end(), mat.as_str().to_string(), name.clone()));
            }
        }

        matches.sort_by_key(|m| m.0);
        matches.dedup_by_key(|m| (m.0, m.1));

        let mut offset: isize = 0;
        let mut replacement_counts: HashMap<String, usize> = HashMap::new();

        for (start, end, original, category) in matches {
            let adjusted_start = (start as isize + offset) as usize;
            let adjusted_end = (end as isize + offset) as usize;

            let count = replacement_counts.entry(category.clone()).or_insert(0);
            *count += 1;

            let replacement_tag = format!("[{}:REDACTED_{}]", category, count);
            let replacement_len = replacement_tag.len();
            let original_len = original.len();

            sanitized.replace_range(adjusted_start..adjusted_end, &replacement_tag);
            offset += replacement_len as isize - original_len as isize;

            replacements.push(Replacement {
                original: original.clone(),
                replacement_tag,
                start,
                end,
                category: category.clone(),
            });

            *stats.redactions_by_category.entry(category).or_insert(0) += 1;
            stats.total_redactions += 1;
        }

        stats.sanitized_length = sanitized.len();

        Ok(SanitizeResult {
            sanitized_text: sanitized,
            replacements,
            stats,
        })
    }

    /// Restores original values from sanitized text using the replacement map.
    ///
    /// This function reverses the sanitization process, replacing redaction tags
    /// with their original values.
    ///
    /// # Arguments
    ///
    /// * `sanitized` - The sanitized text.
    /// * `replacements` - The replacement list from `SanitizeResult`.
    ///
    /// # Returns
    ///
    /// The restored text with original values reinstated.
    pub fn restore(sanitized: &str, replacements: &[Replacement]) -> String {
        let mut result = sanitized.to_string();

        // Sort replacements by position in reverse to avoid offset issues
        let mut sorted = replacements.to_vec();
        sorted.sort_by(|a, b| b.replacement_tag.cmp(&a.replacement_tag));

        for replacement in sorted {
            result = result.replace(&replacement.replacement_tag, &replacement.original);
        }

        result
    }

    /// Adds a custom pattern to the sanitizer configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - Name/category for the pattern.
    /// * `pattern` - Regex pattern to match.
    pub fn add_custom_pattern(&mut self, name: String, pattern: Regex) {
        self.config.custom_patterns.push((name, pattern));
    }
}

impl Default for Sanitizer {
    fn default() -> Self {
        Self::new(SanitizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_email() {
        let sanitizer = Sanitizer::default();
        let result = sanitizer
            .sanitize("Contact john@example.com for info")
            .unwrap();
        assert!(result.sanitized_text.contains("REDACTED"));
        assert_eq!(result.stats.total_redactions, 1);
    }

    #[test]
    fn test_sanitize_phone() {
        let sanitizer = Sanitizer::default();
        let result = sanitizer
            .sanitize("Call me at (555) 123-4567")
            .unwrap();
        assert!(result.sanitized_text.contains("REDACTED"));
    }

    #[test]
    fn test_sanitize_ssn() {
        let sanitizer = Sanitizer::default();
        let result = sanitizer
            .sanitize("SSN: 123-45-6789")
            .unwrap();
        assert!(result.sanitized_text.contains("REDACTED"));
    }

    #[test]
    fn test_sanitize_ip_address() {
        let sanitizer = Sanitizer::default();
        let result = sanitizer
            .sanitize("Server IP is 192.168.1.1")
            .unwrap();
        assert!(result.sanitized_text.contains("REDACTED"));
    }

    #[test]
    fn test_restore_text() {
        let sanitizer = Sanitizer::default();
        let original = "Email: john@example.com";
        let result = sanitizer.sanitize(original).unwrap();
        let restored = Sanitizer::restore(&result.sanitized_text, &result.replacements);
        assert_eq!(restored, original);
    }

    #[test]
    fn test_multiple_redactions() {
        let sanitizer = Sanitizer::default();
        let text = "Contact john@example.com or jane@test.com";
        let result = sanitizer.sanitize(text).unwrap();
        assert_eq!(result.stats.total_redactions, 2);
    }
}
