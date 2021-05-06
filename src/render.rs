use crate::{Editor, Mode};
use crossterm::{cursor, queue, style, terminal};
use std::io::{Stdout, Write};

#[derive(Debug)]
pub struct ScreenVector {
    pub t: usize,
    pub b: usize,
    pub max_w: usize,
    pub max_h: usize,
}

impl ScreenVector {
    pub fn new(t: usize, b: usize, max_w: usize, max_h: usize) -> Self {
        Self { t, b, max_w, max_h }
    }

    pub fn _origin(&self) -> (usize, usize) {
        (self.t, self.b)
    }
pub fn _right(&self) -> usize {
        self.t + self.max_w
    }

    pub fn _left(&self) -> usize {
        self.t
    }

    pub fn _top(&self) -> usize {
        self.b
    }

    pub fn bottom(&self) -> usize {
        self.b + self.max_h
    }
}
pub(crate) fn screen_size() -> ScreenVector {
    let (w, h) =
        crossterm::terminal::size().expect("Your screen size is messed up and I will fix it later");
    ScreenVector::new(0, 0, w as usize, (h - 2) as usize)
}

pub(crate) fn render_enter_alt_screen(w: &mut Stdout) {
    terminal::enable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(w, terminal::EnterAlternateScreen,)
        .expect("something went wrong in render_enter_alt_screen");
}

pub(crate) fn render_exit_alt_screen(w: &mut Stdout) {
    terminal::disable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(w, terminal::LeaveAlternateScreen,)
        .expect("something went wrong in render_exit_alt_screen");
}

pub(crate) fn _render_clear(w: &mut Stdout) {
    queue!(w, terminal::Clear(terminal::ClearType::All),)
        .expect("something went wrong in render_clear");
}

fn render_text(w: &mut Stdout, editor: &Editor) {
    let screen = &editor.screen;
    let mut text = String::new();

    for (line_num, line) in editor.rope.lines_at(screen.t).enumerate() {
        if line_num == screen.max_h {break;}
        text.push_str(line.as_str().unwrap_or("\n"))
    }

    format_text(&mut text, editor.screen.max_w, screen.max_h);

    queue!(
        w,
        cursor::MoveTo(0, 0),
        terminal::DisableLineWrap,
        style::Print(text),
    )
    .expect("Something went wrong while displaying file text.");
}

fn render_command_bar(w: &mut Stdout, editor: &Editor) {
    if let Mode::Command = editor.mode {
        queue!(
            w,
            cursor::MoveTo(0, 1 + editor.screen.bottom() as u16),
            style::Print(&format!(":{}", editor.command.as_str())),
        )
        .expect("Command Bar Error 1");
    } else {
        queue!(
            w,
            cursor::MoveTo(0, 1 + editor.screen.bottom() as u16),
            style::Print(&format!("{}", editor.command.as_str())),
        )
        .expect("Command Bar Error 2");
    }
}

fn render_status_bar(w: &mut Stdout, editor: &Editor) {
    queue!(
        w,
        cursor::MoveTo(0, editor.screen.bottom() as u16),
        style::Print(&format!("{}     {} {}               ", editor.mode, editor.cursor, editor.cursor.max_x)),
    )
    .expect("Status Bar Error");
}

pub(crate) fn render(w: &mut Stdout, editor: &Editor) {
    queue!(w, cursor::Hide,).expect("Error while trying to hide cursor.");

    render_text(w, editor);
    render_status_bar(w, &editor);
    render_command_bar(w, &editor);
    // render_line_numbers(&mut writer, &editor);

    queue!(
        w,
        cursor::MoveTo(editor.cursor.x, editor.cursor.y),
        cursor::Show,
    )
    .expect("Error while trying to show cursor.");
    w.flush().expect("Flush Is BROKEN");
}

fn format_text(text: &mut String, width: usize, height: usize) {
    let filler = ' ';
    let mut new = String::new();
    for (y, line) in text.lines().enumerate() {
        let spaces = width.saturating_sub(line.len());
        let blanks = vec![filler; spaces].iter().collect::<String>();
        new.push_str(&line[..line.len().min(width)].replace("\t", "    "));
        new.push_str(&blanks);
        new.push_str("\r\n");
        if y == height - 1 { break; }
    }
    for _ in 0..((height - 1).saturating_sub(new.count_char('\n'))) {
        new.push_str(&vec![filler; width].iter().collect::<String>());
        new.push_str("\r\n");
    }
    *text = new;
}

pub trait StringCount {
    fn count_char(&self, chr: char) -> usize;
}

impl StringCount for String {
    fn count_char(&self, chr: char) -> usize {
        let mut counter = 0;
        for c in self.chars() {
            if c == chr { counter += 1; }
        }
        counter
    }
}

