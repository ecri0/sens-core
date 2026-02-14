//! Universal Semantic ID (USID) generation and parsing.
//!
//! USIDs provide model-independent semantic identifiers for content using the format:
//! `sens://v1/{namespace}/f{feature_id}_{content_hash_prefix}`
//!
//! Uses SHA-256 hashing to create deterministic, collision-resistant identifiers
//! based on content and feature configuration.

use crate::utils::{Result, SensCoreError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A Universal Semantic ID in the Sens ecosystem.
///
/// Format: `sens://v1/{namespace}/f{feature_id}_{content_hash_prefix}`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Usid {
    /// The namespace for this USID (e.g., "universal", "document").
    pub namespace: String,
    /// The feature ID associated with this content.
    pub feature_id: u32,
    /// The content hash prefix (first 16 hex characters of SHA-256 hash).
    pub content_hash: String,
}

/// Information about a resolved USID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsidInfo {
    /// The full USID URI.
    pub uri: String,
    /// Namespace component.
    pub namespace: String,
    /// Feature ID component.
    pub feature_id: u32,
    /// Content hash component.
    pub content_hash: String,
    /// The feature_id as a prefixed string (e.g., "f1").
    pub feature_prefix: String,
}

/// Generator for creating and parsing Universal Semantic IDs.
///
/// # Examples
///
/// ```
/// use sens_core::usid::UsidGenerator;
///
/// let content = "Hello, world!";
/// let usid = UsidGenerator::generate(content, 1);
/// println!("USID: {}", usid);
///
/// let parsed = UsidGenerator::parse(usid.to_string().as_str()).unwrap();
/// println!("Parsed namespace: {}", parsed.namespace);
/// ```
pub struct UsidGenerator;

impl UsidGenerator {
    /// Generates a USID from content and feature ID.
    ///
    /// Creates a deterministic USID by hashing the content with SHA-256
    /// and combining it with the provided feature ID.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to generate a USID for.
    /// * `feature_id` - The feature ID to associate with this content.
    ///
    /// # Returns
    ///
    /// A `Usid` object representing the generated identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use sens_core::usid::UsidGenerator;
    ///
    /// let usid = UsidGenerator::generate("sample content", 42);
    /// assert_eq!(usid.feature_id, 42);
    /// assert_eq!(usid.namespace, "universal");
    /// ```
    pub fn generate(content: &str, feature_id: u32) -> Usid {
        let hash = Self::compute_hash(content);
        let hash_prefix = hash.chars().take(16).collect::<String>();

        Usid {
            namespace: "universal".to_string(),
            feature_id,
            content_hash: hash_prefix,
        }
    }

    /// Generates a USID with a custom namespace.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to generate a USID for.
    /// * `feature_id` - The feature ID to associate with this content.
    /// * `namespace` - Custom namespace for the USID.
    ///
    /// # Returns
    ///
    /// A `Usid` object with the specified namespace.
    pub fn generate_with_namespace(content: &str, feature_id: u32, namespace: String) -> Usid {
        let hash = Self::compute_hash(content);
        let hash_prefix = hash.chars().take(16).collect::<String>();

        Usid {
            namespace,
            feature_id,
            content_hash: hash_prefix,
        }
    }

    /// Parses a USID string into a `Usid` object.
    ///
    /// Expected format: `sens://v1/{namespace}/f{feature_id}_{content_hash}`
    ///
    /// # Arguments
    ///
    /// * `uri` - The USID URI string to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Usid`, or an error if parsing fails.
    ///
    /// # Example
    ///
    /// ```
    /// use sens_core::usid::UsidGenerator;
    ///
    /// let uri = "sens://v1/universal/f42_abcdef0123456789";
    /// let usid = UsidGenerator::parse(uri).unwrap();
    /// assert_eq!(usid.feature_id, 42);
    /// ```
    pub fn parse(uri: &str) -> Result<Usid> {
        if !uri.starts_with("sens://v1/") {
            return Err(SensCoreError::UsidError(
                "USID must start with 'sens://v1/'".to_string(),
            ));
        }

        let remainder = &uri[10..];
        let parts: Vec<&str> = remainder.split('/').collect();

        if parts.len() != 2 {
            return Err(SensCoreError::UsidError(
                "Invalid USID format: expected namespace/feature".to_string(),
            ));
        }

        let namespace = parts[0].to_string();
        let feature_part = parts[1];

        if !feature_part.starts_with('f') {
            return Err(SensCoreError::UsidError(
                "Feature part must start with 'f'".to_string(),
            ));
        }

        let feature_content = &feature_part[1..];
        let underscore_pos = feature_content.find('_').ok_or_else(|| {
            SensCoreError::UsidError("Invalid USID format: missing '_' separator".to_string())
        })?;

        let feature_id_str = &feature_content[..underscore_pos];
        let content_hash = &feature_content[underscore_pos + 1..];

        let feature_id: u32 = feature_id_str.parse().map_err(|_| {
            SensCoreError::UsidError("Invalid feature ID: not a valid integer".to_string())
        })?;

        if content_hash.is_empty() {
            return Err(SensCoreError::UsidError(
                "Content hash cannot be empty".to_string(),
            ));
        }

        Ok(Usid {
            namespace,
            feature_id,
            content_hash: content_hash.to_string(),
        })
    }

