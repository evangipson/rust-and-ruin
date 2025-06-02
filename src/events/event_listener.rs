use super::{event::Event, input::InputEvent};

/// [`EventListener`] will be implemented to take in an [`InputEvent`],
/// parse it and return an [`Event`] as the result.
pub trait EventListener {
    /// [`EventListener::handle_input`] will parse an [`InputEvent`] and
    /// return an [`Event`] result.
    fn handle_input(&self, input: InputEvent) -> Event;
}
