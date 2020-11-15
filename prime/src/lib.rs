//! # libprime
//!
//! 与素数相关的算法

/// # Eratosthenes 筛法
///
/// 求 $[2, n)$ 区间内的素数
/// + 时间复杂度 $O(n^{\frac{3}{2}})$
/// + 空间复杂度 $O(n)$
// todo 自动推导 Vec<usize> -> Vec<T> 的转换
// ANCHOR: eratosthenes
pub fn eratosthenes(n: usize) -> Vec<usize> {
    let n = n.into();
    let mut table = vec![true; n];
    table[0] = false;
    table[1] = false;
    let mut head = 2;
    'prime: while table[head] {
        for notprime in (2 * head..n).step_by(head) {
            table[notprime] = false;
        }
        for next_prime in (head + 1)..n {
            if table[next_prime] {
                head = next_prime;
                continue 'prime;
            }
        }
        break;
    }

    let primes = table
        .iter()
        .enumerate()
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .collect();

    return primes;
}
// ANCHOR_END: eratosthenes

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eratosthenes() {
        assert_eq!(
            eratosthenes(100usize),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        )

        // note: 空间复杂度 O(n)，因此数字过大时会崩溃
        // eratosthenes(1000_000_000_000);
    }
}
