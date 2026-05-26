use criterion::{Criterion, criterion_group, criterion_main};
use ferrimon_burner::fib::matrix::fib_power;
use ferrimon_burner::fib::matrix_simd_dot::MatrixSimdDot2;
use ferrimon_burner::fib::matrix_simd_dot_large::MatrixSimd;
use ferrimon_burner::fib::matrix_simple::MatrixSimple;
use std::hint::black_box;

criterion_main!(bench_fib2, bench_fib8, bench_fib64);

criterion_group!(bench_fib2, simple_fib2, simd_dot_fib2, simd_dot_f2_fib2);
fn simple_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;

    c.bench_function("simple fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimple<_>>(black_box(s)))
    });
}

fn simd_dot_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;
    c.bench_function("simd_dot fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_dot_f2_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;
    c.bench_function("simd_dot_f2_fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimdDot2>(black_box(s)))
    });
}

criterion_group!(bench_fib8, simple_fib8, simd_dot_fib8);

fn simple_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;

    c.bench_function("simple fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimple<_>>(black_box(s)))
    });
}

fn simd_dot_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;
    c.bench_function("simd_dot fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimd<_>>(black_box(s)))
    });
}

criterion_group!(bench_fib64, simple_fib64, simd_dot_fib64);

fn simple_fib64(c: &mut Criterion) {
    let s = 10_000_usize;

    c.bench_function("simple fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimple<_>>(black_box(s)))
    });
}

fn simd_dot_fib64(c: &mut Criterion) {
    let s = 10_000_usize;
    c.bench_function("simd_dot fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimd<_>>(black_box(s)))
    });
}
