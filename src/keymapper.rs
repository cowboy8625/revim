// use crossdisplay::tui::{Direction, EditorEvent, KeyCode, KeyEvent, KeyModifier};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use crate::{Editor, Mode, usub};
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
            self.get_map_mut(mode).insert(KeyEvent::new(KeyCode::Char(c), modifier), Box::new(move |editor| editor.command.push(c)));
        }
        self
    }
}

pub fn key_builder() -> Mapper {
    use Mode::*;
    Mapper::new()

        /* Normal Mode */

        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        //     Box::new(|editor| editor.is_running = false)
        // )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            Box::new(|editor| {
                // Make cursor move to end of line when moving down if
                // line is sorter then what cursor is currently on
                // before moving down to next line.
                editor.cursor.1 = (editor.cursor.1 + 1)
                    .min((std::cmp::min(editor.screen.bottom(), usub(editor.rope.len_lines(), 2))) as u16);
                editor.cursor.0 = editor.cursor.0
                    .min(usub(editor.rope.line(editor.cursor.1 as usize).chars().len() as u16,  2));
            })
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.cursor.1 = usub(editor.cursor.1, 1);
                editor.cursor.0 = editor.cursor.0
                    .min(usub(editor.rope.line(editor.cursor.1 as usize).chars().len() as u16,  2));
            })
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            Box::new(|editor| editor.cursor.0 = usub(editor.cursor.0, 1))
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.cursor.0 = (editor.cursor.0 + 1)
                    .min(usub(editor.rope.line(editor.cursor.1 as usize).chars().len() as u16,  2));
            })
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE),
            Box::new(|editor| {
                editor.mode = Command;
                editor.command.clear();
            })
        )

        /* Command Mode */

        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Normal)
        )
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| {
                editor.mode = Normal;
                editor.command = vec![" "; editor.screen.max_w].into_iter().collect();
            })
        )
        .insert_mapping_chain(
            &Command,
            ('a'..='z').collect::<String>().as_str(),
            KeyModifiers::NONE,
        )
        .insert_mapping_chain(
            &Command,
            ('a'..='z').collect::<String>().as_str(),
            KeyModifiers::SHIFT,
        )
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            Box::new(|editor: &mut Editor| {
                match editor.command.as_str() {
                "q" => editor.is_running = false,
                _ => {}
                }
                editor.mode = Mode::Normal;
                editor.command = vec![" "; editor.screen.max_w].into_iter().collect();
            })
        )

        /* Insert Mode */

        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.mode = Normal)
        )
        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Char('i'), KeyModifier::NONE),
        //     EditorEvent::ModeInsert,
        // )
}
