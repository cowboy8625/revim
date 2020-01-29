use crate::editor::{Editor, ModeSwitch};

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
}

impl Screen {
    pub fn new(w: Stdout, e: Editor) -> Self {
        Self { w, e, }
    }

    pub fn update(&mut self) {
        let (w, h) = Screen::get_terminal_size();
        let (x, y) = Screen::get_curser_postion();
        // self.welcome_message(w, h);
        self.render_text(y);
        self.status_bar_mode(h, x, y);
        if self.e.is_command() {
            self.message_bar_display(h);
        }
        self.w.flush().unwrap();
    }

    fn render_text(&mut self, y: u16) {
        if self.e.textbuffer.dirty {
            queue!(
                self.w,
                cursor::MoveTo(0, y),
                Print(self.e.textbuffer.get_line(y as usize))
                ).unwrap();
        }
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

    pub fn status_bar_mode(&mut self, h: u16, x: u16, y: u16) {
        let mode = self.e.mode.to_str();
        queue!(
            self.w,
            cursor::SavePosition,
            cursor::MoveTo(0, h - 2),
            Clear(ClearType::CurrentLine),
            style::Print(format!("{}, location: {}/{}", mode, x, y)),
            cursor::RestorePosition,
            style::ResetColor
        )
        .unwrap();
    }

    pub fn message_bar_display(&mut self, y: u16) {
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

    pub fn move_up(&mut self) {
        queue!(self.w, cursor::MoveUp(1)).unwrap_or_default();
    }

    pub fn move_down(&mut self) {
        queue!(self.w, cursor::MoveDown(1)).unwrap_or_default();
    }

    pub fn move_left(&mut self) {
        queue!(self.w, cursor::MoveLeft(1)).unwrap_or_default();
    }

    pub fn move_right(&mut self) {
        queue!(self.w, cursor::MoveRight(1)).unwrap_or_default();
    }

    pub fn backspace(&mut self, idx: usize) {
        let idx = if idx > 0 { idx - 1 } else { idx };
        self.e.textbuffer.remove(idx..);
        queue!(
            self.w,
            cursor::MoveLeft(1),
            Print(' '),
            cursor::MoveLeft(1),
        ).unwrap();
    }

    pub fn line_break(&mut self, x: u16, y: u16) {
        self.e.textbuffer.new_line(x, y);
        queue!(self.w, style::Print("\r\n")).unwrap();
    }

    pub fn insert_char(&mut self, x: u16, y: u16, chr: char) {
        self.e.textbuffer.insert_char(x, y, chr);
    }

    pub fn start(&mut self) {
        execute!(
            self.w,
            cursor::Show,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            style::ResetColor,
        ).unwrap();
        let (w, h) = Screen::get_terminal_size();
        for idx in 0..self.e.textbuffer.len_lines() {
            if idx == h as usize { break; }
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

    fn get_terminal_size() -> (u16, u16) {
        let tsize = match terminal::size() {
            Ok(v) => v,
            Err(e) => panic!("Terminal Size ERROR: {}", e),
        };
        tsize
    }

    fn get_curser_postion() -> (u16, u16) {
        let loc = match cursor::position() {
            Ok(v) => v,
            Err(e) => panic!("Curser Postion ERROR: {}", e),
        };
        loc
    }
}

fn editor_alert(w: &mut Stdout, msg: &str) {
    execute!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(0, 0),
        style::Print(msg),
        cursor::RestorePosition,
    )
    .unwrap();
}
