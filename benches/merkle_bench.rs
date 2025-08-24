use criterion::{criterion_group, criterion_main, Criterion};
use toy_node::blockchain::{Transaction, calculate_merkle_root, build_merkle_tree};

fn merkle_benchmark(c: &mut Criterion) {
    // Generate dummy transactions
    let transactions: Vec<Transaction> = (0..1000)
        .map(|i| Transaction::new(
            "Alice".into(),
            format!("Bob{}", i),
            i,
            "dummy_signature".into(),
        ))
        .collect();

    // Benchmark only root calculation (old method)
    c.bench_function("merkle_root_1000", |b| {
        b.iter(|| calculate_merkle_root(&transactions))
    });

    // Benchmark full Merkle tree build (new method)
    c.bench_function("merkle_tree_1000", |b| {
        b.iter(|| build_merkle_tree(&transactions))
    });
}

// 👇 Register benchmarks
criterion_group!(benches, merkle_benchmark);
criterion_main!(benches);
