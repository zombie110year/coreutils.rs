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
    return vec![n];
}
