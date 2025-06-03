use super::status_bar;
use crate::{
    game::{mode::Mode, player::Player},
    maps::map::Map,
    renderer::render::Render,
};

/// [`draw_playing_screen`] will draw the main gameplay screen containing a [`Map`] and
/// [`Player`] using a [`Render`] implementation.
pub fn draw_playing_screen<R: Render>(map: &Map, player: &Player, renderer: &mut R) {
    map.draw_tiles(renderer);
    player.draw_player(renderer);
    map.draw_buildings(renderer);
    status_bar::draw_status_bar(Mode::Playing, "Press ESC to quit", renderer);
}
