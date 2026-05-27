use crate::common::u32_mod::MOD_U64;
use crate::fib::matrix::Matrix;
use std::fmt::{Display, Formatter};
use std::ops::MulAssign;
use std::simd::Select;
use std::simd::Simd;
use std::simd::cmp::SimdPartialOrd;

#[derive(Clone, Debug)]
pub struct MatrixSimdOuter<const DIM: usize>([Simd<u64, DIM>; DIM]);

impl<const DIM: usize> Matrix<DIM> for MatrixSimdOuter<DIM> {
    const ZERO: Self = Self([Simd::<u64, DIM>::splat(0); DIM]);
    const IDENTITY: Self = {
        let mut values = [[0u64; DIM]; DIM];
        let mut mat = Self::ZERO.0;
        let mut i = 0;
        while i < DIM {
            values[i][i] = 1;
            mat[i] = Simd::<u64, DIM>::from_array(values[i]);
            i += 1;
        }
        Self(mat)
    };

    fn new(values: [[u32; DIM]; DIM]) -> Self {
        let mut mat = Self::ZERO.clone();
        for (i, row) in values.into_iter().enumerate().take(DIM) {
            let mut widened = [0u64; DIM];
            for j in 0..DIM {
                widened[j] = row[j] as u64;
            }
            mat.0[i] = Simd::<u64, DIM>::from_array(widened);
        }
        mat
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.0[row][col] as u32
    }

    fn pow2(&mut self) {
        let value = self.clone();
        self.mul_inner(&value, &value);
    }
}

impl<const DIM: usize> MulAssign<&Self> for MatrixSimdOuter<DIM> {
    fn mul_assign(&mut self, rhs: &Self) {
        let lhs = self.clone();
        self.mul_inner(&lhs, rhs);
    }
}

impl<const DIM: usize> Display for MatrixSimdOuter<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Matrix::fmt(self, f)
    }
}

impl<const DIM: usize> MatrixSimdOuter<DIM> {
    // Barrett reduction constants for MOD = 0xfff1 (65521):
    //   M = floor(2^32 / MOD) = 65551
    // Given acc < 2^40 (safe up to DIM=64), compute acc % MOD without
    // a hardware u64 divider, staying entirely in SIMD lanes (vpmuludq +
    // vpsrlq + vpaddq + vpsubq + select). The default `%` lowering
    // extracts every lane to a GPR and runs scalar mulx/shr per lane,
    // which dominates the kernel cost.
    const BARRETT_M: u64 = (1u64 << 32) / MOD_U64;
    const MASK_LO32: u64 = (1u64 << 32) - 1;

    #[inline]
    fn barrett_mod(acc: Simd<u64, DIM>) -> Simd<u64, DIM> {
        let m = Simd::<u64, DIM>::splat(Self::BARRETT_M);
        let modulus = Simd::<u64, DIM>::splat(MOD_U64);
        let mask_lo = Simd::<u64, DIM>::splat(Self::MASK_LO32);

        let lo = acc & mask_lo;
        let hi = acc >> Simd::<u64, DIM>::splat(32);
        let t1 = lo * m;
        let t2 = hi * m;
        let q = (t1 >> Simd::<u64, DIM>::splat(32)) + t2;
        let r = acc - q * modulus;
        let mask = r.simd_ge(modulus);
        mask.select(r - modulus, r)
    }

    #[inline]
    fn mul_inner(&mut self, lhs: &Self, rhs: &Self) {
        for i in 0..DIM {
            let mut acc = Simd::<u64, DIM>::splat(0);
            for k in 0..DIM {
                let a = Simd::<u64, DIM>::splat(lhs.0[i][k]);
                acc += a * rhs.0[k];
            }
            self.0[i] = Self::barrett_mod(acc);
        }
    }
}

#[cfg(test)]
mod test {
    use super::MatrixSimdOuter;
    use crate::fib::matrix::tests::{test_fib, test_fib_pow};

    #[test]
    fn test_fib_2() {
        test_fib::<2, MatrixSimdOuter<2>>(0..100);
    }
    #[test]
    fn test_fib_power_2() {
        test_fib_pow::<2, MatrixSimdOuter<2>>(0..20);
    }

    #[test]
    fn test_fib_4() {
        test_fib::<4, MatrixSimdOuter<4>>(0..100);
    }
    #[test]
    fn test_fib_power_4() {
        test_fib_pow::<4, MatrixSimdOuter<4>>(0..20);
    }

    #[test]
    fn test_fib_8() {
        test_fib::<8, MatrixSimdOuter<8>>(0..100);
    }
    #[test]
    fn test_fib_power_8() {
        test_fib_pow::<8, MatrixSimdOuter<8>>(0..20);
    }

    #[test]
    fn test_fib_16() {
        test_fib::<16, MatrixSimdOuter<16>>(0..100);
    }
    #[test]
    fn test_fib_power_16() {
        test_fib_pow::<16, MatrixSimdOuter<16>>(0..20);
    }

    #[test]
    fn test_fib_32() {
        test_fib::<32, MatrixSimdOuter<32>>(0..100);
    }
    #[test]
    fn test_fib_power_32() {
        test_fib_pow::<32, MatrixSimdOuter<32>>(0..20);
    }

    #[test]
    fn test_fib_64() {
        test_fib::<64, MatrixSimdOuter<64>>(0..100);
    }
    #[test]
    fn test_fib_power_64() {
        test_fib_pow::<64, MatrixSimdOuter<64>>(0..20);
    }
}
