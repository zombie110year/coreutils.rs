use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg};

// ANCHOR: clap_app
fn get_cli_parser() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!("; "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("NUMBER").multiple(true))
}
// ANCHOR_END: clap_app

// ANCHOR: validator
fn validate_number(s: &String) -> Result<(), String> {
    for c in s.chars() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
            _ => return Err(format!("{:?} is not a valid positive integer", &s)),
        }
    }
    return Ok(());
}
// ANCHOR_END: validator

fn main() {
    let args = get_cli_parser().get_matches();
    if let Some(numbers) = args.values_of("NUMBER") {
        for n in numbers {
            check_and_print_factor(n.to_string());
        }
    } else {
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let number = buf.trim();
            check_and_print_factor(number.to_string());
        }
    }
}

fn check_and_print_factor(n: String) {
    // ANCHOR: true_validate
    if let Err(msg) = validate_number(&n) {
        eprintln!("factor: {}", msg);
    } else {
        let factors = factor(n.parse().unwrap())
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}: {}", &n, factors);
    }
    // ANCHOR_END: true_validate
}

// todo
fn factor(n: u64) -> Vec<u64> {
    if n < 4 {
        return vec![n];
    } else {
        let map = eratosthenes(n as usize);
        let mut ans = Vec::new();
        let mut n = n;

        while n != 1 {
            for p in map.iter() {
                let q = gcd(n, *p);
                if q != 1 {
                    ans.push(q);
                    n /= q;
                    break;
                }
            }
        }

        return ans;
    }
}

// ANCHOR: gcd
fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
    while a % b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return b;
}
// ANCHOR_END: gcd

// ANCHOR: eratosthenes
/// Eratosthenes 筛法求 n 以内的素数表
/// + 时间复杂度 $O(n^{\frac{3}{2}})$
/// + 空间复杂度 $O(n)$
fn eratosthenes(n: usize) -> Vec<u64> {
    let mut map = vec![true; n + 1];
    map[0] = false;
    map[1] = false;
    // O(\sqrt{n})
    for i in 2..=isqrt(n) {
        if map[i] {
            // O(n)
            for j in 2..=(n / i) {
                map[j * i] = false;
            }
        }
    }
    // O(n)
    map.iter()
        .enumerate()
        .filter(|(_i, b)| **b)
        .map(|(i, _b)| i as u64)
        .collect()
}
// ANCHOR_END: eratosthenes

fn isqrt(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        let r = 2 * isqrt(n / 4);
        if (r + 1) * (r + 1) > n {
            r
        } else {
            r + 1
        }
    }
}

/// Polard's rho 算法
fn polard_rho(n: u64) -> (u64, u64) {
    let mut x = 2;
    loop {
        let y = (x * x + 1) % n;
        let p = gcd(n, if x > y { x - y } else { y - x });
        if p > 1 {
            return (p, n / p);
        } else {
            x = y;
        }
    }
}

#[cfg(test)]
#[test]
fn test_eratosthenes() {
    assert_eq!(
        eratosthenes(100),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    )
}

#[cfg(test)]
#[test]
fn test_isqrt() {
    assert_eq!(isqrt(0), 0);
    assert_eq!(isqrt(1), 1);
    assert_eq!(isqrt(2), 1);
    assert_eq!(isqrt(3), 1);
    assert_eq!(isqrt(4), 2);
    for i in 1..256 {
        assert_eq!(isqrt(256 * 256 + i), 256);
    }
}
