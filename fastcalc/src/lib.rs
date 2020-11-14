/// 快速幂算法
///
/// 等效于 $b^e$
// ANCHOR: fastpow
pub fn fastpow(b: u64, e: u32) -> u64 {
    match e {
        0 => 1,
        1 => b,
        e => {
            if e & 1 == 1 {
                b * fastpow(b.wrapping_mul(b), e / 2)
            } else {
                fastpow(b.wrapping_mul(b), e / 2)
            }
        }
    }
}
// ANCHOR_END: fastpow

/// 快速幂算法（非递归版）
/// $b^e$
// ANCHOR: fastpow_norec
pub fn fastpow_norec(b: u64, e: u32) -> u64 {
    match e {
        0 => 1,
        1 => b,
        e => {
            let (mut b, mut e) = (b, e);
            let mut prod: u64 = 1;
            while e != 1 {
                if e & 1 == 1 {
                    prod = prod.wrapping_mul(b);
                }
                b = b.wrapping_mul(b);
                e /= 2;
            }
            prod.wrapping_mul(b)
        }
    }
}
// ANCHOR_END: fastpow_norec

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastpow() {
        assert_eq!(fastpow(2, 10), 1024);

        for i in 1..100 {
            for j in 1..10 {
                assert_eq!(fastpow(i, j), i.pow(j))
            }
        }
    }

    #[test]
    fn test_fastpow_norec() {
        assert_eq!(fastpow_norec(2, 10), 1024);

        for i in 1..100 {
            for j in 1..10 {
                assert_eq!(fastpow_norec(i, j), i.pow(j))
            }
        }
    }
}
