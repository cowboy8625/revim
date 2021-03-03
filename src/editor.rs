use std::fmt;
use crate::{screen_size, ScreenVector};
use ropey::Rope;

#[derive(Debug)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Self::Insert  => "Insert",
            Self::Normal  => "Normal",
            Self::Command => "Command",
        };
        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub struct Editor {
    pub rope: Rope,
    pub screen: ScreenVector,
    pub is_running: bool,
    pub mode: Mode,
    pub cursor: (u16, u16),
    pub command: String,
}

impl Editor {
    pub fn new(rope: Rope) -> Self {
        Self {
            rope,
            screen: screen_size(),
            is_running: true,
            mode: Mode::Normal,
            cursor: (0, 0),
            command: String::new(),
        }
    }
}
