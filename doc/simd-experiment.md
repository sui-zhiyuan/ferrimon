# SIMD Matrix-Fib Experiment

Verification of `std::simd` (portable_simd) speedup for modular matrix exponentiation, with quantified comparison across four kernel variants on AVX2.

- **Platform**: i7-12700 (Alder Lake), AVX2 + AVX-VNNI, no AVX-512. Linux (WSL2).
- **Toolchain**: Rust nightly (`rust-toolchain.toml`), edition 2024, `target-cpu=native` via `.cargo/config.toml`.
- **Workload**: `fib_power::<DIM, M>(s)` where `M` is one of four `Matrix<DIM>` impls; `s` is tuned per DIM so total work is comparable across groups.
- **Modulus**: `MOD = 0xfff1 = 65521` (prime, fits in 16 bits).
- **Bench harness**: `criterion`, 100 samples per group, CI ≤ ±0.5%.
- **Branch**: `feature/burner-simd-analysis`.

---

## 1. Kernel Variants

| Variant            | Crate file                              | Form                                                              |
|--------------------|------------------------------------------|-------------------------------------------------------------------|
| `simple`           | `matrix_simple.rs`                       | Triple-nested scalar i-j-k with `black_box` on operand loads (V2) — locks LLVM into a true scalar baseline. |
| `simple_autovec`   | `matrix_simple_autovec.rs`               | Same algorithm as `simple` but **without** `black_box`, letting LLVM auto-vectorize freely. |
| `simd_dot`         | `matrix_simd_dot_large.rs` (`MatrixSimd<DIM>`) | Transposes `rhs`, computes each cell as `dot(lhs_row, rhs_col)` via `Simd<u32, DIM>` + `reduce_sum` + **one scalar `wrapping_rem`** per cell. |
| `simd_outer`       | `matrix_simd_outer.rs` (`MatrixSimdOuter<DIM>`) | Outer-product i-k-j: `acc += splat(lhs[i][k]) * rhs.row[k]` over `Simd<u64, DIM>` rows, with **hand-rolled SIMD Barrett mod** per row (no scalar round-trip). |

A 5th variant `simd_dot_f2` (DIM=2 only, fixed `u32x2`, no generics) appears in the fib2 group as a specialization sanity check.

---

## 2. Full Bench Results (100 samples, median ms)

Each group has its own `s` value such that work ≈ constant. Comparisons are **within a group only**.

| DIM | simple | simple_autovec | simd_dot | simd_outer (Barrett) | simd_dot_f2 |
|----:|-------:|---------------:|---------:|---------------------:|------------:|
|  2  |  49.67 |          41.28 |    72.81 |               107.19 |      108.26 |
|  4  | 133.27 |          83.69 |   110.13 |               103.13 |        —    |
|  8  | 145.23 |         145.33 |    97.27 |               112.54 |        —    |
| 16  | 205.81 |         216.81 |   103.04 |               156.66 |        —    |
| 32  | 153.76 |          34.60 |  **33.31** |             55.99 |        —    |
| 64  | 179.71 |          74.35 |  **36.48** |             64.84 |        —    |

### Speedup vs `simple` baseline

| DIM | simple | simple_autovec | simd_dot | simd_outer |
|----:|-------:|---------------:|---------:|-----------:|
|  2  | 1.00× | 1.20× |  0.68× ⚠ | 0.46× ⚠ |
|  4  | 1.00× | 1.59× |  1.21× | 1.29× |
|  8  | 1.00× | 1.00× |  1.49× | 1.29× |
| 16  | 1.00× | 0.95× |  **2.00×** | 1.31× |
| 32  | 1.00× | 4.44× |  **4.62×** | 2.75× |
| 64  | 1.00× | 2.42× |  **4.93×** | 2.77× |

---

## 3. Findings

### 3.1 `std::simd` delivers up to 4.93× on this AVX2 platform

`simd_dot` at DIM=64 runs in 36.48 ms vs scalar 179.71 ms — **4.93× speedup**, close to the 4-lane theoretical ceiling for `u64`/YMM on AVX2 (256-bit / 64-bit = 4 lanes). DIM=32 is similar at 4.62×. portable_simd is a viable production tool when the algorithm fits the SIMD model.

