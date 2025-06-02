use super::player::Player;
use crate::{
    events::{self, event_listener::EventListener},
    maps::map::Map,
    renderer::render::Render,
    ui::{self, interface::Interface},
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
            Mode::TitleScreen => ui::main_menu::draw_title_screen(renderer),
            Mode::Playing => ui::playing::draw_playing_screen(map, player, renderer),
            Mode::Crafting => ui::crafting::draw_crafting_screen(map, renderer),
            Mode::Inventory => todo!(),
        }
    }
}

impl EventListener for Mode {
    fn handle_input(&self, input: crate::events::input::InputEvent) -> crate::events::event::Event {
        match self {
            Mode::TitleScreen => events::title_screen::handle_title_screen_input(input),
            Mode::Playing => events::playing::handle_playing_input(input),
            Mode::Crafting => events::crafting::handle_crafting_input(input),
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
