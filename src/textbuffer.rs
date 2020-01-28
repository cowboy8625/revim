#![allow(dead_code)]

extern crate ropey;

use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::ops::RangeFrom;

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

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn remove(&mut self, idx: RangeFrom<usize>) {
        self.text.remove(idx);
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

    pub fn insert_char(&mut self, x: u16, y: u16, chr: char) {
        let line_index = self.text.line_to_char(y as usize);
        self.text.insert_char(line_index + x as usize, chr);
        self.dirty = true;
    }

    pub fn new_line(&mut self, x: u16, y: u16) {
        let line_index = self.text.line_to_char(y as usize);
        self.text.insert_char(line_index + x as usize, '\n');
        self.dirty = true;
    }
}
