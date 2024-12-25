// OmniCAD
// A multi-domain CAD tool, written in Rust.
//
// Copyright (C) 2024 Luai "TheBeautifulOrc" Malek
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
    utils::HashMap,
};
use std::fmt::Debug;

pub struct UXPlugin;

impl Plugin for UXPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>();
        app.add_systems(Update, input_handler);
        app.insert_resource(Mode::default());
        app.insert_resource(ModifierKeys::new());
    }
}

#[derive(Event)]
struct InputEvent {
    key: Key,
    modifiers: ModifierKeys,
}
impl Debug for InputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputEvent")
            .field("key", &format!("{:?}", self.key))
            .field("modifiers", &format!("{:?}", self.modifiers.0))
            .finish()
    }
}

fn input_handler(
    mut event_writer: EventWriter<InputEvent>,
    mut key_event_reader: EventReader<KeyboardInput>,
    mut modifier_keys: ResMut<ModifierKeys>,
) {
    for key_event in key_event_reader.read() {
        // Track modifier keys
        if let Some(modifier_count) = modifier_keys.0.get_mut(&key_event.logical_key) {
            *modifier_count += match key_event.state {
                ButtonState::Pressed => 1,
                ButtonState::Released => -1,
            };
        }
        // Send signale for all other keys
        else if key_event.state == ButtonState::Pressed {
            event_writer.send(InputEvent {
                key: key_event.logical_key.clone(),
                modifiers: modifier_keys.as_ref().clone(),
            });
        }
    }
}

#[derive(Resource, Clone)]
pub struct ModifierKeys(HashMap<Key, i8>);

impl ModifierKeys {
    fn new() -> Self {
        let mut map: HashMap<Key, i8> = HashMap::new();
        map.insert(Key::Shift, 0);
        map.insert(Key::Control, 0);
        map.insert(Key::Alt, 0);
        ModifierKeys(map)
    }
}

#[derive(Resource, Component)]
pub struct Mode {
    name: &'static str,
    // TODO: Change string for enum-based approach for keymap
    keymap: HashMap<InputEvent, &'static str>,
}
impl Default for Mode {
    fn default() -> Self {
        Mode {
            name: "Default mode",
            keymap: HashMap::new(),
        }
    }
}

#[derive(Component, Default)]
enum Activity {
    #[default]
    Inactive,
    Selected,
    Active,
}

#[derive(Bundle)]
struct Viewport {
    default_mode: Mode,
    active: Activity,
    // TODO: Add UI implementation of Viewport
}
