use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn _debug_to_file(message: std::fmt::Arguments) {
    // Create a path to the desired file
    let path = Path::new("log.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            if let Err(why) = file.write_fmt(format_args!("couldn't open {}: {}", display, why)) {
                panic!("couldn't create {}: {}", display, why);
            }
            file
        }

        Ok(file) => file,
    };

    // Writes to message to File.
    let _file = match file.write_fmt(message) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
