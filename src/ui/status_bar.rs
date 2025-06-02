use crate::{
    game::mode::Mode,
    renderer::{color::Color, render::Render},
};

/// [`draw_status_bar`] will draw a status bar based on a provided [`Mode`] and
/// `status` message using a [`Render`] implementation.
pub fn draw_status_bar<R: Render>(mode: Mode, status: &str, renderer: &mut R) {
    let status_message = if status.is_empty() {
        &format!("{mode}")
    } else {
        &format!("{mode} | {status}")
    };

    renderer.draw_text(
        renderer.get_tile_size() / 2.,
        renderer.get_screen_size().1 - (renderer.get_tile_size() / 2.),
        status_message,
        Color::White,
        Color::Black,
    );
}
