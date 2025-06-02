use super::{event::Event, input::InputEvent};

pub fn handle_title_screen_input(event: InputEvent) -> Event {
    match event {
        InputEvent::Character('s') | InputEvent::Interact => Event::Continue,
        InputEvent::Quit => Event::Quit,
        _ => Event::Nothing,
    }
}
