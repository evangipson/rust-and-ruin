use super::player::Player;
use crate::{
    maps::map::Map,
    renderer::render::Render,
    ui::{crafting, interface::Interface, main_menu, playing},
};

/// [`Mode`] represents the current mode of the game.
pub enum Mode {
    TitleScreen,
    Playing,
    Inventory,
    Crafting,
}

impl Interface for Mode {
    fn draw_interface<R: Render>(&self, map: &Map, player: &Player, renderer: &mut R) {
        match self {
            Mode::TitleScreen => main_menu::draw_title_screen(renderer),
            Mode::Playing => playing::draw_playing_screen(map, player, renderer),
            Mode::Crafting => crafting::draw_crafting_screen(map, renderer),
            Mode::Inventory => todo!(),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::TitleScreen => write!(f, "v0.0.1"),
            Mode::Playing => write!(f, "Playing"),
            Mode::Inventory => write!(f, "Inventory"),
            Mode::Crafting => write!(f, "Crafting"),
        }
    }
}
