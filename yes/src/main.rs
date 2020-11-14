// ANCHOR: clap_app
use clap::{crate_version, crate_authors, crate_description, crate_name};
use clap::{App, Arg};

fn get_cli_parser() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("; "))
        .about(crate_description!())
        .arg(Arg::with_name("STRING").multiple(true))
}
// ANCHOR_END: clap_app

// ANCHOR: main
fn main() {
    let args = get_cli_parser().get_matches();
    let text = if let Some(texts) = args.values_of("STRING") {
        let texts: Vec<&str> = texts.into_iter().collect();
        texts.join(" ").to_string()
    } else {
        String::new()
    };

    loop {
        println!("{}", text);
    }
}
// ANCHOR_END: main