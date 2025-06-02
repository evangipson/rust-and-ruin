use super::status_bar;
use crate::{
    game::{map::Map, mode::Mode, player::Player},
    renderer::render::Render,
};

pub fn draw_playing_screen<R: Render>(map: &Map, player: &Player, renderer: &mut R) {
    map.draw_map(renderer);
    player.draw_player(renderer);
    status_bar::draw_status_bar(Mode::Playing, "Press ESC to quit", renderer);
}
