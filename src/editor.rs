use crate::textbuffer::TextBuffer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

enum CommandType {
    Quit,
    Save,
}

impl CommandType {
    fn new(cmd: &str) -> Vec<Self> {
        let mut commands = Vec::<Self>::new();
        for c in cmd.chars() {
            match c {
                'w' => commands.push(Self::Save),
                'q' => commands.push(Self::Quit),
                ' ' => break,
                _ => {}
            }
        }
        commands
    }
}

pub enum Mode {
    NORMAL,
    COMMAND,
    INSERT,
}

impl Mode {
    pub fn to_str(&self) -> &str {
        match self {
            Mode::NORMAL => "Normal",
            Mode::COMMAND => "Command",
            Mode::INSERT => "--insert--",
        }
    }
}

pub trait ModeSwitch {
    fn command_mode(&mut self);
    fn insert_mode(&mut self);
    fn normal_mode(&mut self);
    fn is_normal(&self) -> bool;
    fn is_command(&self) -> bool;
    fn is_insert(&self) -> bool;
}

pub struct Editor {
    pub mode: Mode,
    pub current_command: Vec<char>,
    pub textbuffer: TextBuffer,
    pub quit: bool,
}

impl Editor {
    pub fn new(mode: Mode, file_name: Option<String>) -> Self {
        Self {
            mode,
            current_command: Vec::new(),
            textbuffer: TextBuffer::from_path(file_name),
            quit: false,
        }
    }

    // Move to textbuffer?
    pub fn save(&mut self, name: Option<&str>) {
        let name = match &self.textbuffer.path {
            Some(n) => n,
            None => name.unwrap_or("new_text.txt"),
        };
        self.textbuffer
            .text
            .write_to(BufWriter::new(
                File::create(name).expect("File Creation Error in save_file"),
            ))
            .expect("BufWriter Error: save_file.");
    }

    pub fn run_command(&mut self) {
        let args: Vec<&str>;
        let com: Vec<CommandType>;

        let mut name: Option<&str> = None;
        let command: String = self.current_command.iter().collect::<String>();
        if command.contains(" ") {
            args = command.split(' ').collect();
            com = CommandType::new(args[0]);
            name = Some(args[1]);
        } else {
            com = CommandType::new(command.as_str())
        }
        for c in com.iter() {
            match c {
                CommandType::Quit => self.quit(),
                CommandType::Save => self.save(name),
            }
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}

impl ModeSwitch for Editor {
    fn command_mode(&mut self) {
        self.mode = Mode::COMMAND;
        self.current_command.push(':');
    }

    fn insert_mode(&mut self) {
        self.mode = Mode::INSERT;
    }

    fn normal_mode(&mut self) {
        self.mode = Mode::NORMAL;
    }

    fn is_normal(&self) -> bool {
        match self.mode {
            Mode::NORMAL => true,
            _ => false,
        }
    }

    fn is_command(&self) -> bool {
        match self.mode {
            Mode::COMMAND => true,
            _ => false,
        }
    }

    fn is_insert(&self) -> bool {
        match self.mode {
            Mode::INSERT => true,
            _ => false,
        }
    }
}
