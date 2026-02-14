//! Benchmarks for PDF extraction and text chunking operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sens_core::pdf::PdfExtractor;
use sens_core::utils::ChunkConfig;

fn benchmark_chunk_text_small(c: &mut Criterion) {
    let text = "This is a small sample text. ".repeat(50);

    c.bench_function("chunk_text_small", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 256,
                overlap_tokens: 32,
                separator: ". ".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });
}

fn benchmark_chunk_text_medium(c: &mut Criterion) {
    let text = "This is a paragraph of sample text. ".repeat(500);

    c.bench_function("chunk_text_medium", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 512,
                overlap_tokens: 64,
                separator: ". ".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });
}

fn benchmark_chunk_text_large(c: &mut Criterion) {
    let text = "This is a paragraph of sample text. ".repeat(5000);

    c.bench_function("chunk_text_large", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 1024,
                overlap_tokens: 128,
                separator: "\n".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });
}

fn benchmark_chunk_text_different_separators(c: &mut Criterion) {
    let text = "Sentence one. Sentence two. Sentence three. ".repeat(100);

    let mut group = c.benchmark_group("separators");

    group.bench_function("separator_dot_space", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 256,
                overlap_tokens: 32,
                separator: ". ".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });

    group.bench_function("separator_newline", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 256,
                overlap_tokens: 32,
                separator: "\n".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });

    group.bench_function("separator_double_newline", |b| {
        b.iter(|| {
            let config = ChunkConfig {
                max_tokens: 256,
                overlap_tokens: 32,
                separator: "\n\n".to_string(),
            };
            PdfExtractor::chunk_text(black_box(&text), black_box(&config))
        })
    });

    group.finish();
}

fn benchmark_chunk_text_with_different_overlap(c: &mut Criterion) {
    let text = "Sample text content. ".repeat(200);

    let mut group = c.benchmark_group("overlap_sizes");

    for overlap in &[0, 16, 32, 64, 128] {
        group.bench_function(format!("overlap_{}", overlap), |b| {
            b.iter(|| {
                let config = ChunkConfig {
                    max_tokens: 256,
                    overlap_tokens: *overlap,
                    separator: ". ".to_string(),
                };
                PdfExtractor::chunk_text(black_box(&text), black_box(&config))
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_chunk_text_small,
    benchmark_chunk_text_medium,
    benchmark_chunk_text_large,
    benchmark_chunk_text_different_separators,
    benchmark_chunk_text_with_different_overlap,
);

criterion_main!(benches);
