use super::status_bar;
use crate::{game::mode::Mode, maps::map::Map, renderer::render::Render};

/// [`draw_crafting_screen`] will draw the crafting menu on top of the [`Map`]
/// using a [`Render`] implementation.
pub fn draw_crafting_screen<R: Render>(map: &Map, renderer: &mut R) {
    map.draw_map(renderer);
    renderer.draw_menu(20., 20., "CRAFTING", "This will be where you craft stuff.");
    status_bar::draw_status_bar(Mode::Crafting, "Press ESC to resume", renderer);
}
