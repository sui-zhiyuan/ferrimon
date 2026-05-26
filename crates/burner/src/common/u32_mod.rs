pub const MOD: u32 = 0xfff1;
pub const MOD_U64: u64 = MOD as u64;


#[cfg(test)]
mod tests {
    use crate::common::u32_mod::MOD;

    #[test]
    fn check_largest_prem() {
        assert!(is_prim(MOD));

        for i in (MOD + 1)..=0xffff {
            assert!(!is_prim(i), "value {}", i);
        }
    }

    fn is_prim(v: u32) -> bool {
        for i in 2..v {
            if v.is_multiple_of(i) {
                return false;
            }

            let p = i * i;
            if p >= v {
                break;
            }
        }
        true
    }
}
