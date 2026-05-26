use crate::common::u32_mod::MOD;

pub fn fib<const N: usize>(u: usize) -> u32 {
    let mut store = [0u32; N];
    store[N - 1] = 1u32;

    for _ in 0..u {
        let sum = store.iter().copied().sum::<u32>() % MOD;
        for j in 0..N - 1 {
            store[j] = store[j + 1];
        }
        store[N - 1] = sum;
    }

    store[N - 1]
}

pub fn fib_power<const N: usize>(u: usize) -> u32 {
    if u >= usize::BITS as usize {
        panic!("not supported");
    }

    fib::<N>(1 << u)
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn test_basic_2() {
        let result: Vec<u32> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34];

        for (i, v) in result.iter().enumerate() {
            assert_eq!(fib::<2>(i), *v, "round {i}");
        }
    }

    #[test]
    fn test_basic_4() {
        let result: Vec<u32> = vec![1, 1, 2, 4, 8, 15, 29, 56, 108];

        for (i, v) in result.iter().enumerate() {
            assert_eq!(fib::<4>(i), *v, "round {i}");
        }
    }
}
