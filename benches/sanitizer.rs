//! Benchmarks for text sanitization operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sens_core::sanitizer::Sanitizer;

fn benchmark_sanitize_clean_text(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let text = "This is clean text with no sensitive information. ".repeat(100);

    c.bench_function("sanitize_clean_text", |b| {
        b.iter(|| sanitizer.sanitize(black_box(&text)))
    });
}

fn benchmark_sanitize_with_emails(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let text = "Contact john@example.com or jane@test.org for details. ".repeat(50);

    c.bench_function("sanitize_with_emails", |b| {
        b.iter(|| sanitizer.sanitize(black_box(&text)))
    });
}

fn benchmark_sanitize_with_phone_numbers(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let text = "Call (555) 123-4567 or +1-555-987-6543 for support. ".repeat(50);

    c.bench_function("sanitize_with_phone_numbers", |b| {
        b.iter(|| sanitizer.sanitize(black_box(&text)))
    });
}

fn benchmark_sanitize_with_mixed_pii(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let text = "Contact john@example.com, phone (555) 123-4567, SSN 123-45-6789. ".repeat(50);

    c.bench_function("sanitize_with_mixed_pii", |b| {
        b.iter(|| sanitizer.sanitize(black_box(&text)))
    });
}

fn benchmark_sanitize_different_text_sizes(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let mut group = c.benchmark_group("text_sizes");

    let sizes = vec![
        ("small_1kb", "Sample text with john@example.com ".repeat(29)),
        ("medium_10kb", "Sample text with john@example.com ".repeat(290)),
        ("large_100kb", "Sample text with john@example.com ".repeat(2900)),
    ];

    for (name, text) in sizes {
        group.bench_function(name, |b| {
            b.iter(|| sanitizer.sanitize(black_box(&text)))
        });
    }

    group.finish();
}

fn benchmark_restore_text(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let text = "Contact john@example.com and jane@test.org ".repeat(100);
    let result = sanitizer.sanitize(&text).unwrap();

    c.bench_function("restore_text", |b| {
        b.iter(|| {
            Sanitizer::restore(
                black_box(&result.sanitized_text),
                black_box(&result.replacements),
            )
        })
    });
}

fn benchmark_sanitize_specific_patterns(c: &mut Criterion) {
    let sanitizer = Sanitizer::default();
    let mut group = c.benchmark_group("patterns");

    let patterns = vec![
        ("email_heavy", "john@example.com jane@test.org ".repeat(200)),
        ("phone_heavy", "(555) 123-4567 (555) 987-6543 ".repeat(200)),
        ("ssn_heavy", "123-45-6789 987-65-4321 ".repeat(200)),
        ("ip_heavy", "192.168.1.1 10.0.0.1 172.16.0.1 ".repeat(200)),
    ];

    for (name, text) in patterns {
        group.bench_function(name, |b| {
            b.iter(|| sanitizer.sanitize(black_box(&text)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_sanitize_clean_text,
    benchmark_sanitize_with_emails,
    benchmark_sanitize_with_phone_numbers,
    benchmark_sanitize_with_mixed_pii,
    benchmark_sanitize_different_text_sizes,
    benchmark_restore_text,
    benchmark_sanitize_specific_patterns,
);

criterion_main!(benches);
