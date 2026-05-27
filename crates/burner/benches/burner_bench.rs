use criterion::{Criterion, criterion_group, criterion_main};
use ferrimon_burner::fib::matrix::fib_power;
use ferrimon_burner::fib::matrix_simd_dot::MatrixSimdDot2;
use ferrimon_burner::fib::matrix_simd_dot_large::MatrixSimd;
use ferrimon_burner::fib::matrix_simd_outer::MatrixSimdOuter;
use ferrimon_burner::fib::matrix_simple::MatrixSimple;
use ferrimon_burner::fib::matrix_simple_autovec::MatrixSimpleAutovec;
use std::hint::black_box;

criterion_main!(
    bench_fib2,
    bench_fib4,
    bench_fib8,
    bench_fib16,
    bench_fib32,
    bench_fib64
);

// -------- fib2 --------
criterion_group!(
    bench_fib2,
    simple_fib2,
    simple_autovec_fib2,
    simd_dot_fib2,
    simd_dot_f2_fib2,
    simd_outer_fib2
);

fn simple_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;
    c.bench_function("simple fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;
    c.bench_function("simple_autovec fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimpleAutovec<_>>(black_box(s)))
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
    c.bench_function("simd_dot_f2 fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimdDot2>(black_box(s)))
    });
}

fn simd_outer_fib2(c: &mut Criterion) {
    let s = 10_000_000_usize;
    c.bench_function("simd_outer fib2", |b| {
        b.iter(|| fib_power::<2, MatrixSimdOuter<_>>(black_box(s)))
    });
}

// -------- fib4 --------
criterion_group!(
    bench_fib4,
    simple_fib4,
    simple_autovec_fib4,
    simd_dot_fib4,
    simd_outer_fib4
);

fn simple_fib4(c: &mut Criterion) {
    let s = 5_000_000_usize;
    c.bench_function("simple fib4", |b| {
        b.iter(|| fib_power::<4, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib4(c: &mut Criterion) {
    let s = 5_000_000_usize;
    c.bench_function("simple_autovec fib4", |b| {
        b.iter(|| fib_power::<4, MatrixSimpleAutovec<_>>(black_box(s)))
    });
}

fn simd_dot_fib4(c: &mut Criterion) {
    let s = 5_000_000_usize;
    c.bench_function("simd_dot fib4", |b| {
        b.iter(|| fib_power::<4, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_outer_fib4(c: &mut Criterion) {
    let s = 5_000_000_usize;
    c.bench_function("simd_outer fib4", |b| {
        b.iter(|| fib_power::<4, MatrixSimdOuter<_>>(black_box(s)))
    });
}

// -------- fib8 --------
criterion_group!(
    bench_fib8,
    simple_fib8,
    simple_autovec_fib8,
    simd_dot_fib8,
    simd_outer_fib8
);

fn simple_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;
    c.bench_function("simple fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;
    c.bench_function("simple_autovec fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimpleAutovec<_>>(black_box(s)))
    });
}

fn simd_dot_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;
    c.bench_function("simd_dot fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_outer_fib8(c: &mut Criterion) {
    let s = 1_000_000_usize;
    c.bench_function("simd_outer fib8", |b| {
        b.iter(|| fib_power::<8, MatrixSimdOuter<_>>(black_box(s)))
    });
}

// -------- fib16 --------
criterion_group!(
    bench_fib16,
    simple_fib16,
    simple_autovec_fib16,
    simd_dot_fib16,
    simd_outer_fib16
);

fn simple_fib16(c: &mut Criterion) {
    let s = 200_000_usize;
    c.bench_function("simple fib16", |b| {
        b.iter(|| fib_power::<16, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib16(c: &mut Criterion) {
    let s = 200_000_usize;
    c.bench_function("simple_autovec fib16", |b| {
        b.iter(|| fib_power::<16, MatrixSimpleAutovec<_>>(black_box(s)))
    });
}

fn simd_dot_fib16(c: &mut Criterion) {
    let s = 200_000_usize;
    c.bench_function("simd_dot fib16", |b| {
        b.iter(|| fib_power::<16, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_outer_fib16(c: &mut Criterion) {
    let s = 200_000_usize;
    c.bench_function("simd_outer fib16", |b| {
        b.iter(|| fib_power::<16, MatrixSimdOuter<_>>(black_box(s)))
    });
}

// -------- fib32 --------
criterion_group!(
    bench_fib32,
    simple_fib32,
    simple_autovec_fib32,
    simd_dot_fib32,
    simd_outer_fib32
);

fn simple_fib32(c: &mut Criterion) {
    let s = 10_000_usize;
    c.bench_function("simple fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib32(c: &mut Criterion) {
    let s = 10_000_usize;
    c.bench_function("simple_autovec fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimpleAutovec<_>>(black_box(s)))
    });
}

fn simd_dot_fib32(c: &mut Criterion) {
    let s = 10_000_usize;
    c.bench_function("simd_dot fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_outer_fib32(c: &mut Criterion) {
    let s = 10_000_usize;
    c.bench_function("simd_outer fib32", |b| {
        b.iter(|| fib_power::<32, MatrixSimdOuter<_>>(black_box(s)))
    });
}

// -------- fib64 --------
criterion_group!(
    bench_fib64,
    simple_fib64,
    simple_autovec_fib64,
    simd_dot_fib64,
    simd_outer_fib64
);

fn simple_fib64(c: &mut Criterion) {
    let s = 1_500_usize;
    c.bench_function("simple fib64", |b| {
        b.iter(|| fib_power::<64, MatrixSimple<_>>(black_box(s)))
    });
}

fn simple_autovec_fib64(c: &mut Criterion) {
    let s = 1_500_usize;
    c.bench_function("simple_autovec fib64", |b| {
        b.iter(|| fib_power::<64, MatrixSimpleAutovec<_>>(black_box(s)))
    });
}

fn simd_dot_fib64(c: &mut Criterion) {
    let s = 1_500_usize;
    c.bench_function("simd_dot fib64", |b| {
        b.iter(|| fib_power::<64, MatrixSimd<_>>(black_box(s)))
    });
}

fn simd_outer_fib64(c: &mut Criterion) {
    let s = 1_500_usize;
    c.bench_function("simd_outer fib64", |b| {
        b.iter(|| fib_power::<64, MatrixSimdOuter<_>>(black_box(s)))
    });
}
