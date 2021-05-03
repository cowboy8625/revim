mod commandline;
mod editor;
mod keymapper;
mod render;
mod util;

use commandline::{argparser, from_path};
use editor::{Editor, Mode};
use keymapper::*;
use render::*;
use util::usub;

use crossterm::event;
use ropey::Rope;

fn main() -> crossterm::Result<()> {
    let mut writer = std::io::stdout();
    let file_path = argparser();
    let (rope, path) = from_path(file_path);
    let mut editor = Editor::new(rope, path);
    let key_map = key_builder();
    render_enter_alt_screen(&mut writer);
    render(&mut writer, &editor);
    while editor.is_running {
        if event::poll(std::time::Duration::from_millis(50))? {
            let event = event::read()?;
            if let event::Event::Key(key) = event {
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
