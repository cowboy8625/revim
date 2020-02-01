#![allow(dead_code)]

extern crate ropey;

use std::fs::{File, metadata};
use std::io::{BufReader, BufWriter, ErrorKind};
use std::ops::Range;

use ropey::iter::{Bytes, Chars, Chunks, Lines};
use ropey::{Rope, RopeSlice};

pub struct TextBuffer {
    pub text: Rope,
    pub path: Option<String>,
    pub dirty: bool,
}

impl TextBuffer {
    pub fn from_path(path: Option<String>) -> Self {
        let text = match &path {
            Some(n) => {
                // See if the file already exists
                if metadata(&n).is_ok() {
                    // If the file exists read from it
                    Rope::from_reader(BufReader::new(File::open(&n).unwrap_or_else(|error| {
                        if error.kind() == ErrorKind::NotFound {
                            File::create(n).unwrap_or_else(|error| {
                                panic!("Problem creating the file: {:?}", error);
                            })
                        } else {
                            panic!("Problem opening the file: {:?}", error);
                        }
                    })))
                    .unwrap()
                    // Able to just use unwrap here because the file should always exist due to the read check earlier
                }
                else {
                    // If the file doesn't exist just return a new rope and it will create the file during the first save
                    Rope::new()
                }
            },
            None => Rope::new(),
        };
        Self {
            text: text,
            path: path,
            dirty: false,
        }
    }

    pub fn get_line<'a>(&'a self, idx: usize) -> RopeSlice<'a> {
        self.text.line(idx)
    }

    pub fn line_to_str(&mut self, idx: usize) -> &str {
        self.text.line(idx)
            .as_str()
            .unwrap()
            .trim_end_matches("\n")
            .trim_end_matches("\r")
    }

    pub fn line_len(&self, idx: usize) -> u16 {
        self.text.line(idx)
            .as_str()
            .unwrap()
            .trim_end_matches("\n")
            .trim_end_matches("\r")
            .len() as u16
    }

    pub fn bytes<'a>(&'a self) -> Bytes<'a> {
        self.text.bytes()
    }

    pub fn chars<'a>(&'a self) -> Chars<'a> {
        self.text.chars()
    }

    pub fn lines<'a>(&'a self) -> Lines<'a> {
        self.text.lines()
    }

    pub fn chunks<'a>(&'a self) -> Chunks<'a> {
        self.text.chunks()
    }

    pub fn len_chars(&self) -> usize {
        self.text.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn remove(&mut self, x: u16, y: u16) {
        let line_idx = self.text.line_to_char(y as usize);
        let end = line_idx + x as usize;
        let start = if end > 0 { end - 1 } else { end };
        self.text.remove(start..end);
        self.dirty = true;
    }

    pub fn edit(&mut self, start: usize, end: usize, text: &str) {
        if start != end {
            self.text.remove(start..end);
        }
        if text.len() > 0 {
            self.text.insert(start, text);
        }
        self.dirty = true;
    }

    pub fn remove_line_break(&mut self, line_num: usize) {
        let idx = self.text.line_to_char(line_num);
        let start = self.text.line(line_num)
            .as_str()
            .unwrap()
            .trim_end_matches("\n")
            .trim_end_matches("\r")
            .len() as usize;
        let end = self.text.line(line_num)
            .as_str()
            .unwrap()
            .len() as usize;
        self.text.remove(start..end);
    }

    pub fn insert_line(&mut self, idx: usize, text: &str) {
        self.text.remove(idx..idx + text.len());
        self.text.insert(idx, text);
    }

    pub fn insert_char(&mut self, x: u16, y: u16, chr: char) {
        let line_index = self.text.line_to_char(y as usize);
        self.text.insert_char(line_index + x as usize, chr);
        self.dirty = true;
    }

    pub fn new_line(&mut self, x: u16, y: u16) {
        let line_index = self.text.line_to_char(y as usize);
        self.text.insert_char(line_index + x as usize, '\n');
        self.text.insert_char(line_index + x as usize, '\r');
        self.dirty = true;
    }

    pub fn is_empty(&self) -> bool {
        if self.text.len_chars() == 0 {
            true
        } else {
            false
        }
    }
}
