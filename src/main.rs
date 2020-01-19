/***   ReVim "Rust Edition Vim"   ***/
use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::position,
    cursor,
    terminal,
    style,
    event::{poll, read, Event, KeyCode},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

/***  constants  ***/
const WELCOME: &str = r#"ReVim - Rust Edition Vim - Version 0.0.1
To Quit: :q, To INSERT Text: i, For Help: :help <key or keyword>
GitHub: https://www.github.com/cowboy8625/ReVim"#;
/***  data  ***/

enum Mode {
    NORMAL,
    COMMAND,
}

struct Editor {
    mode: Mode,
}

/***  terminal   ***/

/***  output   ***/


fn debug_display(w: &mut Stdout, e: &Editor) {
    let pos = match cursor::position() {
        Ok(v) => v,
        Err(e) => panic!("CURSOR ERROR"),
    };
    queue!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(0, 0),
        style::Print(format!("X: {}, Y {}", pos.0, pos.1)),
        cursor::RestorePosition,
    ).unwrap();
}

/***  input ***/

fn input_command_mode(w: &mut Stdout, k: KeyCode) -> bool {
    return false;
}

fn input_normal_mode(w: &mut Stdout, k: KeyCode) -> bool {
    match k {
        KeyCode::Char('j') => {
            queue!(w, cursor::MoveDown(1)).unwrap_or_default();
        } KeyCode::Char('k') => {
            queue!(w, cursor::MoveUp(1)).unwrap_or_default();
        } KeyCode::Char('h') => {
            queue!(w, cursor::MoveLeft(1)).unwrap_or_default();
        } KeyCode::Char('l') => {
            queue!(w, cursor::MoveRight(1)).unwrap_or_default();
        }
        _ => {},
    }
    false
}

fn input_events(w: &mut Stdout, e: &Editor) -> bool {
    loop {
        if let Ok(event) = read() {
            if let Event::Key(key) = event {
                // Once Command mode works i can remove this if statement
                if key.code == KeyCode::Esc {
                    return true;
                }

                match e.mode {
                    Mode::NORMAL => return input_normal_mode(w, key.code),
                    Mode::COMMAND => return input_command_mode(w, key.code),
                    _ => {},
                }
            }
        }
    }
}

/*** init ***/

fn main() -> Result<()> {
    let editor: Editor = Editor { mode: Mode::NORMAL };

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::Show,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0,0)
        )?;

    loop {
        if poll(Duration::from_millis(10))? {
            if input_events(&mut stdout, &editor) {
                break;
            }
        }
        debug_display(&mut stdout, &editor);
        stdout.flush()?;
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;

    disable_raw_mode()
}
