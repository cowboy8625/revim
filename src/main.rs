// ReVim Support Crates
use crossdisplay::tui::{
    enter_raw_mode, exit_raw_mode, move_to, poll, read, render, terminal_size, Direction,
    EditorEvent, Event, KeyEvent, KeyCode, Result,
};

// ReVim Modules
mod keymapper;
mod support;
mod screen;
mod commandline;
mod textbuffer;

use support::usubtraction;
use keymapper::{key_builder, Mapper, Mode};
use screen::{string_to_vec, replace_from, insert_at};
use commandline::argparser;
use textbuffer::TextBuffer;

// Standard Library Crates
use std::io::{stdout, Stdout};
use std::time::Duration;

fn main() -> Result<()> {
    let filename = argparser();
    let mut editor = ReVim::new(filename);
    editor.mainloop()?;
    Ok(())
}

struct Cursor {
    loc_x: u16, // x on Screen.
    loc_y: u16, // y on Screen.
    glb_x: u16, // x in text file.
    glb_y: u16, // y in text file.
    max_x: u16, // the max x value used last.
}

impl Cursor {
    fn new() -> Self {
        Self {
            loc_x: 0,
            loc_y: 0,
            glb_x: 0,
            glb_y: 0,
            max_x: 0,
        }
    }

    fn loc(&self) -> (u16, u16) {
        (self.loc_x, self.loc_y)
    }
}

pub struct ReVim {
    stdout: Stdout,
    cursor: Cursor,       // Hold Cursor data.
    dim: (u16, u16),      // dimensions of the Screen,
    view: (u16, u16),     // view is the y of the top/bottom of the Screen in regaurds of the file
    window: Vec<char>,    // is a screen buffer
    filedata: TextBuffer, // is a data structer that holds the hold file.
    queued: Vec<usize>,   // index of lines that will be queued to be updated.
    key_map: Mapper,      // Struct for handling key mappings for modes.
    is_running: bool,     // switch for main loop.
    mode: Mode,           // Shows what mode/state ReVim is in.
}

impl ReVim {
    fn new(filename: Option<String>) -> Self {
        let (w, h) = terminal_size().unwrap();
        let mut window: Vec<char> = (0..w * h).map(|_| '#').collect();
        let filedata = TextBuffer::from_path(filename);
        let lines = filedata.line_to_line(0, h as usize);
        let src = string_to_vec(w, &lines);
        let mut queued: Vec<usize> = Vec::new();
        replace_from(0, w as usize, &mut window, &src, &mut queued);
        Self {
            stdout: stdout(),
            cursor: Cursor::new(),
            dim: (w, h),
            view: (0, h),
            window,
            filedata,
            queued,
            key_map: key_builder(),
            is_running: true,
            mode: Mode::Normal,
        }
    }

    fn scroll_up(&mut self) -> Result<()> {
        self.view.0 -= 1;
        self.view.1 -= 1;
        self.cursor.glb_y -= 1;
        let lines = self
            .filedata
            .line_to_line(self.view.0 as usize, self.view.1 as usize);
        let src = string_to_vec(self.dim.0, &lines);
        replace_from(
            0,
            self.dim.0 as usize,
            &mut self.window,
            &src,
            &mut self.queued,
        );
        Ok(())
    }

    fn scroll_down(&mut self) -> Result<()> {
        self.view.0 += 1;
        self.view.1 += 1;
        self.cursor.glb_y += 1;
        let lines = self
            .filedata
            .line_to_line(self.view.0 as usize, self.view.1 as usize);
        let src = string_to_vec(self.dim.0, &lines);
        replace_from(
            0,
            self.dim.0 as usize,
            &mut self.window,
            &src,
            &mut self.queued,
        );
        Ok(())
    }

    fn cursor_down(&mut self) -> Result<()> {
        if (self.cursor.glb_y as usize) < self.filedata.len_lines() {
            let next_line_len =
                usubtraction(self.filedata.line_len((self.cursor.glb_y + 1) as usize), 1);
            self.cursor.loc_x = std::cmp::min(self.cursor.loc_x, next_line_len as u16);
            if self.cursor.glb_y < self.view.1 - 1{
                self.cursor.loc_y += 1;
                self.cursor.glb_y += 1;
                move_to(&mut self.stdout, self.cursor.loc())?;
            } else {
                self.scroll_down()?;
            }
            self.cursor.loc_x = std::cmp::max(self.cursor.loc_x, self.cursor.max_x);
        }
        Ok(())
    }

