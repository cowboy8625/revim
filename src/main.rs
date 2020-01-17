use crossterm::{
    cursor::{MoveTo},
    event::{read, Event, KeyCode},
    queue,
    style::Color::AnsiValue,
    style::{
        Color, Print, ResetColor, SetBackgroundColor,
        SetForegroundColor, Attribute, Styler, StyledContent
    },
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};

use std::env;
use std::io::{BufReader, stdout, Write};
use terminal_size::{terminal_size, Height, Width};
use std::fs::File;
use ropey::Rope;


/*** data ***/

#[allow(dead_code)]
enum Mode {
    COMMAND,
    NORMAL,
    INSERT,
    SEARCH,
}

struct BfgColor { fg: Color, bg: Color }

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

    fn push_down(&mut self) {
        self.left += 3;
    }

    fn pull_down(&mut self) {
        self.left -= 3;
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

#[allow(dead_code)]
struct StatusBar {
    loc: Vector,
    loc_mode: Vector,
    loc_ft: Vector,
    loc_loc: Vector,
    status_imode: BfgColor,
    status_nmode: BfgColor,
    status_cmode: BfgColor,
    status_loc: BfgColor,
    status_ft: BfgColor,
}

#[allow(dead_code)]
impl StatusBar {
    fn new(x: i16, y: i16) -> Self {
        Self {
            /*** Location ***/
            loc: Vector::new(x, y),

            /*** Location and color of curser ***/
            loc_loc: Vector::new(x, y),
            status_loc: BfgColor{fg: Color::Black, bg: Color::Yellow},

            /*** Location and color of Modes ***/
            loc_mode: Vector::new(0, y),
            status_imode: BfgColor{fg: Color::Black, bg: Color::Cyan},
            status_nmode: BfgColor{fg: Color::Black, bg: Color::Yellow},
            status_cmode: BfgColor{fg: Color::Black, bg: Color::Magenta},

            /*** Location and color of File Type ***/
            loc_ft: Vector::new(x, y),
            status_ft: BfgColor{fg: Color::Cyan, bg: Color::DarkGrey},
        }
    }
    fn render_status_bar(&self, mode: &Mode) {
        self.status_mode(mode);
        //self.status_loc(mode);
        //self.status_file_type(mode);
    }

    fn status_mode(&self, mode: &Mode) {
        // Need to figure out the x, and y.
        // if someone is watching IM SO SORRY FOR THE HORRID CODE. :(
        let (current, color): (&str, &BfgColor) = match mode {
            Mode::COMMAND => (" COMMAND ", &self.status_cmode),
            Mode::NORMAL => (" NORMAL ", &self.status_nmode),
            Mode::INSERT => (" --insert-- ", &self.status_imode),
            _ => ("NORMAL", &self.status_nmode),
        };
        queue!(
            stdout(),
            MoveTo(self.loc_mode.x as u16, self.loc_mode.y as u16),
            SetForegroundColor(color.fg),
            SetBackgroundColor(AnsiValue(190)),
            Print(current.bold()),
            ResetColor
        )
        .unwrap();
    }

    fn status_loc(&self) {
        queue!(
            stdout(),
            MoveTo(self.loc_loc.x as u16, self.loc_loc.y as u16),
            SetForegroundColor(color.fg),
            SetBackgroundColor(AnsiValue(190)),
            Print(current.bold()),
            ResetColor
        )
        .unwrap();
    }

    fn status_file_type(&self) {
    }
}

#[allow(dead_code)]
struct Screen {
    c_pos: Vector,
    s_pos: Vector,
    line_num: bool,
    debug: bool,
    refeash: bool,
    text: Option<Rope>,
    terminal_size: Vector,
    win_dim: WinDim,
    version: String,
    status_bar: StatusBar,
    bg: Color,                 // remove
    mode: Mode,
}

impl Screen {
    fn new(c_pos: Vector, text: Option<Rope>)
    -> Self {

        let (width, height) = get_terminal_size();
        Self {
            c_pos,
            s_pos: Vector::new(0, (height - 3) as i16),
            line_num: false,
            debug: false,
            refeash: false,
            text,
            terminal_size: Vector::new(width as i16, height as i16),
            win_dim: WinDim::new(0, (height - 3) as i16, 0, width as i16),
            version: String::from("0.0.1"),
            status_bar: StatusBar::new(width as i16, (height - 2)as i16),
            bg: Color::Black,
            mode: Mode::NORMAL,
        }
    }

    fn step(&mut self, x: i16, y: i16) {
        self.c_pos.x += x;
        self.c_pos.y += y;
    }

    fn toggle_line_num(&mut self) {
        self.refeash = true;
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

    fn clear(&self) {
        queue!(
            stdout(),
            Clear(ClearType::All),
            ResetColor
        ).unwrap();
    }

    fn display_welcome(&self) {
        let mut messages = Vec::with_capacity(2);
        messages.push(
            format!("ReVim - Rust Edition Vim - Version {}", self.version));
/*
        messages.push(
            "To Quit: :q, To INSERT Text: i, For Help: :help <key or keyword>"
            .to_string());
*/
        messages.push("GitHub: https://www.github.com/cowboy8625/ReVim"
                      .to_string());
        for (y, msg) in messages.iter().enumerate() {
            let x = self.terminal_size.x / 2 - ((msg.len() as i16) / 2);
            let y = self.terminal_size.y / 3 + y as i16;
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
    }

    fn display_numbers(&self, fg: Color) {
        for num in self.s_pos.x..self.s_pos.y+1 {
            queue!(
                stdout(),
                MoveTo(0, num as u16),
                SetForegroundColor(fg),
                SetBackgroundColor(Color::Black),
                Print(num + 1),
                ResetColor
            )
            .unwrap();
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

    fn display_refresh(&mut self) -> Result<()> {

        if self.refeash { self.clear() }
        self.status_bar.render_status_bar(&self.mode);

        if let None = &self.text { self.display_welcome(); }

        if self.line_num {
            self.display_numbers(Color::Yellow);
        }

        if self.debug { self.debug_display(); }

        self.display_text();

        queue!(
            stdout(),
            MoveTo(self.c_pos.x as u16, self.c_pos.y as u16),
            ResetColor
        )
        .unwrap();
        stdout().flush()?;
        Ok(())
    }

    fn debug_display(&mut self) {
        self.refeash = true;
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
    let mut args = env::args();

    enable_raw_mode()?;

    let mut text = None;
    let _ = args.next();
    if let Some(arg) = args.next() {
        text = Some(Rope::from_reader(
            BufReader::new(File::open(&arg)?)
        )?);
    }

    let mut editor = Screen::new( Vector::new(0, 0), text);

    editor.clear();
    let update = (true, false);

    loop {
        if update.0 { editor.display_refresh()?; }

        let update = editor_key_event(&mut editor);

        if update.1 { break }
    }
    editor.clear();
    disable_raw_mode()
}

