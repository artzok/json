use criterion::criterion_main;

use criterion::{criterion_group, Criterion};
use json::ToJson;

fn benchmark_parse(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    c.bench_function("parse", |b| b.iter(|| json::parse(&json)));
}

fn benchmark_pretty(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = json::parse(&json).unwrap();
    c.bench_function("pretty", |b| b.iter(|| json.pretty()));
}

criterion_group!(benches, benchmark_parse, benchmark_pretty);
criterion_main!(benches);
