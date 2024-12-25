use bevy::{prelude::*, utils::HashMap};

pub struct UXPlugin;

impl Plugin for UXPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>();
        app.add_systems(Update, input_handler);
        app.insert_resource(Mode::default());
    }
}
#[derive(Event)]
struct InputEvent(KeyCode);

fn input_handler(
    mut ev_writer: EventWriter<InputEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    mut viewports: Query<(&Activity, &mut Mode)>,
) {
    for key in keys.get_just_pressed() {
        ev_writer.send(InputEvent(key.clone()));
    }
}
// fn event_sender(mut ev_test: EventWriter<TestEvent>, keys: Res<ButtonInput<KeyCode>>) {
//     for key in keys.get_just_pressed() {
//         ev_test.send(TestEvent(key.clone()));
//     }
// }
// fn event_receiver(mut ev_test: EventReader<TestEvent>) {
//     for ev in ev_test.read() {
//         println!("Event with payload {:?} received", ev.0);
//     }
// }

#[derive(Resource, Component)]
pub struct Mode {
    name: &'static str,
    // TODO: Change string for enum-based approach for keymap
    keymap: HashMap<KeyCode, &'static str>,
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
