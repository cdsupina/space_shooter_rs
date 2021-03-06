use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

pub struct PausedState;

impl SimpleState for PausedState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}
