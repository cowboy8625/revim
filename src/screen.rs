use crate::dimensions::Dimensions;
use crate::editor::{Editor, Mode, ModeSwitch};
use crate::position::Position;

use std::cmp::min;
use std::io::{BufReader, BufWriter, Stdout, Write};

use crossterm::{
    cursor, style, execute, queue, terminal,
    terminal::{Clear, ClearType},
};

const WELCOME: &str = r#"ReVim - Rust Edition Vim - Version 0.0.1
To Quit: :q, To INSERT Text: i, For Help: :help <key or keyword>
GitHub: https://www.github.com/cowboy8625/ReVim"#;

pub struct Screen {
    w: Stdout,
    pub e: Editor,
    dim: Dimensions,
    cursor: Position,
    can_welcome: bool,
}

impl Screen {
    pub fn new(w: Stdout, e: Editor) -> Self {
        Self {
            w,
            e,
            dim: Screen::get_terminal_size(),
            cursor: Position::new(0, 0),
            can_welcome: true,
        }
    }

    pub fn update(&mut self) {

        self.render_file();
        self.welcome_message();
        self.render_empty_lines();
        self.status_bar_mode();
        self.message_bar_display(self.dim.h);
        self.w.flush().unwrap();
    }

    pub fn move_up(&mut self) {
        if self.cursor.y > 0 {
            let include_line_ends = match self.e.mode {
                Mode::INSERT => 0,
                _ => 1,
            };

            let x_val = if self.cursor.x == self.e.textbuffer.line_len(self.cursor.y as usize) {
                self.e.textbuffer.line_len(self.cursor.y as usize - 1)
            } else {
                min(
                    self.cursor.x,
                    self.e.textbuffer.line_len(self.cursor.y as usize - 1) - include_line_ends,
                )
            };
            queue!(self.w, cursor::MoveTo(x_val, self.cursor.y - 1)).unwrap();
            if x_val != self.cursor.x {
                self.cursor.x = x_val;
            }
            self.cursor.move_up(1, 0);
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.y + 1 < self.e.textbuffer.text.len_lines() as u16 {
            let include_line_ends = match self.e.mode {
                Mode::INSERT => 0,
                _ => 1,
            };

            let x_val = if self.cursor.x == self.e.textbuffer.line_len(self.cursor.y as usize) {
                self.e.textbuffer.line_len(self.cursor.y as usize + 1)
            } else {
                min(
                    self.cursor.x,
                    self.e.textbuffer.line_len(self.cursor.y as usize + 1) - include_line_ends,
                )
            };

            queue!(self.w, cursor::MoveTo(x_val, self.cursor.y + 1)).unwrap();

            if x_val != self.cursor.x {
                self.cursor.x = x_val;
            }

            self.cursor.move_down(1, self.dim.h);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor.x > 0 {
            queue!(self.w, cursor::MoveLeft(1)).unwrap();
            self.cursor.move_left(1, 0);
        } else if self.cursor.y > 0 {
            let include_line_ends = match self.e.mode {
                Mode::INSERT => 0,
                _ => 1,
            };

            let x_val = self.e.textbuffer.line_len(self.cursor.y as usize - 1) - include_line_ends;

            queue!(self.w, cursor::MoveTo(x_val, self.cursor.y - 1)).unwrap();

            self.cursor.x = x_val;
            self.cursor.move_up(1, 0);
        }
    }

    pub fn move_right(&mut self) {
        let include_line_ends = match self.e.mode {
            Mode::INSERT => 1,
            _ => 0,
        };

        if self.cursor.x + 1
            < self.e.textbuffer.line_len(self.cursor.y as usize) + include_line_ends
        {
            queue!(self.w, cursor::MoveRight(1)).unwrap();
            self.cursor.move_right(1, self.dim.w);
        } else if self.cursor.y + 1 < self.e.textbuffer.len_lines() as u16 {
            queue!(self.w, cursor::MoveTo(0, self.cursor.y + 1)).unwrap();

            self.cursor.x = 0;
            self.cursor.move_down(1, self.dim.h);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor.x == 0 && self.cursor.y != 0 {
            // Goto end of line above.

            let line_char_len = self.e.textbuffer.line_len((self.cursor.y - 1) as usize);

            self.e
                .textbuffer
                .remove_line_break((self.cursor.y - 1) as usize);
            self.cursor.move_up(1, 0);
            self.cursor.move_right(line_char_len, self.dim.w);

            queue!(
                self.w,
                cursor::Hide,
                cursor::MoveUp(1),
                cursor::MoveRight(line_char_len),
                cursor::Show,
            )
            .unwrap();
        } else {
            // backspace to left
            self.e.textbuffer.remove(self.cursor.x, self.cursor.y);
            self.cursor.move_left(1, 0);
            queue!(
                self.w,
                cursor::Hide,
                cursor::MoveLeft(1),
                style::Print(' '),
                cursor::MoveLeft(1),
                cursor::Show,
            )
            .unwrap();
        }
    }

    pub fn line_break(&mut self) {
        self.e.textbuffer.new_line(self.cursor.x, self.cursor.y);
        self.cursor.x = 0;
        self.cursor.move_down(1, self.dim.h);
        queue!(
            self.w,
            cursor::Hide,
            cursor::MoveToNextLine(1),
            cursor::Show
        )
        .unwrap();
    }

    pub fn insert_char(&mut self, chr: char) {
        self.e
            .textbuffer
            .insert_char(self.cursor.x, self.cursor.y, chr);
        self.move_right();
    }

    pub fn start(&mut self) {
        execute!(
            self.w,
            cursor::Show,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            style::ResetColor,
        )
        .unwrap();
        for idx in 0..self.e.textbuffer.len_lines() {
            if idx == self.dim.h as usize {
                break;
            }
            queue!(
                self.w,
                style::Print(format!("{}\r", self.e.textbuffer.get_line(idx))),
            )
            .unwrap();
        }
        queue!(self.w, cursor::MoveTo(0, 0)).unwrap();
    }

    pub fn end(&mut self) {
        execute!(self.w, terminal::LeaveAlternateScreen).unwrap();
    }

    pub fn resize(&mut self, x: u16, y: u16) {
        queue!(self.w, terminal::SetSize(x, y), Clear(ClearType::All),).unwrap();
    }
}

/*** private ***/

impl Screen {

    fn render_file(&mut self) {
        if !self.e.textbuffer.is_empty() && self.e.textbuffer.dirty {
            queue!(
                self.w,
                cursor::SavePosition,
                cursor::MoveTo(0, 0),
                Clear(ClearType::All),
                style::Print(self.e.textbuffer.text.slice(..)),
                cursor::RestorePosition,
            )
            .unwrap();
        }
    }

    fn render_empty_lines(&mut self) {
        let start = self.e.textbuffer.lines().len() as u16;

        queue!(
            self.w,
            cursor::SavePosition,
            cursor::Hide,
            style::SetForegroundColor(style::Color::Cyan),
        ).unwrap();

        for i in start..self.dim.h - 2 {
            queue!(
                self.w,
                cursor::MoveToNextLine(1),
                style::Print('~')
            ).unwrap();
        }

        queue!(
            self.w,
            cursor::RestorePosition,
            cursor::Show,
            style::ResetColor,
        ).unwrap();
    }

    fn welcome_message(&mut self) {
        if self.can_welcome {
            if self.e.is_insert() {
                queue!(self.w, Clear(ClearType::All)).unwrap();
            } else {
                for (y, msg) in WELCOME.split("\n").enumerate() {
                    let x = self.dim.w / 2 - ((msg.len() as u16) / 2);
                    let y = self.dim.h / 3 + y as u16;
                    queue!(
                        self.w,
                        cursor::SavePosition,
                        cursor::MoveTo(x, y),
                        style::Print(msg),
                        cursor::RestorePosition,
                    )
                    .unwrap();
                }
            }
            if self.e.textbuffer.dirty {
                self.can_welcome = false;
            }
        }
    }

    fn status_bar_mode(&mut self) {
        let mode = self.e.mode.to_str();
        queue!(
            self.w,
            cursor::SavePosition,
            cursor::MoveTo(0, self.dim.h - 2),
            Clear(ClearType::CurrentLine),
            style::Print(format!(
                "{}, location: {}/{}",
                mode, self.cursor.x, self.cursor.y
            )),
            cursor::RestorePosition,
            style::ResetColor
        )
        .unwrap();
    }

    fn message_bar_display(&mut self, y: u16) {
        if self.e.is_command() {
            queue!(
                self.w,
                cursor::SavePosition,
                cursor::MoveTo(0, y),
                Clear(ClearType::CurrentLine),
                style::Print(format!(
                    "{}",
                    self.e.current_command.iter().map(|c| c).collect::<String>()
                )),
                cursor::RestorePosition,
            )
            .unwrap();
        }
    }

    fn editor_alert(&mut self, msg: &str) {
        queue!(
            self.w,
            cursor::SavePosition,
            cursor::MoveTo(0, self.dim.h - 1),
            Clear(ClearType::CurrentLine),
            style::Print(msg),
            cursor::RestorePosition,
        )
        .unwrap();
    }
}

impl Screen {
    fn get_terminal_size() -> Dimensions {
        match terminal::size() {
            Ok(v) => Dimensions::new(v.0, v.1),
            Err(e) => panic!("Terminal Size ERROR: {}", e),
        }
    }
}
