mod commandline;
mod render;
mod keymapper;
mod util;

use render::*;
use commandline::argparser;
use keymapper::*;
use util::usub;

use std::fs::{metadata, OpenOptions};
use std::io::BufReader;
use ropey::Rope;
use crossterm::event;

// wrapper around Rope for a drity flag.
fn from_path(path: Option<String>) -> (Rope, Option<String>) {
    let text = path
        .as_ref()
        .filter(|path| metadata(&path).is_ok())
        .map_or_else(Rope::new, |path| {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Problem opening the file");

            Rope::from_reader(BufReader::new(file)).unwrap()
        });

    ( text, path )
}

fn screen_size() -> (usize, usize) {
    let (_, h) = crossterm::terminal::size()
        .expect("Your screen size is messed up and I will fix it later");
    (0, h as usize)
}

#[derive(Debug)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

#[derive(Debug)]
pub struct Editor {
    rope: Rope,
    screen: (usize, usize),
    is_running: bool,
    mode: Mode,
    cursor: (u16, u16),
}

impl Editor {
    pub fn new(rope: Rope) -> Self {
        Self {
            rope,
            screen: screen_size(),
            is_running: true,
            mode: Mode::Normal,
            cursor: (0, 0),
        }
    }
}

fn main() -> crossterm::Result<()> {
    let mut writer = std::io::stdout();
    let file_path = argparser();
    let (rope, _path) = from_path(file_path);
    let mut editor = Editor::new(rope);
    let key_map = key_builder();
    render_enter_alt_screen(&mut writer);
    render_clear(&mut writer);
    render(&mut writer, &editor);
    while editor.is_running {
        if event::poll(std::time::Duration::from_micros(100))? {
            if let event::Event::Key(key) = event::read()? {
                if let Some(handle) = key_map.get_mapping(&editor.mode, &key) {
                    handle(&mut editor);
                }
            }
            render(&mut writer, &editor);
        }
    }
    render_exit_alt_screen(&mut writer);
    Ok(())
}
