use crate::{Editor, editor::EditorError, Mode, usub, render::StringCount};
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

    fn build_normal(self) -> Self {
        use Mode::*;
        /* Normal Mode */
        self.insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.is_running = false),
        )
        // Cursor Down
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            Box::new(|editor| {
                if editor.cursor.y != editor.screen.max_h.saturating_sub(1) as u16 {
                    // This is for moving cursor
                    editor.cursor.y = (editor.cursor.y + 1).min(
                        (std::cmp::min(editor.screen.bottom(), editor.rope.len_lines().saturating_sub(2)))
                            as u16);

                    editor.cursor.gy = (editor.cursor.gy + 1).min(editor.rope.len_lines().saturating_sub(2) as u16);
                } else {
                    // This is for scrolling
                    editor.screen.t = (editor.screen.t + 1).min(
                        std::cmp::max(editor.screen.bottom(), editor.rope.len_lines().saturating_sub(2)));

                    editor.cursor.gy = (editor.cursor.gy + 1).min(editor.rope.len_lines().saturating_sub(2) as u16);
                }
                editor.cursor.x = std::cmp::min(end_of_line_without_new_line(&editor), editor.cursor.max_x);
                editor.cursor.gx = std::cmp::min(end_of_line_without_new_line(&editor), editor.cursor.max_x);
            }),
        )
        // Cursor Up
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            Box::new(|editor| {
                if editor.cursor.y != 0 {
                    // This is for moving cursor
                    editor.cursor.y = editor.cursor.y.saturating_sub(1);
                    editor.cursor.gy = editor.cursor.gy.saturating_sub(1);
                } else {
                    // This is for scrolling
                    editor.screen.t = editor.screen.t.saturating_sub(1);
                    editor.cursor.gy = editor.cursor.gy.saturating_sub(1);
                }
                editor.cursor.x = std::cmp::min(end_of_line_without_new_line(&editor), editor.cursor.max_x);
                editor.cursor.gx = std::cmp::min(end_of_line_without_new_line(&editor), editor.cursor.max_x);
            }),
        )
        // Cursor Left
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.cursor.x = editor.cursor.x.saturating_sub(1);
                editor.cursor.gx = editor.cursor.gx.saturating_sub(1);
                editor.cursor.max_x = std::cmp::min(editor.cursor.gx, editor.cursor.max_x);
            }),
        )
        // Cursor Right
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.cursor.x = editor.cursor.x.saturating_add(1).min(end_of_line_without_new_line(&editor));
                editor.cursor.gx = editor.cursor.gx.saturating_add(1).min(end_of_line_without_new_line(&editor));
                editor.cursor.max_x = std::cmp::max(editor.cursor.x, editor.cursor.max_x);
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
                editor.error = EditorError::NONE;
                editor.output = String::new();
            }),
        )
    }

    fn build_insert(self) -> Self {
        use Mode::*;
        /* Insert Mode */
        self.insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Mode::Insert),
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            Box::new(|editor| {
                insert_str_to_rope(editor, "    ");
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
                editor.cursor.x = 0;
                editor.cursor.y += 1;
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
                if editor.cursor.x == 0 && editor.cursor.y == 0 {
                    return;
                }

                let line_index = editor.rope.line_to_char(editor.cursor.y as usize);
                let index = (line_index as u16 + editor.cursor.x - 1) as usize;
                editor.rope.remove(index..index + 1);

                let new_line = editor.rope.char_to_line(index);
                if new_line != editor.cursor.y as usize {
                    editor.cursor.y -= usub(editor.cursor.y, 1);
                }
                editor.cursor.x = (index - editor.rope.line_to_char(new_line)) as u16;
            }),
        )
        .key_adder(&Insert)
    }

    fn build_command(self) -> Self {
        use Mode::*;
        /* Command Mode */
        self.insert_mapping(
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
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            Box::new(|editor| {
                let _ = editor.command.pop();
            })
        )
        .key_adder(&Command)
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            Box::new(|editor: &mut Editor| {
                match editor.command.as_str() {
                    "q" => editor.is_running = false,
                    "w" => {
                            editor.rope.write_to(
                            std::io::BufWriter::new(
                                std::fs::File::create(editor.file_path.clone().unwrap()).expect("File Did Not save!"))).expect("Rope Did not save");
                            editor.error = EditorError::NONE;
                        }
                    "lenline" => editor.output = end_of_line_without_new_line(&editor).to_string(),
                    "height" => editor.output = editor.screen.max_h.to_string(),
                    "line" => editor.output = editor.rope.line(editor.cursor.gy as usize).chars().collect::<String>().trim_end().to_string(),
                    c => editor.error = EditorError::InvalidCommand(c.to_string()),
                }
                editor.mode = Mode::Normal;
                editor.command = String::new();// vec![" "; editor.screen.max_w].into_iter().collect();
            }),
        )
    }
}

pub fn key_builder() -> Mapper {
    Mapper::new()
        .build_normal()
        .build_insert()
        .build_command()
}

fn insert_char_to_rope(editor: &mut Editor, c: char) {
    let line_index = editor.rope.line_to_char(editor.cursor.y as usize);
    editor
        .rope
        .insert_char(line_index + editor.cursor.x as usize, c);
    editor.cursor.x += 1;
}

fn insert_str_to_rope(editor: &mut Editor, s: &str) {
    for c in s.chars() {
        insert_char_to_rope(editor, c);
    }
}

pub(crate) fn end_of_line_without_new_line(editor: &Editor) -> u16 {
    let line = editor
        .rope
        .line(editor.cursor.gy as usize)
        .chars()
        .collect::<String>()
        .trim_end()
        .to_string();
    let len = line.len().saturating_sub(1);
    let tabs = line.count_char('\t') * 3;
    (len + tabs) as u16

}

#[test]
fn test_line_len_rope() {
    use crate::commandline;
    let (rope, s) = commandline::from_path(Some("./KJV.txt".to_string()));
    assert_eq!(s, Some("./KJV.txt".to_string()));
    let v = rope.line(0).as_str().unwrap_or("\n").trim().chars().collect::<Vec<char>>();
    assert_eq!(v.len(), 66);
}

#[test]
fn test_line_end_on_rope() {
    use crate::commandline;
    let (rope, s) = commandline::from_path(Some("./KJV.txt".to_string()));
    assert_eq!(s, Some("./KJV.txt".to_string()));
    let v = rope.line(0).as_str().unwrap_or("\n").chars().collect::<Vec<char>>();
    eprintln!("len of line: {}", v.len());
    eprintln!("{:?}", v);
    assert_eq!(v[v.len() - 3], '.');
    assert_eq!(v[v.len() - 2], '\r');
    assert_eq!(v[v.len() - 1], '\n');
}

#[test]
fn test_line_len_rope_keymapper() {
    use crate::commandline;
    let (rope, s) = commandline::from_path(Some("./src/keymapper.rs".to_string()));
    assert_eq!(s, Some("./src/keymapper.rs".to_string()));
    let v = rope.line(8).as_str().unwrap_or("\n").trim().chars().collect::<Vec<char>>();
    println!("{}", v.iter().collect::<String>());
    assert_eq!(v.len(), 13);
}
