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

/// 快速乘算法
/// $ab$
// ANCHOR: fastmul
pub fn fastmul(a: u64, b: u64) -> u64 {
    match b {
        0 => 0,
        1 => a,
        b => {
            if b & 1 == 1 {
                fastmul(a.wrapping_add(a), b / 2).wrapping_add(a)
            } else {
                fastmul(a.wrapping_add(a), b / 2)
            }
        }
    }
}
// ANCHOR_END: fastmul

/// 快速乘算法（非递归版）
/// $ab$
// ANCHOR: fastmul_norec
pub fn fastmul_norec(a: u64, b: u64) -> u64 {
    match b {
        0 => 0,
        1 => a,
        b => {
            let (mut a, mut b) = (a, b);
            let mut sum: u64 = 0;
            while b != 1 {
                if b & 1 == 1 {
                    sum = sum.wrapping_add(a);
                }
                a = a.wrapping_add(a);
                b /= 2;
            }
            sum.wrapping_add(a)
        }
    }
}
// ANCHOR_END: fastmul_norec

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

    #[test]
    fn test_fastmul() {
        for i in 0..1000 {
            for j in 0..1000 {
                assert_eq!(fastmul(i, j), i * j);
            }
        }

        assert_eq!(
            fastmul(std::u64::MAX, std::u64::MAX),
            std::u64::MAX.wrapping_mul(std::u64::MAX)
        );
    }
    #[test]
    fn test_fastmul_norec() {
        for i in 0..1000 {
            for j in 0..1000 {
                assert_eq!(fastmul_norec(i, j), i * j);
            }
        }

        assert_eq!(
            fastmul_norec(std::u64::MAX, std::u64::MAX),
            std::u64::MAX.wrapping_mul(std::u64::MAX)
        );
    }
}