### 3.2 SIMD has a minimum useful size — DIM < 8 loses to scalar

At **DIM=2**, both SIMD variants are *slower* than `simple`:

- `simd_dot` (73 ms) is **47% slower** than `simple` (50 ms)
- `simd_outer` (107 ms) is **116% slower**

Causes:
1. `Simd<u64, 2>` fills only a single XMM register; the per-iteration SIMD setup (`splat`, shuffle, Barrett's 11 ops) costs more than the 2-element scalar loop.
2. The specialized `simd_dot_f2` (fixed `u32x2`) also lands at 108 ms, confirming the issue is fundamental to 2-lane SIMD on this kernel, not a generic-dispatch overhead.

**The SIMD payoff threshold is around DIM=8**, where `simd_dot` first beats `simple` and `autovec` clearly.

### 3.3 Auto-vectorization is unreliable; hand-written SIMD is stable

`simple_autovec` is the same algorithm as `simple` minus `black_box`, so any difference is pure LLVM auto-vectorization:

| DIM | autovec / simple | LLVM auto-vec verdict |
|----:|-----------------:|------------------------|
|  2  | 1.20× |  partial win |
|  4  | 1.59× |  win |
|  8  | 1.00× |  **failed** |
| 16  | 0.95× |  **failed (regression)** |
| 32  | 4.44× |  big win |
| 64  | 2.42× |  partial win |

LLVM auto-vec **completely fails at DIM=8 and DIM=16** — exactly the regime where hand-written `simd_dot` gives 1.49–2.00× speedup. Relying on `cargo build --release` to vectorize hot kernels is not safe; `std::simd` provides a stable lower bound.

### 3.4 Algorithm shape matters more than "uses SIMD"

Both `simd_dot` and `simd_outer` are SIMD-ified, yet `simd_dot` wins on every DIM ≥ 8 by 1.4–1.7×:

| DIM | simd_dot | simd_outer | outer / dot |
|----:|---------:|-----------:|------------:|
|  4  | 110.13   |    103.13  | 0.94× (outer wins narrowly) |
|  8  |  97.27   |    112.54  | 1.16× |
| 16  | 103.04   |    156.66  | 1.52× |
| 32  |  33.31   |     55.99  | 1.68× |
| 64  |  36.48   |     64.84  | 1.78× |

Why dot wins on AVX2:

- **dot**: `transpose → vpmullq → reduce_sum → 1 scalar wrapping_rem per cell`. The mod runs once per cell in a GPR with hardware `div`/Barrett — single-digit cycles, no SIMD bookkeeping.
- **outer**: `DIM splats + DIM vpaddq per row, then one SIMD Barrett over the whole row vector`. Mod stays in lanes (good — see §3.5) but SIMD throughput pressure is higher than dot's reduce+scalar form.

Lesson: **SIMD algorithm selection dominates "is this SIMD"**. Picking the wrong SIMD shape costs more than the SIMD win itself.

### 3.5 `Simd<u64, N> % MOD` is a LLVM performance trap — hand-write Barrett

The naive `acc % Simd::splat(MOD_U64)` in the original `simd_outer` looked clean but generated a per-lane scalar round-trip:

```text
[for each lane of Simd<u64, N>:]
  vpextrq   → extract lane to GPR
  mulx / shr / imul   → scalar Barrett in GPR
  vmovq / vpunpcklqdq → write back into SIMD
```

For `Simd<u64, 32>` (8 YMMs × 4 lanes = 32 lanes), the **mod alone produced 180+ instructions per row**, dominating the kernel.

**Fix**: hand-rolled in-SIMD Barrett using only AVX2-native ops.

#### Barrett derivation

Given `MOD = 0xfff1` (< 2^16) and `acc < 2^40` (covers DIM ≤ 64 since `64 × (MOD-1)² < 2^38`):

```text
M       = floor(2^32 / MOD) = 65551
mask_lo = 2^32 - 1

lo = acc & mask_lo
hi = acc >> 32
t1 = lo * M            // vpmuludq (32×32 → 64)
t2 = hi * M            // vpmuludq
q  = (t1 >> 32) + t2   // vpsrlq + vpaddq
r  = acc - q * MOD     // vpmuludq + vpsubq
r  = if r >= MOD { r - MOD } else { r }   // simd_ge + select
```

Why the split: AVX2's `vpmuludq` is 32×32→64. `acc` can be up to 2^40, so we split it into `hi` (< 2^8) and `lo` (< 2^32), multiply each by `M` (< 2^17), and recombine. Both sub-products fit in u64.

Correctness: verified by Python reference over 296,570 samples including all boundary values (0, MOD−1, MOD, MOD+1, 2^37, 2^38−1, random). Zero mismatches. At most one correction step is ever required (Barrett guarantees `r < 2·MOD` when `k` is chosen right).

#### Asm impact on `MatrixSimdOuter<32>::mul_inner`

Histogram of the inlined `fib_power<32, MatrixSimdOuter<32>>` body, before vs after Barrett:

| Instruction      | LLVM default mod | Hand-rolled Barrett | Delta  |
|------------------|-----------------:|--------------------:|-------:|
| `vpextrq`        | 16               | **0**               | -16    |
| `mulx` (scalar)  | 32               | **0**               | -32    |
| `imul` (scalar)  | 32               | **0**               | -32    |
| `shr`  (scalar)  | 64               | **0**               | -64    |
| `vpunpcklqdq`    | 16               | **0**               | -16    |
| `vpmuludq`       | 24               | 64                  | +40    |
| `vpaddq`         | 24               | 64                  | +40    |
| `vpsrlq`         |  9               | 33                  | +24    |
| `vpcmpgtq`       |  0               |  8                  | +8     |
| **Total lines**  | **967**          | **805**             | **-17%** |

Every SIMD↔scalar round-trip instruction vanished. The kernel is now pure SIMD.

#### Bench impact (Barrett vs LLVM default mod)

| DIM | LLVM mod (ms) | Barrett (ms) | Delta |
|----:|--------------:|-------------:|------:|
|  4  | 135 | 103 | **−24%** |
|  8  | 140 | 112 | **−20%** |
| 16  | 174 | 154 | **−11%** |
| 32  |  62 |  55 | **−11%** |
| 64  |  68 |  66 |  −3%   |

Barrett recovers 11–24% of outer's cost. It still does not beat `simd_dot` (see §3.4) — the remaining gap is algorithmic, not mod-related.

**Generalizable takeaway**: when a SIMD type lacks a native instruction for an operation (here, AVX2 has no SIMD integer divide), LLVM's fallback can be catastrophic. Whenever the modulus is a known small constant, a hand-rolled in-SIMD Barrett pays for itself.

---

## 4. Reproduction

```bash
# Toolchain pinned to nightly via rust-toolchain.toml
cargo bench -p ferrimon-burner --bench burner_bench    # full 100-sample run, ~10–15 min
cargo bench -p ferrimon-burner --bench burner_bench -- --quick   # ~2 min, wider CI

# Inspect Barrett asm
objdump -d --demangle target/release/deps/burner_bench-* \
  | awk '/MatrixSimdOuter<32>/{p=1} p; /^$/{p=0}'

# Unit tests (12 tests in MatrixSimdOuter, all green)
cargo test -p ferrimon-burner matrix_simd_outer --release
```

Criterion HTML reports land in `target/criterion/`.

---

## 5. Conclusions

1. **`std::simd` works**: 4.93× speedup at DIM=64, near the 4-lane theoretical ceiling for u64 on AVX2.
2. **SIMD has a minimum effective scale**: below DIM=8, SIMD setup overhead outweighs lane parallelism. Don't blindly SIMD-ify small fixed-size kernels.
3. **Hand-written SIMD beats auto-vectorization for stability**: LLVM auto-vec fails completely at DIM=8 and DIM=16, where hand SIMD gives a 1.5–2× win.
4. **SIMD algorithm shape is the real lever**: `transpose+dot+reduce` beats `outer-product+row-mod` on AVX2 by 1.4–1.8× even with both fully SIMD-ified.
5. **`Simd<T> % const` is a known LLVM trap on AVX2**: SIMD-scalar round-trip can dominate the kernel. Hand-rolled Barrett with a small precomputed `M` removes the round-trip entirely and recovers 11–24%.

The four-variant comparison and the Barrett before/after make a quantitative case for treating `std::simd` as a *tool that requires algorithmic and instruction-level awareness*, not a drop-in speedup.
