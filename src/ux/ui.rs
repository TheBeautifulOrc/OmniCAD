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

use super::input::InputEvent;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Mode::default());
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
