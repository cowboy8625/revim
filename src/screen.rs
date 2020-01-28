use crate::editor::{Editor, ModeSwitch};

use std::io::{BufReader, BufWriter, Stdout, Write};

use crossterm::{
    cursor,
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
        let (x, y) = Screen::get_terminal_size();
        self.welcome_message(x, y);
        self.status_bar_mode(y);
        if self.e.is_command() {
            self.message_bar_display(y);
        }
        self.w.flush().unwrap();
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

    pub fn status_bar_mode(&mut self, y: u16) {
        let mode = self.e.mode.to_str();
        queue!(
            self.w,
            cursor::SavePosition,
            cursor::MoveTo(0, y - 2),
            style::Print(format!("{}, location: {}/{}", mode, 0, y - 2)),
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

    pub fn backspace(&mut self) {
        queue!(
            self.w,
            cursor::MoveLeft(1),
            style::Print(" "),
            cursor::MoveLeft(1),
        ).unwrap();
    }

    pub fn line_break(&mut self) {
        queue!(self.w, style::Print("\r\n")).unwrap();
    }

    pub fn insert_char(&mut self, chr: char) {
        queue!(self.w, style::Print(chr),).unwrap();
    }

    pub fn start(&mut self) {
        execute!(
            self.w,
            cursor::Show,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0)
        ).unwrap();
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
