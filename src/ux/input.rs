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
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        ButtonState,
    },
    prelude::*,
};
use std::fmt;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>();
        app.add_systems(
            Update,
            (
                keyboard_handler.run_if(on_event::<KeyboardInput>),
                click_handler.run_if(on_event::<MouseButtonInput>),
                scroll_handler.run_if(on_event::<MouseWheel>),
                mouse_motion_handler.run_if(on_event::<MouseMotion>),
                #[cfg(feature = "debug_input")]
                event_receiver.run_if(on_event::<InputEvent>),
            ),
        );
    }
}

pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}
impl fmt::Debug for ScrollDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
        };
        write!(f, "{}", dir)
    }
}

#[derive(Event)]
pub enum InputEvent {
    Keyboard {
        key: Key,
        state: ButtonState,
        repeat: bool,
    },
    Click {
        button: MouseButton,
        state: ButtonState,
    },
    Scroll(ScrollDirection),
    MouseMotion(Vec2),
}

#[cfg(debug_assertions)]
impl fmt::Debug for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputEvent::Keyboard { key, state, repeat } => f
                .debug_struct("InputEvent::Keyboard")
                .field("key", key)
                .field("state", state)
                .field("repeat", repeat)
                .finish(),
            InputEvent::Click { button, state } => f
                .debug_struct("InputEvent::Click")
                .field("button", button)
                .field("state", state)
                .finish(),
            InputEvent::Scroll(dir) => f
                .debug_struct("InputEvent::Scroll")
                .field("direction", dir)
                .finish(),
            InputEvent::MouseMotion(vec) => f
                .debug_struct("InputEvent::MouseMotion")
                .field("vec", vec)
                .finish(),
        }
    }
}

fn keyboard_handler(
    mut key_event_reader: EventReader<KeyboardInput>,
    mut event_writer: EventWriter<InputEvent>,
) {
    for key_event in key_event_reader.read() {
        event_writer.send(InputEvent::Keyboard {
            key: key_event.logical_key.clone(),
            state: key_event.state,
            repeat: key_event.repeat,
        });
    }
}

fn click_handler(
    mut click_event_reader: EventReader<MouseButtonInput>,
    mut event_writer: EventWriter<InputEvent>,
) {
    for click_event in click_event_reader.read() {
        event_writer.send(InputEvent::Click {
            button: click_event.button,
            state: click_event.state,
        });
    }
}

fn scroll_handler(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut event_writer: EventWriter<InputEvent>,
) {
    for scroll_event in scroll_event_reader.read() {
        let mut dir: Option<ScrollDirection> = None;
        dir = Some(if scroll_event.x > 0f32 {
            ScrollDirection::Left
        } else {
            ScrollDirection::Right
        });
        dir = Some(if scroll_event.y > 0f32 {
            ScrollDirection::Up
        } else {
            ScrollDirection::Down
        });
        if let Some(dir) = dir {
            event_writer.send(InputEvent::Scroll(dir));
        }
    }
}

fn mouse_motion_handler(
    mut event_reader: EventReader<MouseMotion>,
    mut event_writer: EventWriter<InputEvent>,
) {
    for event in event_reader.read() {
        event_writer.send(InputEvent::MouseMotion(event.delta));
    }
}

#[cfg(feature = "debug_input")]
fn event_receiver(mut event_reader: EventReader<InputEvent>) {
    for event in event_reader.read() {
        println!("{:?}", event);
    }
}
