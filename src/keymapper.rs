use crossdisplay::tui::{Direction, EditorEvent, KeyCode, KeyEvent, KeyModifier};
use std::collections::HashMap;

type KeyMap = HashMap<KeyEvent, EditorEvent>;

pub enum Mode {
    Insert,
    Normal,
    Command,
}

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

    pub fn get_mapping(&self, mode: &Mode, event: &KeyEvent) -> Option<EditorEvent> {
        Some(self.get_map(mode).get(event)?.clone())
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
            KeyEvent::new(KeyCode::Esc, KeyModifier::NONE),
            EditorEvent::Quit,
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('j'), KeyModifier::NONE),
            EditorEvent::Cursor(Direction::Down(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('k'), KeyModifier::NONE),
            EditorEvent::Cursor(Direction::Up(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('h'), KeyModifier::NONE),
            EditorEvent::Cursor(Direction::Left(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('l'), KeyModifier::NONE),
            EditorEvent::Cursor(Direction::Right(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('e'), KeyModifier::Control),
            EditorEvent::Scroll(Direction::Down(1), Direction::Up(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('y'), KeyModifier::Control),
            EditorEvent::Scroll(Direction::Up(1), Direction::Down(1)),
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char(':'), KeyModifier::NONE),
            EditorEvent::ModeCommand,
        )
        .insert_mapping(
            &Command,
            KeyEvent::new(KeyCode::Esc, KeyModifier::NONE),
            EditorEvent::ModeNormal,
        )
        .insert_mapping(
            &Normal,
            KeyEvent::new(KeyCode::Char('i'), KeyModifier::NONE),
            EditorEvent::ModeInsert,
        )
        .insert_mapping(
            &Insert,
            KeyEvent::new(KeyCode::Esc, KeyModifier::NONE),
            EditorEvent::ModeNormal,
        )
}
