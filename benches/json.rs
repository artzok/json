use criterion::criterion_main;

use criterion::{criterion_group, Criterion};
use json::{ToJson, BuildConfig};

fn benchmark_parse(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    c.bench_function("parse", |b| b.iter(|| json::parse(&json)));
}

fn benchmark_pretty(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = json::parse(&json).unwrap();
    c.bench_function("pretty", |b| b.iter(|| json.pretty()));
}

fn benchmark_pretty_with_check_nest(c: &mut Criterion) {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = json::parse(&json).unwrap();
    c.bench_function("pretty_with_check_nest", |b| b.iter(|| json.to_json(&BuildConfig::new(true, "| ", true))));
}

criterion_group!(benches, benchmark_parse, benchmark_pretty, benchmark_pretty_with_check_nest);
criterion_main!(benches);
