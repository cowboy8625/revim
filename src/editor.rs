use crate::{screen_size, ScreenVector};
use ropey::Rope;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Insert => "Insert",
            Self::Normal => "Normal",
            Self::Command => "Command",
        };
        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub struct Editor {
    pub rope: Rope,
    pub file_path: Option<String>,
    pub screen: ScreenVector,
    pub is_running: bool,
    pub mode: Mode,
    pub cursor: Cursor,
    pub command: String,
}

impl Editor {
    pub fn new(rope: Rope, file_path: Option<String>) -> Self {
        Self {
            rope,
            file_path,
            screen: screen_size(),
            is_running: true,
            mode: Mode::Normal,
            cursor: Cursor::default(),
            command: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub max_x: u16,
    pub max_y: u16,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            max_x: 0,
            max_y: 0,
        }
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
