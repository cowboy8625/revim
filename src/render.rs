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
        text.push_str(&line.chars().collect::<String>())
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
    // TODO: Compact this.
    let dot = if Mode::Command == editor.mode { ":" } else { "" };
    let mut command = format!("{}{}", dot, editor.command.as_str());
    format_command_bar(&mut command, editor.screen.max_w);
    queue!(
        w,
        cursor::Show,
        cursor::MoveTo(0, editor.screen.max_h.saturating_add(1) as u16),
        style::Print(command),
    )
    .expect("Command Bar Error");
}

fn render_status_bar(w: &mut Stdout, editor: &Editor) {
    let width = editor.screen.max_w.saturating_sub(editor.mode.to_string().len() + editor.cursor.to_string().len());
    let space = vec![' '; width].iter().collect::<String>();
    queue!(
        w,
        cursor::MoveTo(0, editor.screen.max_h as u16),
        style::Print(&format!("{}{}{}", editor.mode, space, editor.cursor)),
    )
    .expect("Status Bar Error");
}

fn render_cursor(w: &mut Stdout, editor: &Editor) {
    let x = if editor.mode == Mode::Command { editor.command.len().saturating_add(1) as u16 } else {editor.cursor.x};
    let y = if editor.mode == Mode::Command { (1 + editor.screen.bottom()) as u16 } else {editor.cursor.y};
    queue!(
        w,
        cursor::Show,
        cursor::MoveTo(x, y),
    ).expect("Error while rendering cursor");
}

fn render_error_message(w: &mut Stdout, editor: &Editor) {
    let x = 0;
    let y = (editor.screen.bottom().saturating_add(2)) as u16;
    queue!(
        w,
        cursor::MoveTo(x, y),
        style::Print(style::style(&editor.error).on(style::Color::DarkRed)),
    ).expect("Error while rendering cursor");
}

fn render_output(w: &mut Stdout, editor: &Editor) {
    let x = 0;
    let y = (1 + editor.screen.bottom()) as u16;
    queue!(
        w,
        cursor::MoveTo(x, y),
        style::Print(&editor.output),
    ).expect("Error while rendering cursor");
}

pub(crate) fn render(w: &mut Stdout, editor: &Editor) {
    queue!(w, cursor::Hide,).expect("Error while trying to hide cursor.");

    render_text(w, editor);
    render_status_bar(w, &editor);
    render_command_bar(w, &editor);
    // render_line_numbers(&mut writer, &editor);
    render_error_message(w, &editor);
    render_output(w, &editor);
    render_cursor(w, &editor);

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
    for _ in 0..(height.saturating_sub(new.count_char('\n'))) {
        new.push_str(&vec![filler; width].iter().collect::<String>());
        new.push_str("\r\n");
    }
    *text = new;
}

fn format_command_bar(line: &mut String, length: usize) {
    let filler = ' ';
    let spaces = length.saturating_sub(line.len());
    let blanks = vec![filler; spaces].iter().collect::<String>();
    line.push_str(&blanks);
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

