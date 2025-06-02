use super::status_bar;
use crate::{game::mode::Mode, renderer::render::Render};

pub fn draw_crafting_screen<R: Render>(renderer: &mut R) {
    renderer.draw_menu(5., 10., "CRAFTING", "This will be where you craft stuff.");
    status_bar::draw_status_bar(Mode::Crafting, "Press ESC to resume", renderer);
}
