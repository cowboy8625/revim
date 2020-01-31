use crate::editor::{Editor, ModeSwitch};
use crate::position::Position;
use crate::dimensions::Dimensions;

use std::io::{BufReader, BufWriter, Stdout, Write};

use crossterm::{
    cursor,
    style::{Print},
    execute, queue, style, terminal,
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
}

impl Screen {
    pub fn new(w: Stdout, e: Editor) -> Self {
        Self {
            w,
            e,
            dim: Screen::get_terminal_size(),
            cursor: Position::new(0, 0),
        }
    }

    pub fn update(&mut self) {
        // self.clear();
        // self.welcome_message(w, h);

        //problem is in this block
        let (x, y) = match cursor::position() {
            Ok(v) => (v.0, v.1),
            Err(e) => panic!("Curser Postion ERROR: {}", e),
        };
        let msg = format!("X: {}, y: {}", x, y);
        self.editor_alert(&msg);
        if !self.e.textbuffer.is_empty() {
            self.render_text();
        }

        self.status_bar_mode();
        if self.e.is_command() {
            self.message_bar_display(self.dim.h);
        }
        self.w.flush().unwrap();
    }


    pub fn move_up(&mut self) {
        queue!(self.w, cursor::MoveUp(1)).unwrap();
        self.cursor.move_up(1, 0);
    }

    pub fn move_down(&mut self) {
        queue!(self.w, cursor::MoveDown(1)).unwrap();
        self.cursor.move_down(1, self.dim.h);
    }

    pub fn move_left(&mut self) {
        queue!(self.w, cursor::MoveLeft(1)).unwrap();
        self.cursor.move_left(1, 0);
    }

    pub fn move_right(&mut self) {
        queue!(self.w, cursor::MoveRight(1)).unwrap();
        self.cursor.move_right(1, self.dim.w);
    }

    pub fn backspace(&mut self) {
        self.e.textbuffer.remove(self.cursor.x, self.cursor.y);
        self.cursor.move_left(1, 0);
        queue!(
            self.w,
            cursor::Hide,
            cursor::MoveLeft(1),
            Print(' '),
            cursor::MoveLeft(1),
            cursor::Show,
        ).unwrap();
    }

    pub fn line_break(&mut self) {
        self.e.textbuffer.new_line(self.cursor.x, self.cursor.y);
        self.cursor.x = 0;
        self.cursor.move_down(1, self.dim.h);
        queue!(self.w,
               cursor::Hide,
               cursor::MoveToNextLine(1),
               cursor::Show
            ).unwrap();
    }

    pub fn insert_char(&mut self, chr: char) {
        self.e.textbuffer.insert_char(self.cursor.x, self.cursor.y, chr);
        self.cursor.move_right(1, self.dim.w);
    }

    pub fn start(&mut self) {
        execute!(
            self.w,
            cursor::Show,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            style::ResetColor,
        ).unwrap();
        for idx in 0..self.e.textbuffer.len_lines() {
            if idx == self.dim.h as usize { break; }
            queue!(
                self.w,
                Print(format!("{}\r", self.e.textbuffer.get_line(idx))),
                ).unwrap();
        }
        queue!(self.w, cursor::MoveTo(0, 0)).unwrap();

    }

    pub fn end(&mut self) {
        execute!(self.w, terminal::LeaveAlternateScreen).unwrap();
    }

    pub fn resize(&mut self, x: u16, y: u16) {
        queue!(
            self.w,
            terminal::SetSize(x, y),
            Clear(ClearType::All),
            ).unwrap();
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

/*** private ***/

impl Screen {
    fn render_line(&mut self, y: u16, line: &str) {
        queue!(
            self.w,
            cursor::MoveTo(0, y),
            Print(line),
            ).unwrap();
    }

    fn render_char(&mut self, chr: char) {
        queue!(
            self.w,
            cursor::MoveTo(0, self.cursor.y),
            Print(chr),
            ).unwrap();
    }

    fn render_text(&mut self) {
        let (x, y) = match cursor::position() {
            Ok(v) => (v.0, v.1),
            Err(e) => panic!("Curser Postion ERROR: {}", e),
        };
        queue!(
            self.w,
            cursor::MoveTo(0, y),
            Print(self.e.textbuffer.get_line(y as usize)),
            ).unwrap();
    }

    fn welcome_message(&mut self, width: u16, height: u16) {
        for (y, msg) in WELCOME.split("\n").enumerate() {
            let x = width / 2 - ((msg.len() as u16) / 2);
            let y = height / 3 + y as u16;
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

    fn status_bar_mode(&mut self) {
        let mode = self.e.mode.to_str();
        queue!(
            self.w,
            cursor::SavePosition,
            cursor::MoveTo(0, self.dim.h - 2),
            Clear(ClearType::CurrentLine),
            style::Print(format!("{}, location: {}/{}", mode, self.cursor.x, self.cursor.y)),
            cursor::RestorePosition,
            style::ResetColor
        )
        .unwrap();
    }

    fn message_bar_display(&mut self, y: u16) {
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

    fn clear(&mut self) {
        queue!(self.w, Clear(ClearType::All)).unwrap();
    }
}

impl Screen {
    fn get_terminal_size() -> Dimensions {
        match terminal::size() {
            Ok(v) => Dimensions::new(v.0, v.1),
            Err(e) => panic!("Terminal Size ERROR: {}", e),
        }
    }

    fn get_curser_postion() -> Position {
        match cursor::position() {
            Ok(v) => Position::new(v.0, v.1),
            Err(e) => panic!("Curser Postion ERROR: {}", e),
        }
    }
}