    /// Resolves a USID into detailed information.
    ///
    /// # Arguments
    ///
    /// * `usid` - The USID to resolve.
    ///
    /// # Returns
    ///
    /// A `UsidInfo` object with all components and the full URI.
    pub fn resolve(usid: &Usid) -> UsidInfo {
        let uri = usid.to_string();
        let feature_prefix = format!("f{}", usid.feature_id);

        UsidInfo {
            uri,
            namespace: usid.namespace.clone(),
            feature_id: usid.feature_id,
            content_hash: usid.content_hash.clone(),
            feature_prefix,
        }
    }

    /// Computes the SHA-256 hash of content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to hash.
    ///
    /// # Returns
    ///
    /// The hexadecimal representation of the SHA-256 hash.
    fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash_bytes = hasher.finalize();
        format!("{:x}", hash_bytes)
    }

    /// Computes a deterministic short ID from content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to hash.
    ///
    /// # Returns
    ///
    /// A 16-character hexadecimal string.
    pub fn compute_short_id(content: &str) -> String {
        let hash = Self::compute_hash(content);
        hash.chars().take(16).collect()
    }
}

impl std::fmt::Display for Usid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sens://v1/{}/f{}_{}",
            self.namespace, self.feature_id, self.content_hash
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_usid() {
        let usid = UsidGenerator::generate("test content", 1);
        assert_eq!(usid.namespace, "universal");
        assert_eq!(usid.feature_id, 1);
        assert_eq!(usid.content_hash.len(), 16);
    }

    #[test]
    fn test_usid_format() {
        let usid = UsidGenerator::generate("content", 42);
        let usid_str = usid.to_string();
        assert!(usid_str.starts_with("sens://v1/universal/f42_"));
    }

    #[test]
    fn test_parse_usid() {
        let uri = "sens://v1/universal/f42_abcdef0123456789";
        let usid = UsidGenerator::parse(uri).unwrap();
        assert_eq!(usid.feature_id, 42);
        assert_eq!(usid.namespace, "universal");
        assert_eq!(usid.content_hash, "abcdef0123456789");
    }

    #[test]
    fn test_parse_invalid_usid() {
        let invalid = "invalid://format";
        assert!(UsidGenerator::parse(invalid).is_err());
    }

    #[test]
    fn test_deterministic_generation() {
        let content = "same content";
        let usid1 = UsidGenerator::generate(content, 1);
        let usid2 = UsidGenerator::generate(content, 1);
        assert_eq!(usid1, usid2);
    }

    #[test]
    fn test_resolve_usid() {
        let usid = UsidGenerator::generate("content", 42);
        let info = UsidGenerator::resolve(&usid);
        assert_eq!(info.feature_id, 42);
        assert_eq!(info.namespace, "universal");
    }

    #[test]
    fn test_custom_namespace() {
        let usid = UsidGenerator::generate_with_namespace("content", 10, "document".to_string());
        assert_eq!(usid.namespace, "document");
        let uri = usid.to_string();
        assert!(uri.contains("document"));
    }

    #[test]
    fn test_short_id() {
        let short_id = UsidGenerator::compute_short_id("test");
        assert_eq!(short_id.len(), 16);
    }

    #[test]
    fn test_roundtrip_parse() {
        let original = UsidGenerator::generate("roundtrip test", 99);
        let uri = original.to_string();
        let parsed = UsidGenerator::parse(&uri).unwrap();
        assert_eq!(original, parsed);
    }
}
