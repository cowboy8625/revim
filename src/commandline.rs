// External Crates
use clap::{App, Arg};

pub fn argparser() -> Option<String> {
    let matches = App::new("ReVim")
        .version("0.0.1")
        .author("Cowboy8625 <cowboy8625@protonmail.com>")
        .about("Cross Platform Small Simple Terminal Text Editor Written In Rust")
        .arg(Arg::with_name("in_file").index(1))
        .after_help(
            "Longer explanation to appear after the options when \
                                         displaying the help information from --help or -h",
        )
        .get_matches();

    match matches.value_of("in_file") {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}
