use crossterm::{
    cursor::{position, MoveTo},
    event::{read, Event, KeyCode},
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};

use std::env;
use std::io::{BufReader, BufWriter, stdout, Write};
use terminal_size::{terminal_size, Height, Width};
use std::fs::File;
use ropey::Rope;

/*** data ***/

struct WinDim{
    top: i16,
    bottom: i16,
    left: i16,
    right: i16,
}

impl WinDim {
    fn new(top: i16, bottom: i16, left: i16, right: i16) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn push_left(&mut self) {
        self.left -= 3;
    }

    fn pull_left(&mut self) {
        self.left += 3;
    }
}

struct Vector {
    x: i16,
    y: i16,
}

impl Vector {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

struct Screen {
    c_pos: Vector,
    c_color: Color,
    c_blink: bool,
    line_num: bool,
    debug: bool,
    text: Option<Rope>,
    terminal_size: Vector,
    win_dim: WinDim,
    version: String,
    bg: Color,
}

impl Screen {
    fn new(c_pos: Vector, c_color: Color, c_blink: bool, text: Option<Rope>)
    -> Self {

        let (width, height) = get_terminal_size();
        Self {
            c_pos,
            c_color,
            c_blink,
            line_num: false,
            debug: false,
            text,
            terminal_size: Vector::new(width as i16, height as i16),
            win_dim: WinDim::new(0, height as i16, 0, width as i16),
            version: String::from("0.0.1"),
            bg: Color::Black,
        }
    }

    fn step(&mut self, x: i16, y: i16) {
        self.c_pos.x += x;
        self.c_pos.y += y;
    }

    fn toggle_line_num(&mut self) {
        if self.line_num {
            self.line_num = false;
            self.win_dim.push_left();
            self.c_pos.x -= 3;
        } else {
            self.line_num = true;
            self.win_dim.pull_left();
            self.c_pos.x += 3;
        };
    }

    fn display_welcome(&self) {
        // x, y for the message start.  Needs Center - text size.
        let msg = format!("ReVim Version {}", self.version);
        let x = self.terminal_size.x / 2 - (msg.len() as i16);
        let y = self.terminal_size.y / 3;
        queue!(
            stdout(),
            MoveTo(x as u16, y as u16),
            SetForegroundColor(Color::Cyan),
            SetBackgroundColor(self.bg),
            Print(msg),
            ResetColor
        )
        .unwrap();
    }

    fn display_numbers(&self, mut start: u16, height: u16, fg: Color, bg: Color) {
        for y in 0..height {
            queue!(
                stdout(),
                MoveTo(0, y),
                SetForegroundColor(fg),
                SetBackgroundColor(bg),
                Print(start),
                ResetColor
            )
            .unwrap();
            start += 1;
        }
    }

    fn display_text(&self) {
        if let Some(x) = &self.text {

            queue!(
                stdout(),
                MoveTo(self.win_dim.left as u16, self.win_dim.top as u16),
                Print(x.line(0)),
                ResetColor
            ).unwrap();
    /*
            for y in 0..x.len_lines() {
                //Keep Render on screen.
                if editor.terminal_size.y {
            }
    */
        }
    }

    fn display_refresh(&self) -> Result<()> {
        clear()?;

        if let None = &self.text {
            self.display_welcome();
        }
        if self.line_num {
            self.display_numbers(
                1, self.win_dim.bottom as u16, Color::Green, Color::Black
                );
        }
        if self.debug {
            self.debug_display();
        }
        self.display_text();
        queue!(
            stdout(),
            MoveTo(self.c_pos.x as u16, self.c_pos.y as u16),
            ResetColor
        )
        .unwrap();
        stdout().flush();
        Ok(())
    }

    fn debug_display(&self) {
        queue!(
            stdout(),
            MoveTo(self.win_dim.left as u16, self.win_dim.top as u16),
            Print(
                format!(
                    "X: {}, Y: {} | LEFT: {}, RIGHT: {}, TOP: {}, BOTTOM: {}",
                    self.c_pos.x,
                    self.c_pos.y,
                    self.win_dim.left,
                    self.win_dim.right,
                    self.win_dim.top,
                    self.win_dim.bottom,
                    )
                ),
            ResetColor
        ).unwrap();
    }
}

/*** input ***/

fn editor_key_event(e: &mut Screen) -> (bool, bool) {
    if let Ok(event) = read() {
        if let Event::Key(key) = event {
          match key.code {
                KeyCode::Char('j') => {
                    if e.c_pos.y < e.win_dim.bottom {
                        e.step(0, 1);
                        return (true, false);
                    }
                } KeyCode::Char('k') => {
                    if e.c_pos.y > e.win_dim.top {
                        e.step(0, -1);
                        return (true, false);
                    }
                } KeyCode::Char('h') => {
                    if e.c_pos.x > e.win_dim.left {
                        e.step(-1, 0);
                        return (true, false);
                    }
                } KeyCode::Char('l') => {
                    if e.c_pos.x < e.win_dim.right {
                        e.step(1, 0);
                        return (true, false);
                    }
                } KeyCode::Char('n') => {
                    e.toggle_line_num();
                    return (true, false);
                } KeyCode::Char('d') => {
                    e.debug = if e.debug { false } else { true };
                    return (true, false);
                }
                KeyCode::Esc => return (true, true),
                _ => {},
            }
        }
    }
    (false, false)
}

/*** terminal ***/

fn clear() -> Result<()> {
    queue!(
        stdout(),
        Clear(ClearType::All),
        ResetColor
    ).unwrap();
    stdout().flush();
    Ok(())
}

fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        (20, 20)
    }
}

/*** init ***/

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut args = env::args();

    enable_raw_mode()?;

    let (width, height) = get_terminal_size();

    let mut text = None;
    let _ = args.next();
    if let Some(arg) = args.next() {
        text = Some(Rope::from_reader(
            BufReader::new(File::open(&arg)?)
        )?);
    }

    let mut editor = Screen::new(
        Vector::new(0, 0),
        Color::White,
        true,
        text
        );

    clear()?;
    let update = (true, false);

    loop {
        if update.0 { editor.display_refresh(); }

        let update = editor_key_event(&mut editor);

        if update.1 { break }
    }
    disable_raw_mode()
}

