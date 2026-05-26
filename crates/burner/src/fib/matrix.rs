use std::fmt::{Debug, Display, Formatter};
use std::ops::MulAssign;

pub trait Matrix<const DIM: usize>
where
    Self: Clone + Debug + Display,
    Self: for<'a> MulAssign<&'a Self>,
{
    const ZERO: Self;
    const IDENTITY: Self;
    fn new(values: [[u32; DIM]; DIM]) -> Self;

    fn get(&self, row: usize, col: usize) -> u32;

    fn pow2(&mut self);

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut longest = 1usize;

        for row in 0..DIM {
            for col in 0..DIM {
                longest = longest.max(digit_len(self.get(row, col)));
            }
        }

        let width = longest + 1;
        for row in 0..DIM {
            for col in 0..DIM {
                write!(f, "{:>width$}", self.get(row, col), width = width)?;
            }
            if row + 1 < DIM {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub fn fib<const DIM: usize, M: Matrix<DIM>>(n: usize) -> u32 {
    let mut base = new_transfer_matrix::<DIM, M>();
    let mut acc = M::IDENTITY;
    let mut exp = n;

    while exp > 0 {
        if exp & 1 == 1 {
            acc *= &base;
        }
        exp >>= 1;
        if exp > 0 {
            base.pow2();
        }
    }

    acc.get(0, 0)
}

pub fn fib_power<const DIM: usize, M: Matrix<DIM>>(u: usize) -> u32 {
    let mut base = new_transfer_matrix::<DIM, M>();
    for _ in 0..u {
        base.pow2();
    }

    base.get(0, 0)
}

fn new_transfer_matrix<const DIM: usize, M: Matrix<DIM>>() -> M {
    let mut base = [[0; DIM]; DIM];
    for row in base.iter_mut() {
        row[0] = 1;
    }
    for (i, row) in base.iter_mut().enumerate() {
        if i + 1 < DIM {
            row[i + 1] = 1;
        }
    }
    M::new(base)
}

fn digit_len(mut value: u32) -> usize {
    if value == 0 {
        return 1;
    }

    let mut len = 0usize;
    while value > 0 {
        len += 1;
        value /= 10;
    }

    len
}

#[cfg(test)]
pub mod tests {
    use crate::fib::matrix::Matrix;
    use crate::fib::{basic_fib, matrix};

    pub fn test_fib<const DIM: usize, M: Matrix<DIM>>(range: impl IntoIterator<Item = usize>) {
        for i in range {
            assert_eq!(
                basic_fib::fib::<DIM>(i),
                matrix::fib::<DIM, M>(i),
                "index {i}"
            );
        }
    }

    pub fn test_fib_pow<const DIM: usize, M: Matrix<DIM>>(range: impl IntoIterator<Item = usize>) {
        for i in range {
            assert_eq!(
                basic_fib::fib_power::<DIM>(i),
                matrix::fib_power::<DIM, M>(i),
                "index {i}"
            );
        }
    }
}
