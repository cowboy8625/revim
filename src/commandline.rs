use crate::Rope;
use std::fs::{metadata, OpenOptions};
use std::io::BufReader;

// External Crates
use clap::{crate_version, App, Arg};

pub fn argparser() -> Option<String> {
    let matches = App::new("ReVim")
        .version(crate_version!())
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

// wrapper around Rope for a drity flag.
pub fn from_path(path: Option<String>) -> (Rope, Option<String>) {
    let text = path
        .as_ref()
        .filter(|path| metadata(&path).is_ok())
        .map_or_else(Rope::new, |path| {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Problem opening the file");

            Rope::from_reader(BufReader::new(file)).unwrap()
        });

    (text, path)
}
