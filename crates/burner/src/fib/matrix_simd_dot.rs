use crate::common::u32_mod::{MOD, MOD_U64};
use crate::fib::matrix::Matrix;
use std::fmt::{Display, Formatter};
use std::ops::MulAssign;
use std::simd::u32x2;

#[derive(Clone, Debug)]
pub struct MatrixSimdDot2([u32x2; 2]);

const MOD32X2: u32x2 = u32x2::splat(MOD);

impl Matrix<2> for MatrixSimdDot2 {
    const ZERO: Self = Self([u32x2::splat(0); 2]);
    const IDENTITY: Self = Self([
        u32x2::from_array([1u32, 0u32]),
        u32x2::from_array([0u32, 1u32]),
    ]);

    fn new(values: [[u32; 2]; 2]) -> Self {
        Self([
            u32x2::from_array(values[0]) % MOD32X2,
            u32x2::from_array(values[1]) % MOD32X2,
        ])
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.0[row].to_array()[col]
    }

    fn pow2(&mut self) {
        let lhs = self.clone();
        let mut rhs = Self::ZERO.clone();
        (rhs.0[0], rhs.0[1]) = self.0[0].interleave(self.0[1]);
        self.mul_inner(&lhs, &rhs);
    }
}

impl MulAssign<&Self> for MatrixSimdDot2 {
    fn mul_assign(&mut self, rhs: &Self) {
        let lhs = self.clone();
        let mut transpose_rhs = Self::ZERO.clone();
        (transpose_rhs.0[0], transpose_rhs.0[1]) = rhs.0[0].interleave(rhs.0[1]);
        self.mul_inner(&lhs, &transpose_rhs);
    }
}

impl MatrixSimdDot2 {
    #[inline]
    fn mul_inner(&mut self, lhs: &Self, rhs: &Self) {
        for i in 0..2 {
            for j in 0..2 {
                self.0[i][j] = Self::dot_mod2(lhs.0[i] * rhs.0[j]);
            }
        }
    }

    #[inline]
    fn dot_mod2(value: u32x2) -> u32 {
        let sum = value[0] as u64 + value[1] as u64;
        (sum % MOD_U64) as u32
    }
}

impl Display for MatrixSimdDot2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Matrix::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use super::MatrixSimdDot2;
    use crate::fib::matrix::tests::{test_fib, test_fib_pow};

    #[test]
    fn test_fib_2() {
        test_fib::<2, MatrixSimdDot2>(0..100);
    }
    #[test]
    fn test_fib_power_2() {
        test_fib_pow::<2, MatrixSimdDot2>(0..20);
    }
}
