use crossterm::{execute, queue};
use std::io::{Stdout, Write};
pub type Result<T> = std::result::Result<T, crossterm::ErrorKind>;

pub fn read() -> Result<Event> {
    match crossterm::event::read() {
        Ok(r) => Ok(r.into()),
        Err(e) => Err(e),
    }
}

pub fn move_to(stdout: &mut Stdout, loc: (u16, u16)) -> Result<()> {
    queue!(stdout, crossterm::cursor::MoveTo(loc.0, loc.1))?;
    Ok(())
}

pub fn enter_raw_mode(stdout: &mut Stdout) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Show,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::style::ResetColor,
    )?;
    Ok(())
}

pub fn exit_raw_mode(stdout: &mut Stdout) -> Result<()> {
    execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::style::ResetColor,
    )?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

pub fn terminal_size() -> Result<(u16, u16)> {
    Ok(crossterm::terminal::size()?)
}

pub fn render(stdout: &mut Stdout, width: &u16, grid: &[char], queued: &Vec<usize>) -> Result<()> {
    let mut slice: String;
    queue!(
        stdout,
        crossterm::cursor::Hide,
        crossterm::cursor::SavePosition
    )?;
    for line_num in queued {
        slice = grid[*line_num..*width as usize + line_num]
            .into_iter()
            .collect();
        queue!(
            stdout,
            crossterm::cursor::MoveTo(0, *line_num as u16 / width),
            crossterm::style::Print(slice)
        )?;
    }
    queue!(
        stdout,
        crossterm::cursor::Show,
        crossterm::cursor::RestorePosition
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn poll(timeout: std::time::Duration) -> Result<bool> {
    Ok(crossterm::event::poll(timeout)?)
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum EditorEvent {
    // Cursor Movement,
    Cursor(Direction),
    // Scroll Take 2 Direction's (Scroll Direction, Cursor Direction)
    Scroll(Direction, Direction),
    // Change Modes,
    ModeNormal,
    ModeCommand,
    ModeInsert,
    // Exit Program,
    Quit,
}

pub enum Event {
    Key(KeyEvent),
    Mouse, //(MouseEvent), TODO: This Needs to hold a Mouse Event
    Resize(u16, u16),
}

impl From<crossterm::event::Event> for Event {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(k) => Event::Key(k.into()),
            crossterm::event::Event::Mouse(_) => Event::Mouse,
            crossterm::event::Event::Resize(w, h) => Event::Resize(w, h),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifier: KeyModifier,
}

impl KeyEvent {
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Self {
        Self { code, modifier }
    }
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(key: crossterm::event::KeyEvent) -> Self {
        let code = match key.code {
            crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
            crossterm::event::KeyCode::Enter => KeyCode::Enter,
            crossterm::event::KeyCode::Left => KeyCode::Left,
            crossterm::event::KeyCode::Right => KeyCode::Right,
            crossterm::event::KeyCode::Up => KeyCode::Up,
            crossterm::event::KeyCode::Down => KeyCode::Down,
            crossterm::event::KeyCode::Home => KeyCode::Home,
            crossterm::event::KeyCode::End => KeyCode::End,
            crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
            crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
            crossterm::event::KeyCode::Tab => KeyCode::Tab,
            crossterm::event::KeyCode::BackTab => KeyCode::BackTab,
            crossterm::event::KeyCode::Delete => KeyCode::Delete,
            crossterm::event::KeyCode::Insert => KeyCode::Insert,
            crossterm::event::KeyCode::F(n) => KeyCode::F(n),
            crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
            crossterm::event::KeyCode::Null => KeyCode::Null,
            crossterm::event::KeyCode::Esc => KeyCode::Esc,
        };

        let modifier = match key.modifiers {
            crossterm::event::KeyModifiers::CONTROL => KeyModifier::Control,
            crossterm::event::KeyModifiers::ALT => KeyModifier::Alt,
            crossterm::event::KeyModifiers::SHIFT => KeyModifier::Shift,
            crossterm::event::KeyModifiers::NONE => KeyModifier::NONE,
            _ => KeyModifier::NONE,
        };

        Self { code, modifier }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

#[derive(Eq, PartialEq, Hash)]
pub enum KeyModifier {
    Shift,
    Alt,
    Control,
    NONE,
}
