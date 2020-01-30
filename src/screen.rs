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
        self.render_text(self.cursor.y);
        self.status_bar_mode(self.dim.h, self.cursor.x, self.cursor.y);
        if self.e.is_command() {
            self.message_bar_display(self.dim.h);
        }
        self.w.flush().unwrap();
    }


    pub fn move_up(&mut self) {
        queue!(self.w, cursor::MoveUp(1)).unwrap_or_default();
        self.cursor.y -= 1;
    }

    pub fn move_down(&mut self) {
        queue!(self.w, cursor::MoveDown(1)).unwrap_or_default();
        self.cursor.y += 1;
    }

    pub fn move_left(&mut self) {
        queue!(self.w, cursor::MoveLeft(1)).unwrap_or_default();
        self.cursor.x -= 1;
    }

    pub fn move_right(&mut self) {
        queue!(self.w, cursor::MoveRight(1)).unwrap_or_default();
        self.cursor.x += 1;
    }

    pub fn backspace(&mut self) {
        // self.buffer.remove(self.cursor_index - 1..self.cursor_index);

        let idx = self.cursor.to_index(self.dim.w) as usize;
        self.e.textbuffer.remove(idx);
        self.cursor.x -= 1;
/*
        queue!(
            self.w,
            cursor::MoveLeft(1),
            Print(' '),
            cursor::MoveLeft(1),
        ).unwrap();
*/
    }

    pub fn line_break(&mut self) {
        self.e.textbuffer.new_line(self.cursor.x, self.cursor.y);
        self.cursor.x = 0;
        self.cursor.y += 1;
        queue!(self.w, style::Print("\r")).unwrap();
    }

    pub fn insert_char(&mut self, chr: char) {
        self.e.textbuffer.insert_char(self.cursor.x, self.cursor.y, chr);
        self.cursor.x += 1;
    }

    pub fn start(&mut self) {
        execute!(
            self.w,
            cursor::Show,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            style::ResetColor,
        ).unwrap();
        // TODO remove this and store width, height, loc_x and loc_y in Screen struct.
        // This is causeing lag.
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


}

/*** private ***/

impl Screen {
    fn render_line(&mut self, y: u16, line: &str) {
        queue!(
            self.w,
            cursor::MoveTo(0, y),
            Print(line)
            ).unwrap();
    }

    fn render_char(&mut self, x: u16, y: u16, chr: char) {
        queue!(
            self.w,
            cursor::MoveTo(0, y),
            Print(chr)
            ).unwrap();
    }

    fn render_text(&mut self, y: u16) {
        queue!(
            self.w,
            cursor::MoveTo(0, y),
            Print(self.e.textbuffer.get_line(y as usize))
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

    fn status_bar_mode(&mut self, h: u16, x: u16, y: u16) {
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