    fn cursor_up(&mut self) -> Result<()> {
        // all curser movements need to be moved to functions.
        if self.cursor.glb_y > 0 {
            let next_line_len =
                usubtraction(self.filedata.line_len((self.cursor.glb_y - 1) as usize), 1);
            self.cursor.loc_x = std::cmp::min(self.cursor.loc_x, next_line_len as u16);
            if self.cursor.glb_y > self.view.0{
                self.cursor.loc_y -= 1;
                self.cursor.glb_y -= 1;
                move_to(&mut self.stdout, self.cursor.loc())?;
            } else {
                self.scroll_up()?;
            }
            self.cursor.loc_x = std::cmp::max(self.cursor.loc_x, self.cursor.max_x);
        }
        Ok(())
    }

    fn cursor_left(&mut self) -> Result<()> {
        // all curser movements need to be moved to functions.
        if self.cursor.glb_x > 0 {
            self.cursor.loc_x -= 1;
            self.cursor.glb_x -= 1;
            move_to(&mut self.stdout, self.cursor.loc())?;
            self.cursor.max_x = self.cursor.glb_x;
        }
        Ok(())
    }

    fn cursor_right(&mut self) -> Result<()> {
        // all curser movements need to be moved to functions.
        // TODO: Move Right goes one space to far "some time"
        if (self.cursor.glb_x as usize)
            < usubtraction(self.filedata.line_len(self.cursor.glb_y as usize), 1)
        {
            self.cursor.loc_x += 1;
            self.cursor.glb_x += 1;
            move_to(&mut self.stdout, self.cursor.loc())?;
            self.cursor.max_x = self.cursor.loc_x;
        }
        Ok(())
    }

    fn insert_char(&mut self, chr: char) -> Result<()> {
        self.filedata.insert_char(self.cursor.glb_x, self.cursor.glb_y, chr);
        self.cursor.glb_x += 1;
        self.cursor.loc_x += 1;
        let width = self.dim.0 as usize;
        let idx = width * (self.cursor.loc_y as usize) +(self.cursor.loc_x as usize) - 1;
        insert_at(idx, width, &mut self.window, &mut self.queued, chr);
        move_to(&mut self.stdout, self.cursor.loc())?;
        Ok(())
    }

    fn quit(&mut self) -> Result<()> {
        self.is_running = false;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        render(&mut self.stdout, &self.dim.0, &self.window, &self.queued)?;
        self.queued.clear();
        Ok(())
    }

    fn handle_event(&mut self, event: EditorEvent) -> Result<()> {
        match event {
            EditorEvent::Cursor(d) => {
                match d {
                    Direction::Up(_) => self.cursor_up()?,
                    Direction::Down(_) => self.cursor_down()?,
                    Direction::Left(_) => self.cursor_left()?,
                    Direction::Right(_) => self.cursor_right()?,
                };
            }
            EditorEvent::Scroll(d) => {
                match d {
                    Direction::Up(_) => self.scroll_up()?,
                    Direction::Down(_) => self.scroll_down()?,
                    _ => {}
                };
            }
            EditorEvent::Quit => self.quit()?,
            EditorEvent::ModeNormal => self.mode = Mode::Normal,
            EditorEvent::ModeInsert => self.mode = Mode::Insert,
            EditorEvent::ModeCommand => self.mode = Mode::Command,
        }
        Ok(())
    }

    fn handle_modes(&mut self, key: KeyEvent) -> Result<()> {
            match self.mode {
                Mode::Insert => {
                    match key.code {
                        KeyCode::Char(chr) => self.insert_char(chr)?,
                        //Enter => //self.window.insert_enter(),
                        //Backspace => //self.window.backspace(),
                        //Delete => //self.window.delete(),
                        _ => {},
                    };
                },
                Mode::Command => {
                    /*
                    match self.bar.handle_key(key) {
                        ExResult::Aborted => self.set_mode(Mode::Normal),
                        ExResult::StillEditing => {},
                        ExResult::Finished(cmd) => {
                            self.perform_ex_cmd(cmd);
                            self.set_mode(Mode::Normal);
                        },
                    }
                     */
                },
                _ => {}
            }
            Ok(())
        }

    fn mainloop(&mut self) -> Result<()> {
        enter_raw_mode(&mut self.stdout)?;
        while self.is_running {
            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = read()? {
                    if let Some(event) = self.key_map.get_mapping(&self.mode, &key) {
                        self.handle_event(event)?;
                    } else {
                        self.handle_modes(key)?;
                    }
                }
            }
            self.draw()?;
        }
        exit_raw_mode(&mut self.stdout)?;
        Ok(())
    }
}

