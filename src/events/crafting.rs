use super::{event::Event, input::InputEvent};

pub fn handle_crafting_input(event: InputEvent) -> Event {
    match event {
        InputEvent::Quit => Event::Back,
        _ => Event::Nothing,
    }
}
