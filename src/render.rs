use std::io::{Stdout, Write};
use crossterm::{queue, cursor, terminal, style};
use crate::Editor;

pub(crate) fn render_enter_alt_screen(w: &mut Stdout) {
    terminal::enable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(
        w,
        terminal::EnterAlternateScreen,
    ).expect("lksdjfksajdkj");
}

pub(crate) fn render_exit_alt_screen(w: &mut Stdout) {
    terminal::disable_raw_mode().expect("Exit raw mode bad thing happened");
    queue!(
        w,
        terminal::LeaveAlternateScreen,
    ).expect("lksdjfksajdkj");
}

pub(crate) fn render_clear(w: &mut Stdout) {
    queue!(
        w,
        terminal::Clear(terminal::ClearType::All),
    ).expect("lksdjfksajdkj");
}

pub(crate) fn render(w: &mut Stdout, editor: &Editor) {
    let (h, max_h) = editor.screen;
    queue!(
        w,
        cursor::Hide,
    ).expect("lksdjfksajdkj");

    'outer: for (y, line) in editor.rope.lines_at(editor.screen.0).enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if y == h + max_h - 1{ break 'outer; }
            queue!(
                w,
                cursor::MoveTo(x as u16, y as u16),
                terminal::DisableLineWrap,
                style::Print(chr),
            ).expect("some crap went wrong, Fix you shit!");
        }
    }

    queue!(
        w,
        cursor::MoveTo(editor.cursor.0, editor.cursor.1),
        cursor::Show,
    ).expect("lksdjfksajdkj");
    w.flush().expect("Flush Is BROKEN");
}
