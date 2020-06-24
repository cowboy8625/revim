// textbuffer.rs holds all rope data and logic.
//
// External Crates
use ropey::Rope;

// Standerd Library Crates
use std::fs::{metadata, OpenOptions};
use std::io::BufReader;

#[derive(Debug)]
pub struct TextBuffer {
    text: Rope,
    path: Option<String>,
}

impl TextBuffer {
    pub fn from_path(path: Option<String>) -> Self {
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

        Self { text, path }
    }

    pub fn line_len(&self, idx: usize) -> usize {
        self.text
            .line(idx)
            .as_str()
            .unwrap_or("")
            .trim_end_matches(|c| c == '\n' || c == '\r')
            .len()
    }

    pub fn len_lines(&self) -> usize {
        // Returns total of lines in file.
        self.text.len_lines()
    }

    pub fn line_to_line(&self, start: usize, end: usize) -> String {
        //Returns String from file.
        //from string line to ending line.
        //returns line == end
        let e = self.text.len_lines();
        let end = if end > e { e } else { end };
        let mut lines = String::new();
        (start..end).for_each(|idx| {
            lines.extend(self.text.line(idx).chars().filter(|&c| c != '\n'));
            lines.push_str("\r\n");
        });
        lines
    }

    pub fn _get_text(&self) -> String {
        // Returns all of the file.
        self.text.slice(..).to_string()
    }

    pub fn _get_path(&self) -> String {
        // This may need to be removed.
        self.path.as_deref().unwrap_or("No Path").to_owned()
    }

    pub fn insert_char(&mut self, x: u16, y: u16, chr: char) {
        let line_index = self.text.line_to_char(y as usize);
        self.text.insert_char(line_index + x as usize, chr);
    }

    pub fn get_line(&mut self, y: u16) -> String {
        self.text.line(y as usize).to_string()
    }
}
