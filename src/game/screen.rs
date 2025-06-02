use crate::{events::input::InputEvent, renderer::render::Render};

/// [`Screen`] will be implemented to handle drawing a screen and handling that screen's input.
pub trait Screen {
    /// [`Screen::draw_screen`] will draw a screen using the provided [`Render`] implementation.
    fn draw_screen<R: Render>(&self, renderer: &mut R);

    /// [`Screen::handle_input`] will handle the `input` for any screen.
    fn handle_input(&mut self, input: InputEvent, frame_time: f32);
}
