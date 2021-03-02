use std::io::{Stdout, Write};
use crossterm::{queue, cursor, terminal, style};
use crate::{Mode, Editor};

#[derive(Debug)]
pub struct ScreenVector {
    pub x: usize,
    pub y: usize,
    pub max_w: usize,
    pub max_h: usize,
}

impl ScreenVector {
    pub fn new(x: usize, y: usize, max_w: usize, max_h: usize) -> Self {
        Self {
            x, y, max_w, max_h
        }
    }

    pub fn _origin(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn _right(&self) -> usize {
        self.x + self.max_w
    }

    pub fn _left(&self) -> usize {
        self.x
    }

    pub fn _top(&self) -> usize {
        self.y
    }

    pub fn bottom(&self) -> usize {
        self.y + self.max_h
    }
}
pub(crate) fn screen_size() -> ScreenVector {
    let (w, h) = crossterm::terminal::size()
        .expect("Your screen size is messed up and I will fix it later");
    ScreenVector::new(0, 0, w as usize, h as usize)
}

pub(crate) fn render_enter_alt_screen(w: &mut Stdout) {
    terminal::enable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(
        w,
        terminal::EnterAlternateScreen,
    ).expect("something went wrong in render_enter_alt_screen");
}

pub(crate) fn render_exit_alt_screen(w: &mut Stdout) {
    terminal::disable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(
        w,
        terminal::LeaveAlternateScreen,
    ).expect("something went wrong in render_exit_alt_screen");
}

pub(crate) fn render_clear(w: &mut Stdout) {
    queue!(
        w,
        terminal::Clear(terminal::ClearType::All),
    ).expect("something went wrong in render_clear");
}

fn render_text(w: &mut Stdout, editor: &Editor) {
    let screen = &editor.screen;

    'outer: for (y, line) in editor.rope.lines_at(screen.x).enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if y == screen.bottom() - 1{ break 'outer; }
            queue!(
                w,
                cursor::MoveTo(x as u16, y as u16),
                terminal::DisableLineWrap,
                style::Print(chr),
            ).expect("Something went wrong while displaying file text.");
        }
    }

}

fn render_command_bar(w: &mut Stdout, editor: &Editor) {
    if let Mode::Command = editor.mode {
        queue!(
            w,
            cursor::MoveTo(0, 2 + editor.screen.bottom() as u16),
            style::Print(&format!(":{}", editor.command.as_str())),
        ).expect("some crap went wrong, Fix you shit!");
    } else {
        queue!(
            w,
            cursor::MoveTo(0, 2 + editor.screen.bottom() as u16),
            style::Print(&format!("{}", editor.command.as_str())),
        ).expect("some crap went wrong, Fix you shit!");
    }
}

pub(crate) fn render(w: &mut Stdout, editor: &Editor) {
    queue!(
        w,
        cursor::Hide,
    ).expect("Error while trying to hide cursor.");

    render_text(w, editor);
    // render_status_bar(w, &editor);
    render_command_bar(w, &editor);
    // render_line_numbers(&mut writer, &editor);

    queue!(
        w,
        cursor::MoveTo(editor.cursor.0, editor.cursor.1),
        cursor::Show,
    ).expect("Error while trying to show cursor.");
    w.flush().expect("Flush Is BROKEN");
}
