use crate::{
    game::mode::Mode,
    renderer::{color::Color, render::Render},
};

pub fn draw_status_bar<R: Render>(mode: Mode, status: &str, renderer: &mut R) {
    let status_message = if status.is_empty() {
        &format!("{mode}")
    } else {
        &format!("{mode} | {status}")
    };

    renderer.draw_text(
        renderer.get_tile_size(),
        renderer.get_screen_size().1 - renderer.get_tile_size(),
        status_message,
        Color::White,
        Color::Black,
    );
}
