mod editor;
mod commandline;
mod render;
mod keymapper;
mod util;

use render::*;
use commandline::{argparser, from_path};
use keymapper::*;
use util::usub;
use editor::{Editor, Mode};

use ropey::Rope;
use crossterm::event;

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
        if event::poll(std::time::Duration::from_micros(200))? {
            if let event::Event::Key(key) = event::read()? {
                if let Mode::Command = &editor.mode {
                    if let crossterm::event::KeyEvent{code: crossterm::event::KeyCode::Char(c), ..} = key {
                        editor.rope.insert_char(
                    }
                }
                if let Mode::Insert = &editor.mode {
                }
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
