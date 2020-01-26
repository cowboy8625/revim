/***   ReVim "Rust Edition Vim"   ***/
use std::env;
use std::fs::File;
use std::io::ErrorKind;

use std::{
    io::{stdout, BufReader, BufWriter, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style, terminal,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};

use ropey::Rope;

const WELCOME: &str = r#"ReVim - Rust Edition Vim - Version 0.0.1
To Quit: :q, To INSERT Text: i, For Help: :help <key or keyword>
GitHub: https://www.github.com/cowboy8625/ReVim"#;

/***  traits   ***/

/***  data  ***/

enum Mode {
    NORMAL,
    COMMAND,
    INSERT,
}

enum CommandType {
    Quit,
    Save,
    Empty,
}

impl CommandType {
    fn new(s: &str) -> Self {
        match s {
            ":q" => Self::Quit,
            ":w" => Self::Save,
            _ => Self::Empty,
        }
    }
}

struct Editor {
    mode: Mode,
    current_command: Vec<char>,
    file_text: Option<Rope>,
    file_name: Option<String>,
    quit: bool,
}

impl Editor {
    fn new(mode: Mode, file_text: Option<Rope>, file_name: Option<String>) -> Self {
        Self {
            mode,
            current_command: Vec::new(),
            file_text,
            file_name,
            quit: false,
        }
    }

    fn save(&mut self, name: Option<&str>) {
        let name = &self
            .file_name
            .clone()
            .unwrap_or_else(|| match &self.file_name {
                Some(f) => {
                    if f == "new_file.txt" {
                        name.unwrap_or("new_text.txt").to_string()
                    } else {
                        name.unwrap_or("new_text.txt").to_string()
                    }
                }
                None => name.unwrap_or("new_text.txt").to_string()
            });
        match &mut self.file_text {
            Some(f) => {
                f.write_to(BufWriter::new(
                    File::create(name).expect("File Creation Error in save_file"),
                ))
                .expect("BufWriter Error: save_file.");
            }
            None => {
                panic!("NO FILE");
            }
        };
    }
}

/***  output   ***/

fn update_display(w: &mut Stdout, e: &Editor) {
    let tsize = match terminal::size() {
        Ok(v) => v,
        Err(e) => panic!("Terminal Size ERROR: {}", e),
    };
    if let Some(f) = &e.file_text {
        welcome_message(w, tsize.0, tsize.1);
    }
    if let Mode::COMMAND = e.mode {
        message_bar_display(w, e, tsize);
    }
    status_bar_mode(w, e, tsize.0, tsize.1);
}

fn status_bar_mode(w: &mut Stdout, e: &Editor, _x: u16, y: u16) {
    let mode = match e.mode {
        Mode::NORMAL => "Normal",
        Mode::COMMAND => "Command",
        Mode::INSERT => "--insert--",
    };
    queue!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(0, y - 2),
        style::Print(format!("{}, location: {}/{}", mode, 0, y - 2)),
        cursor::RestorePosition,
        style::ResetColor
    )
    .unwrap();
}

fn welcome_message(w: &mut Stdout, width: u16, height: u16) {
    // Message Bar Displays Commands, Invalid Commands, Errors, Warnings.
    for (y, msg) in WELCOME.split("\n").enumerate() {
        let x = width / 2 - ((msg.len() as u16) / 2);
        let y = height / 3 + y as u16;
        queue!(
            w,
            cursor::SavePosition,
            cursor::MoveTo(x, y),
            style::Print(msg),
            cursor::RestorePosition,
        )
        .unwrap();
    }
}

fn message_bar_display(w: &mut Stdout, e: &Editor, tsize: (u16, u16)) {
    queue!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(0, tsize.1),
        Clear(ClearType::CurrentLine),
        style::Print(format!(
            "{}",
            e.current_command.iter().map(|c| c).collect::<String>()
        )),
        cursor::RestorePosition,
    )
    .unwrap();
}

fn debug_display(w: &mut Stdout, _e: &Editor) {
    let tsize = match terminal::size() {
        Ok(v) => v,
        Err(e) => panic!("Terminal Size ERROR: {}.", e),
    };
    let x = tsize.0 / 2;
    let y = tsize.1 - 2;
    queue!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(x, y),
        Clear(ClearType::CurrentLine),
        style::Print(format!("No Test")),
        cursor::RestorePosition,
    )
    .unwrap();
}

/***  input ***/

fn run_command(e: &mut Editor) {
    let args: Vec<&str>;
    let com: CommandType;
    let mut name: Option<&str> = None;
    let command: String = e.current_command.iter().collect::<String>();
    if command.contains(" ") {
        args = command.split(' ').collect();
        com = CommandType::new(args[0]);
        name = Some(args[1]);
    } else {
        com = CommandType::new(command.as_str())
    }
    match com {
        CommandType::Quit => e.quit = true,
        CommandType::Save => {
            e.save(name);
        }
        CommandType::Empty => {}
    }
}

fn input_command_mode(_w: &mut Stdout, k: KeyEvent, e: &mut Editor) {
    //KeyEvent
    match k {
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            e.current_command.pop();
        }
        KeyEvent { code, .. } => match code {
            KeyCode::Enter => {
                run_command(e);
                e.current_command.clear();
                e.mode = Mode::NORMAL;
            }
            KeyCode::Char(c) => {
                e.current_command.push(c);
            }
            _ => {}
        },
    }
}

