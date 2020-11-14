use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg};

use std::collections::BTreeSet;

static mut ERROR_COLLECTOR: Vec<String> = Vec::new();

// ANCHOR: clap_app
fn get_cli_parser() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!("; "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("NUMBER")
                .multiple(true)
                .validator(validate_number),
        )
}
// ANCHOR_END: clap_app

// ANCHOR: validator
fn validate_number(s: String) -> Result<(), String> {
    let mut invalid_chars: BTreeSet<usize> = BTreeSet::new();
    for (i, c) in s.chars().enumerate() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
            _ => {
                invalid_chars.insert(i);
            }
        }
    }
    if invalid_chars.is_empty() {
        return Ok(());
    } else {
        // 用 ANSI Color Sequence 标注原字符串的错误数位
        let red_begin = "\x1b[31m";
        let red_end = "\x1b[0m";
        let mut report = String::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            if invalid_chars.contains(&i) {
                report.push_str(red_begin);
                report.push(c);
                report.push_str(red_end);
            } else {
                report.push(c);
            }
        }

        // 读写 static 变量需要 unsafe
        unsafe {
            ERROR_COLLECTOR.push(report);
        }

        return Ok(());
    }
}
// ANCHOR_END: validator

fn main() {
    let args = get_cli_parser().get_matches();
    // ANCHOR: true_validate
    unsafe {
        if !ERROR_COLLECTOR.is_empty() {
            eprintln!("Invalid value for '<NUMBER>...':");
            for msg in ERROR_COLLECTOR.iter() {
                eprintln!("{}", msg);
            }
            std::process::exit(1);
        }
    }
    // ANCHOR_END: true_validate

    let numbers: Vec<u64> = args
        .values_of("NUMBER")
        .expect("no number input!")
        .into_iter()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    println!("{:?}", numbers);
}
