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
}

pub fn key_builder() -> Mapper {
    use Mode::*;
    Mapper::new()
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Box::new(|editor| editor.is_running = false)
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            Box::new(|editor| {
                // Make cursor move to end of line when moving down if
                // line is sorter then what cursor is currently on
                // before moving down to next line.
                editor.cursor.1 = (editor.cursor.1 + 1)
                    .min((editor.screen.0 + editor.screen.1) as u16);
            })
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            Box::new(|editor| editor.cursor.1 = usub(editor.cursor.1, 1))
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
        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Char('e'), KeyModifier::Control),
        //     EditorEvent::Scroll(Direction::Down(1), Direction::Up(1)),
        // )
        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Char('y'), KeyModifier::Control),
        //     EditorEvent::Scroll(Direction::Up(1), Direction::Down(1)),
        // )
        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Char(':'), KeyModifier::NONE),
        //     EditorEvent::ModeCommand,
        // )
        // .insert_mapping(
        //     &Command,
        //     KeyEvent::new(KeyCode::Esc, KeyModifier::NONE),
        //     EditorEvent::ModeNormal,
        // )
        // .insert_mapping(
        //     &Normal,
        //     KeyEvent::new(KeyCode::Char('i'), KeyModifier::NONE),
        //     EditorEvent::ModeInsert,
        // )
        // .insert_mapping(
        //     &Insert,
        //     KeyEvent::new(KeyCode::Esc, KeyModifier::NONE),
        //     EditorEvent::ModeNormal,
        // )
}
