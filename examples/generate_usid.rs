//! Example: USID generation, parsing, and resolution.
//!
//! This example demonstrates how to:
//! 1. Generate Universal Semantic IDs (USIDs)
//! 2. Parse USID strings
//! 3. Resolve USID components
//! 4. Use custom namespaces
//!
//! Run with: `cargo run --example generate_usid`

use sens_core::usid::UsidGenerator;

fn main() -> sens_core::Result<()> {
    println!("=== USID Generation Example ===\n");
    println!("Universal Semantic IDs provide model-independent content identification");
    println!("Format: sens://v1/{{namespace}}/f{{feature_id}}_{{content_hash}}\n");

    // Example 1: Basic USID generation
    println!("=== Example 1: Basic USID Generation ===");
    let content1 = "This is sample content for USID generation";
    let feature_id1 = 1;

    let usid1 = UsidGenerator::generate(content1, feature_id1);
    println!("Content: \"{}\"", content1);
    println!("Feature ID: {}", feature_id1);
    println!("Generated USID: {}\n", usid1);

    // Display components
    println!("USID Components:");
    println!("  Namespace: {}", usid1.namespace);
    println!("  Feature ID: {}", usid1.feature_id);
    println!("  Content Hash: {}\n", usid1.content_hash);

    // Example 2: Deterministic generation
    println!("=== Example 2: Deterministic Generation ===");
    let content2 = "Deterministic content";
    let usid2a = UsidGenerator::generate(content2, 42);
    let usid2b = UsidGenerator::generate(content2, 42);

    println!("Content: \"{}\"", content2);
    println!("First generation:  {}", usid2a);
    println!("Second generation: {}", usid2b);
    println!("Match: {}\n", usid2a == usid2b);

    // Different feature IDs produce different USIDs
    let usid2c = UsidGenerator::generate(content2, 43);
    println!("With feature_id=43: {}", usid2c);
    println!("Different: {}\n", usid2a != usid2c);

    // Example 3: Parsing USID strings
    println!("=== Example 3: Parsing USID Strings ===");
    let uri = "sens://v1/universal/f42_abcdef0123456789abcdef0123456789";
    println!("USID string: {}", uri);

    match UsidGenerator::parse(uri) {
        Ok(parsed) => {
            println!("Parsed successfully:");
            println!("  Namespace: {}", parsed.namespace);
            println!("  Feature ID: {}", parsed.feature_id);
            println!("  Content Hash: {}\n", parsed.content_hash);
        }
        Err(e) => println!("Parse error: {}\n", e),
    }

    // Example 4: Invalid USID parsing
    println!("=== Example 4: Invalid USID Handling ===");
    let invalid_uris = vec![
        "invalid://format",
        "sens://v1/universal/invalid",
        "sens://v1/universal/f_nohash",
    ];

    for invalid in invalid_uris {
        println!("Attempting to parse: {}", invalid);
        match UsidGenerator::parse(invalid) {
            Ok(_) => println!("  ✓ Parsed"),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }
    println!();

    // Example 5: Resolving USID information
    println!("=== Example 5: USID Resolution ===");
    let content5 = "Content to be resolved";
    let usid5 = UsidGenerator::generate(content5, 99);
    let info = UsidGenerator::resolve(&usid5);

    println!("Generated USID: {}", usid5);
    println!("\nResolved information:");
    println!("  URI: {}", info.uri);
    println!("  Namespace: {}", info.namespace);
    println!("  Feature ID: {}", info.feature_id);
    println!("  Feature Prefix: {}", info.feature_prefix);
    println!("  Content Hash: {}\n", info.content_hash);

    // Example 6: Custom namespace
    println!("=== Example 6: Custom Namespaces ===");
    let content6 = "Document content";
    let namespaces = vec!["universal", "document", "email", "image"];

    for ns in namespaces {
        let usid = UsidGenerator::generate_with_namespace(
            content6,
            10,
            ns.to_string()
        );
        println!("Namespace '{}': {}", ns, usid);
    }
    println!();

    // Example 7: Feature ID variations
    println!("=== Example 7: Feature ID Variations ===");
    let content7 = "Fixed content";
    let feature_ids = vec![1, 10, 100, 1000, 65535];

    println!("Content: \"{}\"", content7);
    for fid in feature_ids {
        let usid = UsidGenerator::generate(content7, fid as u32);
        println!("  Feature ID {:5}: {}", fid, usid);
    }
    println!();

    // Example 8: Content sensitivity
    println!("=== Example 8: Content Hash Sensitivity ===");
    let base = "similar content";
    let variations = vec![
        base,
        "similar content ",      // trailing space
        "similar  content",      // double space
        "similar content.",      // period
        "Similar content",       // different case
    ];

    println!("Demonstrating hash sensitivity:");
    for var in variations {
        let usid = UsidGenerator::generate(var, 5);
        println!("  '{}' -> {}", var, usid.content_hash);
    }
    println!();

    // Example 9: Roundtrip conversion
    println!("=== Example 9: Roundtrip Conversion ===");
    let original_content = "Roundtrip test content";
    let original_feature = 777;

    // Generate USID
    let generated = UsidGenerator::generate(original_content, original_feature);
    println!("Generated: {}", generated);

    // Convert to string
    let uri_string = generated.to_string();
    println!("As string: {}", uri_string);

    // Parse back
    let parsed = UsidGenerator::parse(&uri_string)?;
    println!("Parsed back: {}", parsed);

    // Verify roundtrip
    println!("Roundtrip successful: {}", generated == parsed);
    println!();

    // Example 10: Batch processing
    println!("=== Example 10: Batch Processing ===");
    let documents = vec![
        "Document 1 content",
        "Document 2 content",
        "Document 3 content",
    ];

    println!("Processing {} documents:", documents.len());
    for (idx, doc) in documents.iter().enumerate() {
        let feature_id = (idx + 1) as u32;
        let usid = UsidGenerator::generate(doc, feature_id);
        println!("  Doc {}: {}", idx + 1, usid);
    }
    println!();

    // Example 11: Short ID computation
    println!("=== Example 11: Short ID Computation ===");
    let content11 = "Short ID generation";
    let short_id = UsidGenerator::compute_short_id(content11);
    println!("Content: \"{}\"", content11);
    println!("Short ID (16 chars): {}", short_id);
    println!("Length: {}\n", short_id.len());

    println!("=== Usage Summary ===");
    println!("Basic usage:");
    println!("  1. Generate: let usid = UsidGenerator::generate(content, feature_id);");
    println!("  2. Display: println!(\"USID: {{}}\", usid);");
    println!("  3. Parse: let parsed = UsidGenerator::parse(uri_string)?;");
    println!("  4. Resolve: let info = UsidGenerator::resolve(&usid);");
    println!();
    println!("Features:");
    println!("  - Deterministic (same content + feature = same USID)");
    println!("  - Content-addressable (hash-based)");
    println!("  - Custom namespaces supported");
    println!("  - SHA-256 based hashing");
    println!("  - Full roundtrip parse/generate support");

    Ok(())
}
