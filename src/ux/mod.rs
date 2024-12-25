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

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use input::InputPlugin;
use ui::UIPlugin;

mod input;
mod ui;

pub struct UXPlugins;

impl PluginGroup for UXPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(UIPlugin)
    }
}
