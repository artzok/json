use criterion::criterion_main;

use criterion::{criterion_group, Criterion};
use json::{JsonValue, ToJson};

fn parse(str: &str) -> JsonValue {
    json::parse(str).unwrap()
}

fn pretty(json: &JsonValue) {
    let _ = json.pretty();
}

fn benchmark_parse(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    c.bench_function("parse", |b| b.iter(|| parse(&json)));
}

fn benchmark_pretty(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = parse(&json);
    c.bench_function("pretty", |b| b.iter(|| pretty(&json)));
}

criterion_group!(benches, benchmark_parse, benchmark_pretty);
criterion_main!(benches);
