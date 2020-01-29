/***   ReVim "Rust Edition Vim"   ***/

mod editor;
mod screen;
mod textbuffer;

use std::env;


use std::{io::stdout, time::Duration};
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
// use ropey::Rope;
use editor::{Editor, ModeSwitch, Mode};
use screen::Screen;
use textbuffer::TextBuffer;

fn input_command_mode(s: &mut Screen, k: KeyEvent) {
    //KeyEvent
    match k {
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
        } => {
                //TODO fix this!
            s.e.current_command.pop();
        }
        KeyEvent { code, .. } => match code {
            KeyCode::Enter => {
                s.e.run_command();
                //TODO fix this!
                s.e.current_command.clear();
                s.e.normal_mode();
            }
            KeyCode::Char(c) => {
                //TODO fix this!
                s.e.current_command.push(c);
            }
            _ => {}
        },
    }
}

fn input_normal_mode(s: &mut Screen, k: KeyEvent) {
    match k {
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        } => {}
        KeyEvent { code, .. } => match code {
            KeyCode::Char(':') => {
                s.e.command_mode();
            }
            KeyCode::Char('i') => {
                s.e.insert_mode();
            }
            KeyCode::Char('j') => s.move_down(),
            KeyCode::Char('k') => s.move_up(),
            KeyCode::Char('h') => s.move_left(),
            KeyCode::Char('l') => s.move_right(),
            _ => {}
        },
    }
}

fn input_insert_mode(s: &mut Screen, k: KeyEvent) {
    let pos = match cursor::position() {
        Ok(v) => v,
        Err(e) => panic!("CURSOR ERROR: {}.", e),
    };
    let tsize = match terminal::size() {
        Ok(v) => v,
        Err(e) => panic!("Terminal Size ERRO: {}.", e),
    };
    let idx: usize = (pos.1 * tsize.0 + pos.0) as usize;
    match k {
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            // TODO Fix backspace/delete
            s.backspace(idx);
        }
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        } => s.e.normal_mode(),
        KeyEvent { code, .. } => {
            match code {
                KeyCode::Enter => {
                    // new line
                    s.line_break(pos.0, pos.1);
                }
                KeyCode::Char(c) => {
                    // append to text file
                    s.insert_char(pos.0, pos.1, c);
                }
                _ => {}
            }
        }
    }
}

fn input_events(s: &mut Screen) {
    match read() {
        Ok(Event::Key(key)) => {
            // Once Command mode works i can remove this if statement
            if key.code == KeyCode::Esc {
                s.e.quit();
            }

            match s.e.mode {
                Mode::NORMAL => input_normal_mode(s, key),
                Mode::COMMAND => input_command_mode(s, key),
                Mode::INSERT => input_insert_mode(s, key),
            }
        }
        Ok(Event::Resize(x, y)) => s.resize(x, y),
        Ok(Event::Mouse(_)) => {},
        Err(e) => panic!("This wont run minecraft: {}", e),
    }
}

/*** init ***/


fn main() -> Result<()> {
    let mut args = env::args();
    let file_name = args.nth(1);
    enable_raw_mode()?;

    let stdout = stdout();

    let editor: Editor = Editor::new(Mode::NORMAL, file_name);
    let mut screen: Screen = Screen::new(stdout, editor);

    screen.start();

    loop {
        screen.update();
        if poll(Duration::from_millis(10))? {
            input_events(&mut screen);
            if screen.e.quit {
                break;
            }
        }
    }

    screen.end();

    disable_raw_mode()
}

