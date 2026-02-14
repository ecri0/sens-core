//! Example: Text sanitization and PII redaction.
//!
//! This example demonstrates how to:
//! 1. Create a sanitizer with default patterns
//! 2. Detect and redact various types of PII
//! 3. View sanitization statistics
//! 4. Restore original values
//!
//! Run with: `cargo run --example sanitize_text`

use sens_core::sanitizer::Sanitizer;

fn main() -> sens_core::Result<()> {
    println!("=== Text Sanitization Example ===\n");

    // Create a sanitizer with default configuration
    let sanitizer = Sanitizer::default();
    println!("Initialized sanitizer with default patterns:");
    println!("  - Email addresses");
    println!("  - Phone numbers");
    println!("  - Credit card numbers");
    println!("  - Social Security Numbers (SSN)");
    println!("  - IP addresses\n");

    // Example 1: Sanitize text with multiple PII types
    println!("=== Example 1: Multi-Type PII Detection ===");
    let text1 = "Contact information: email john.doe@example.com, \
                 phone (555) 123-4567, SSN 123-45-6789";
    println!("Original: {}", text1);

    let result1 = sanitizer.sanitize(text1)?;
    println!("Sanitized: {}", result1.sanitized_text);
    println!("Redactions: {}", result1.stats.total_redactions);

    println!("\nDetailed replacements:");
    for (idx, replacement) in result1.replacements.iter().enumerate() {
        println!("  {}. {} -> {} ({})",
            idx + 1,
            replacement.original,
            replacement.replacement_tag,
            replacement.category
        );
    }

    // Example 2: Email sanitization
    println!("\n=== Example 2: Email Detection ===");
    let text2 = "The team includes john@example.com, jane_smith@company.org, \
                 and support@service.net";
    println!("Original: {}", text2);

    let result2 = sanitizer.sanitize(text2)?;
    println!("Sanitized: {}", result2.sanitized_text);
    println!("Emails found: {}", result2.stats.redactions_by_category
        .get("Email")
        .unwrap_or(&0));

    // Example 3: Phone number detection
    println!("\n=== Example 3: Phone Number Detection ===");
    let text3 = "Call us at (555) 123-4567 or +1-555-987-6543 for support.";
    println!("Original: {}", text3);

    let result3 = sanitizer.sanitize(text3)?;
    println!("Sanitized: {}", result3.sanitized_text);
    println!("Phone numbers found: {}", result3.stats.redactions_by_category
        .get("PhoneNumber")
        .unwrap_or(&0));

    // Example 4: SSN and credit card
    println!("\n=== Example 4: Financial Information ===");
    let text4 = "Payment details: Card 4532-1111-2222-3333, SSN 987-65-4321";
    println!("Original: {}", text4);

    let result4 = sanitizer.sanitize(text4)?;
    println!("Sanitized: {}", result4.sanitized_text);

    println!("\nRedaction breakdown:");
    for (category, count) in &result4.stats.redactions_by_category {
        println!("  {}: {}", category, count);
    }

    // Example 5: IP address detection
    println!("\n=== Example 5: IP Address Detection ===");
    let text5 = "Server IPs: 192.168.1.1, 10.0.0.1, 172.16.0.1";
    println!("Original: {}", text5);

    let result5 = sanitizer.sanitize(text5)?;
    println!("Sanitized: {}", result5.sanitized_text);

    // Example 6: Restoration of original values
    println!("\n=== Example 6: Text Restoration ===");
    let text6 = "Contact john@example.com or call (555) 123-4567";
    println!("Original: {}", text6);

    let result6 = sanitizer.sanitize(text6)?;
    println!("Sanitized: {}", result6.sanitized_text);

    // Restore original text
    let restored = Sanitizer::restore(&result6.sanitized_text, &result6.replacements);
    println!("Restored: {}", restored);

    // Verify restoration
    if restored == text6 {
        println!("✓ Restoration successful!");
    } else {
        println!("✗ Restoration mismatch");
    }

    // Example 7: Statistics analysis
    println!("\n=== Example 7: Sanitization Statistics ===");
    let text7 = "User: john.doe@example.com (555)123-4567 CC: 4532-1111-2222-3333";
    let result7 = sanitizer.sanitize(text7)?;

    println!("Text analysis:");
    println!("  Original length: {} chars", result7.stats.original_length);
    println!("  Sanitized length: {} chars", result7.stats.sanitized_length);
    println!("  Total redactions: {}", result7.stats.total_redactions);
    println!("\nRedactions by category:");
    for (category, count) in &result7.stats.redactions_by_category {
        println!("  - {}: {}", category, count);
    }

    // Example 8: No PII detected
    println!("\n=== Example 8: Clean Text (No PII) ===");
    let text8 = "This is a clean document with no sensitive information.";
    println!("Original: {}", text8);

    let result8 = sanitizer.sanitize(text8)?;
    println!("Sanitized: {}", result8.sanitized_text);
    println!("Redactions: {}", result8.stats.total_redactions);

    if result8.stats.total_redactions == 0 {
        println!("✓ No PII detected");
    }

    // Example 9: Complex document simulation
    println!("\n=== Example 9: Complex Document ===");
    let complex_doc = "
        Customer Record:
        Name: John Doe
        Email: john.doe@company.com
        Phone: +1 (555) 123-4567
        SSN: 123-45-6789

        Billing Information:
        Card: 4532-1111-2222-3333
        Expires: 12/25

        Server Configuration:
        Primary: 192.168.1.1
        Secondary: 10.0.0.1
        Backup: 172.16.0.1
    ";

    println!("Original document (first 100 chars): {}",
        &complex_doc.chars().take(100).collect::<String>().replace('\n', " "));

    let result9 = sanitizer.sanitize(complex_doc)?;
    println!("\nSanitization complete!");
    println!("Total redactions: {}", result9.stats.total_redactions);
    println!("\nRedaction summary:");
    for (category, count) in &result9.stats.redactions_by_category {
        println!("  {}: {}", category, count);
    }

    println!("\n=== Usage Summary ===");
    println!("1. Create sanitizer: let sanitizer = Sanitizer::default();");
    println!("2. Sanitize text: let result = sanitizer.sanitize(text)?;");
    println!("3. Access result fields:");
    println!("   - result.sanitized_text: String");
    println!("   - result.replacements: Vec<Replacement>");
    println!("   - result.stats: SanitizationStats");
    println!("4. Restore if needed: Sanitizer::restore(sanitized, replacements)");

    Ok(())
}
