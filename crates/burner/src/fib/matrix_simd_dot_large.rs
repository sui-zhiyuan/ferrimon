use crate::common::u32_mod::MOD_U64;
use crate::fib::matrix::Matrix;
use std::fmt::{Display, Formatter};
use std::ops::MulAssign;
use std::simd::Simd;
use std::simd::num::SimdUint;

#[derive(Clone, Debug)]
pub struct MatrixSimd<const DIM: usize>([Simd<u32, DIM>; DIM]);

impl<const DIM: usize> Matrix<DIM> for MatrixSimd<DIM> {
    const ZERO: Self = Self([Simd::<u32, DIM>::splat(0); DIM]);
    const IDENTITY: Self = {
        let mut values = [[0; DIM]; DIM];
        let mut mat = Self::ZERO.0;
        let mut i = 0;
        while i < DIM {
            values[i][i] = 1;
            mat[i] = Simd::<u32, DIM>::from_array(values[i]);
            i += 1;
        }
        Self(mat)
    };

    fn new(values: [[u32; DIM]; DIM]) -> Self {
        let mut mat = Self::ZERO.clone();
        for (i, row) in values.into_iter().enumerate().take(DIM) {
            mat.0[i] = Simd::<u32, DIM>::from_array(row);
        }
        mat
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.0[row].to_array()[col]
    }

    fn pow2(&mut self) {
        let lhs = self.clone();
        let mut rhs = Self::ZERO.clone();
        self.transpose_to(&mut rhs);
        self.mul_inner(&lhs, &rhs);
    }
}

impl<const DIM: usize> MulAssign<&Self> for MatrixSimd<DIM> {
    fn mul_assign(&mut self, rhs: &Self) {
        let lhs = self.clone();
        let mut transpose_rhs = Self::ZERO.clone();
        rhs.transpose_to(&mut transpose_rhs);
        self.mul_inner(&lhs, &transpose_rhs);
    }
}

impl<const DIM: usize> MatrixSimd<DIM> {
    #[inline]
    fn mul_inner(&mut self, lhs: &Self, rhs: &Self) {
        for i in 0..DIM {
            for j in 0..DIM {
                self.0[i][j] = Self::sum_mod(lhs.0[i] * rhs.0[j]);
            }
        }
    }

    #[inline]
    fn transpose_to(&self, target: &mut Self) {
        for i in 0..DIM {
            for j in 0..DIM {
                target.0[j][i] = self.0[i][j];
            }
        }
    }

    #[inline]
    fn sum_mod(vec: Simd<u32, DIM>) -> u32 {
        // Widen before reduce: DIM lanes of u32 products can sum to
        // DIM * (2^32 - 1) ≈ 2^37 for DIM=32, overflowing u32.
        let widened: Simd<u64, DIM> = vec.cast::<u64>();
        widened.reduce_sum().wrapping_rem(MOD_U64) as u32
    }
}

impl<const DIM: usize> Display for MatrixSimd<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Matrix::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use super::MatrixSimd;
    use crate::fib::matrix::tests::{test_fib, test_fib_pow};

    #[test]
    fn test_fib_2() {
        test_fib::<2, MatrixSimd<2>>(0..100);
    }
    #[test]
    fn test_fib_power_2() {
        test_fib_pow::<2, MatrixSimd<2>>(0..20);
    }

    #[test]
    fn test_fib_8() {
        test_fib::<8, MatrixSimd<8>>(0..100);
    }

    #[test]
    fn test_fib_power_8() {
        test_fib_pow::<8, MatrixSimd<8>>(0..20);
    }

    #[test]
    fn test_fib_32() {
        test_fib::<32, MatrixSimd<32>>(0..100);
    }
    #[test]
    fn test_fib_power_32() {
        test_fib_pow::<32, MatrixSimd<32>>(0..20);
    }
}