fn input_normal_mode(w: &mut Stdout, k: KeyEvent, e: &mut Editor) {
    match k {
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        } => {}
        KeyEvent { code, .. } => match code {
            KeyCode::Char(':') => {
                e.mode = Mode::COMMAND;
                e.current_command.push(':');
            }
            KeyCode::Char('i') => {
                e.mode = Mode::INSERT;
            }
            KeyCode::Char('j') => {
                queue!(w, cursor::MoveDown(1)).unwrap_or_default();
            }
            KeyCode::Char('k') => {
                queue!(w, cursor::MoveUp(1)).unwrap_or_default();
            }
            KeyCode::Char('h') => {
                queue!(w, cursor::MoveLeft(1)).unwrap_or_default();
            }
            KeyCode::Char('l') => {
                queue!(w, cursor::MoveRight(1)).unwrap_or_default();
            }
            _ => {}
        },
    }
}

fn input_insert_mode(w: &mut Stdout, k: KeyEvent, e: &mut Editor) {
    let pos = match cursor::position() {
        Ok(v) => v,
        Err(e) => panic!("CURSOR ERROR: {}.", e),
    };
    let tsize = match terminal::size() {
        Ok(v) => v,
        Err(e) => panic!("Terminal Size ERRO: {}.", e),
    };
    let idx: usize = (pos.1 * tsize.0 + pos.0) as usize;

    // if let None = e.file_text {
    // e.save(Some("CRASH.txt"));
    // }
    queue!(
        w,
        cursor::SavePosition,
        cursor::MoveTo(50, 30),
        style::Print(format!(
            "X: {}, Y: {}, width: {}, height: {}",
            pos.0, pos.1, tsize.0, tsize.1
        )),
        cursor::RestorePosition,
    )
    .unwrap();
    match k {
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            if let Some(ft) = &mut e.file_text {
                ft.remove(idx..idx);
            }
            queue!(
                w,
                cursor::MoveLeft(1),
                style::Print(" "),
                cursor::MoveLeft(1),
            )
            .unwrap();
        }
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        } => e.mode = Mode::NORMAL,
        KeyEvent { code, .. } => {
            match code {
                KeyCode::Enter => {
                    // new line
                    if let Some(ft) = &mut e.file_text {
                        ft.insert_char(pos.1 as usize, '\n')
                    }
                    queue!(w, style::Print("\r\n"),).unwrap();
                }
                KeyCode::Char(c) => {
                    // append to text file
                    if let Some(ft) = &mut e.file_text {
                        ft.insert_char(pos.1 as usize, c)
                    }
                    queue!(w, style::Print(c),).unwrap();
                }
                _ => {}
            }
        }
    }
}

fn input_events(w: &mut Stdout, e: &mut Editor) {
    if let Ok(event) = read() {
        if let Event::Key(key) = event {
            // Once Command mode works i can remove this if statement
            if key.code == KeyCode::Esc {
                e.quit = true;
            }

            match e.mode {
                Mode::NORMAL => input_normal_mode(w, key, e),
                Mode::COMMAND => input_command_mode(w, key, e),
                Mode::INSERT => input_insert_mode(w, key, e),
            }
        }
    }
}

/*** init ***/

fn editor_alert(msg: &str) {
    execute!(
        stdout(),
        cursor::SavePosition,
        cursor::MoveTo(0, 0),
        style::Print(msg),
        cursor::RestorePosition,
    )
    .unwrap();
}

fn cli_file() -> (Option<Rope>, Option<String>) {
    let mut args = env::args();
    let name = args.nth(2);
    let rope: Rope = match &name {
        Some(n) => {
            Rope::from_reader(BufReader::new(File::open(&n).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create(n).unwrap_or_else(|error| {
                        panic!("Problem creating the file: {:?}", error);
                    })
                } else {
                    panic!("Problem opening the file: {:?}", error);
                }
            })))
            .unwrap()
        },
        None => Rope::new(),
    };
    (Some(rope), name)
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::Show,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0)
    )?;
    let (file_text, file_name) = cli_file();
    let mut editor: Editor = Editor::new(Mode::NORMAL, file_text, file_name);
    update_display(&mut stdout, &editor);
    stdout.flush();

    loop {
        if poll(Duration::from_millis(10))? {
            input_events(&mut stdout, &mut editor);
            if editor.quit {
                break;
            }
            update_display(&mut stdout, &editor);
        }
        // debug_display(&mut stdout, &editor);
        stdout.flush()?;
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;

    disable_raw_mode()
}

/*
#[derive(Debug)]
enum SomeErr {
    MissingArg,
    Io(std::io::Error),
}

impl From<std::io::Error> for SomeErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
fn cli_file() -> Result<(Rope, String), SomeErr> {
    let mut args = env::args();
    let _ = args.next();

    let name = args.next().ok_or(SomeErr::MissingArg)?;
    let rope = Rope::from_reader(BufReader::new(File::open(&name)?))?;

    Ok((rope, name))
}
 */
