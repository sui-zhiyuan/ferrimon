use crate::common::u32_mod::MOD_U64;
use crate::fib::matrix::Matrix;
use std::fmt::{Display, Formatter};
use std::ops::MulAssign;

#[derive(Clone, Debug)]
pub struct MatrixSimpleAutovec<const DIM: usize>([[u32; DIM]; DIM]);

impl<const DIM: usize> Matrix<DIM> for MatrixSimpleAutovec<DIM> {
    const ZERO: Self = Self([[0u32; DIM]; DIM]);
    const IDENTITY: Self = {
        let mut values = Self::ZERO.0;
        let mut i = 0;
        while i < DIM {
            values[i][i] = 1;
            i += 1;
        }

        Self(values)
    };

    fn new(values: [[u32; DIM]; DIM]) -> Self {
        Self(values)
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.0[row][col]
    }

    fn pow2(&mut self) {
        let value = self.clone();
        self.mul_inner(&value, &value);
    }
}

impl<const DIM: usize> MulAssign<&Self> for MatrixSimpleAutovec<DIM> {
    fn mul_assign(&mut self, rhs: &Self) {
        let lhs = self.clone();
        self.mul_inner(&lhs, rhs);
    }
}

impl<const DIM: usize> Display for MatrixSimpleAutovec<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Matrix::fmt(self, f)
    }
}

impl<const DIM: usize> MatrixSimpleAutovec<DIM> {
    fn mul_inner(&mut self, lhs: &Self, rhs: &Self) {
        for i in 0..DIM {
            for j in 0..DIM {
                let mut cell = 0u64;
                for k in 0..DIM {
                    cell += lhs.0[i][k] as u64 * rhs.0[k][j] as u64;
                }
                self.0[i][j] = (cell % MOD_U64) as u32;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::MatrixSimpleAutovec;
    use crate::fib::matrix::tests::{test_fib, test_fib_pow};

    #[test]
    fn test_fib_2() {
        test_fib::<2, MatrixSimpleAutovec<2>>(0..100);
    }
    #[test]
    fn test_fib_power_2() {
        test_fib_pow::<2, MatrixSimpleAutovec<2>>(0..20);
    }

    #[test]
    fn test_fib_8() {
        test_fib::<8, MatrixSimpleAutovec<8>>(0..100);
    }

    #[test]
    fn test_fib_power_8() {
        test_fib_pow::<8, MatrixSimpleAutovec<8>>(0..20);
    }

    #[test]
    fn test_fib_32() {
        test_fib::<32, MatrixSimpleAutovec<32>>(0..100);
    }
    #[test]
    fn test_fib_power_32() {
        test_fib_pow::<32, MatrixSimpleAutovec<32>>(0..20);
    }
}
