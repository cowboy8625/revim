// use crossdisplay::tui::{Direction, EditorEvent, KeyCode, KeyEvent, KeyModifier};
use crate::{usub, Editor, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

type EditorEvent = Box<dyn Fn(&mut Editor)>;
type KeyMap = HashMap<KeyEvent, EditorEvent>;

pub struct Mapper {
    nmaps: KeyMap,
    imaps: KeyMap,
    cmaps: KeyMap,
}

impl Mapper {
    fn new() -> Self {
        Self {
            nmaps: KeyMap::new(),
            imaps: KeyMap::new(),
            cmaps: KeyMap::new(),
        }
    }

    fn get_map(&self, mode: &Mode) -> &KeyMap {
        use Mode::*;
        match mode {
            Normal => &self.nmaps,
            Insert => &self.imaps,
            Command => &self.cmaps,
        }
    }

    fn get_map_mut(&mut self, mode: &Mode) -> &mut KeyMap {
        use Mode::*;
        match mode {
            Normal => &mut self.nmaps,
            Insert => &mut self.imaps,
            Command => &mut self.cmaps,
        }
    }

    pub fn get_mapping(&self, mode: &Mode, event: &KeyEvent) -> Option<&EditorEvent> {
        Some(self.get_map(mode).get(event)?)
    }

    pub fn insert_mapping(mut self, mode: &Mode, key: KeyEvent, event: EditorEvent) -> Self {
        self.get_map_mut(mode).insert(key, event);
        self
    }

    pub fn insert_mapping_chain(mut self, mode: &Mode, keys: &str, modifier: KeyModifiers) -> Self {
        for c in keys.chars() {
            match mode {
                Mode::Command => {
                    self.get_map_mut(mode).insert(
                        KeyEvent::new(KeyCode::Char(c), modifier),
                        Box::new(move |editor| editor.command.push(c)),
                    );
                }
                Mode::Insert => {
                    self.get_map_mut(mode).insert(
                        KeyEvent::new(KeyCode::Char(c), modifier),
                        Box::new(move |editor| {
                            insert_char_to_rope(editor, c);
                        }),
                    );
                }
                _ => {}
            }
        }
        self
    }

    pub fn key_adder(self, mode: &Mode) -> Self {
        self
            .insert_mapping_chain(
                mode,
                ('a'..='z').collect::<String>().as_str(),
                KeyModifiers::NONE,
            )
            .insert_mapping_chain(
                mode,
                ('A'..='Z').collect::<String>().as_str(),
                KeyModifiers::SHIFT,
            )
            .insert_mapping_chain(
                mode,
                ('0'..='9').collect::<String>().as_str(),
                KeyModifiers::NONE,
            )
            .insert_mapping_chain(
                mode,
                "!@#$%^&*()_+-=[]{}\\|\"':;,.<>/?",
                KeyModifiers::NONE,
            )
    }
}

pub fn key_builder() -> Mapper {
    use Mode::*;
    Mapper::new()
        /* Normal Mode */
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.is_running = false),
        )
        // Cursor Down
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            Box::new(|editor| {
                // TODO (1):
                // Make cursor move to end of line when moving down if
                // line is sorter then what cursor is currently on
                // before moving down to next line.
                // TODO (2):
                // Scrolling.
                if editor.cursor.1 == editor.screen.max_h as u16 {
                    editor.screen.x = (editor.screen.x + 1).min(
                        std::cmp::max(editor.screen.bottom(), usub(editor.rope.len_lines(), 2))
                        )
                } else {
                    editor.cursor.1 = (editor.cursor.1 + 1).min(
                        // MAYBE: Fix This.
                        (std::cmp::min(editor.screen.bottom(), editor.rope.len_lines().saturating_sub(2)))
                            as u16,
                    );
                }
                editor.cursor.0 = editor.cursor.0.min(usub(
                    editor.rope.line(editor.cursor.1 as usize + editor.screen.y).chars().len() as u16,
                    2,
                ));
            }),
        )
        // Cursor Up
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            Box::new(|editor| {
                // TODO (1):
                // Make cursor move to end of line when moving down if
                // line is sorter then what cursor is currently on
                // before moving down to next line.
                // TODO (2):
                // Scrolling.
                if editor.cursor.1 == 0 as u16 {
                    editor.screen.x = editor.screen.x.saturating_sub(1);
                } else {
                    editor.cursor.1 = (editor.cursor.1 - 1).max(0);
                }
                editor.cursor.0 = editor.cursor.0.min(usub(
                    editor.rope.line(editor.cursor.1 as usize + editor.screen.y).chars().len() as u16,
                    2,
                ));
            // Box::new(|editor| {
            //     editor.cursor.1 = usub(editor.cursor.1, 1);
            //     editor.cursor.0 = editor.cursor.0.min(usub(
            //         editor.rope.line(editor.cursor.1 as usize).chars().len() as u16,
            //         2,
            //     ));
            }),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            Box::new(|editor| editor.cursor.0 = usub(editor.cursor.0, 1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.cursor.0 = (editor.cursor.0 + 1).min(usub(
                    editor.rope.line(editor.cursor.1 as usize).chars().len() as u16,
                    2,
                ));
            }),
        )
        .insert_mapping(
            &Normal,
            if cfg!(windows) {
                KeyEvent::new(KeyCode::Char(':'), KeyModifiers::SHIFT)
            } else {
                KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE)
            },
            Box::new(|editor| {
                editor.mode = Command;
                editor.command.clear();
            }),
        )
        /* Command Mode */
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Normal),
        )
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| {
                editor.mode = Normal;
                editor.command = vec![" "; editor.screen.max_w].into_iter().collect();
            }),
        )
        .key_adder(&Command)
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            Box::new(|editor: &mut Editor| {
                match editor.command.as_str() {
                    "q" => editor.is_running = false,
                    "w" => editor.rope.write_to(
                        std::io::BufWriter::new(
                            std::fs::File::create(editor.file_path.clone().unwrap()).expect("File Did Not save!"))).expect("Rope Did not save"),
                    _ => {}
                }
                editor.mode = Mode::Normal;
                editor.command = vec![" "; editor.screen.max_w].into_iter().collect();
            }),
        )
        /* Insert Mode */
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Mode::Insert),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            Box::new(|editor| {
                insert_char_to_rope(editor, ' ');
                insert_char_to_rope(editor, ' ');
                insert_char_to_rope(editor, ' ');
                insert_char_to_rope(editor, ' ');
            }),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Normal),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            Box::new(|editor| {
                insert_char_to_rope(editor, '\n');
                editor.cursor.0 = 0;
                editor.cursor.1 += 1;
            }),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            Box::new(|editor| {
                insert_char_to_rope(editor, ' ');
            }),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            Box::new(|editor| {
                if editor.cursor == (0, 0) {
                    return;
                }

                let line_index = editor.rope.line_to_char(editor.cursor.1 as usize);
                let index = (line_index as u16 + editor.cursor.0 - 1) as usize;
                editor.rope.remove(index..index + 1);

                let new_line = editor.rope.char_to_line(index);
                if new_line != editor.cursor.1 as usize {
                    editor.cursor.1 -= usub(editor.cursor.1, 1);
                }
                editor.cursor.0 = (index - editor.rope.line_to_char(new_line)) as u16;
            }),
        )
        .key_adder(&Insert)
}

fn insert_char_to_rope(editor: &mut Editor, c: char) {
    let line_index = editor.rope.line_to_char(editor.cursor.1 as usize);
    editor
        .rope
        .insert_char(line_index + editor.cursor.0 as usize, c);
    editor.cursor.0 += 1;
}
