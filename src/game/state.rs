use super::{mode::Mode, player::Player, screen::Screen};
use crate::{
    events::{self, event::Event, input::InputEvent},
    maps::{
        building::{Building, BuildingType},
        map::Map,
    },
    renderer::render::Render,
    ui::interface::Interface,
};

/// [`GameState`] holds all stateful information about the game.
pub struct GameState {
    pub mode: Mode,
    pub quit_game: bool,
    pub player: Player,
    pub map: Map,
}

impl GameState {
    pub fn new() -> Self {
        let mut game_map = Map::new();
        game_map.add_building(Building::new(BuildingType::CraftingBench, 20., 20., 3., 1.));

        Self {
            mode: Mode::TitleScreen,
            quit_game: false,
            player: Player::new(10., 10.),
            map: game_map,
        }
    }

    /// [`GameState::update`] will run every tick to update the state of the game.
    pub fn update(&mut self) {
        match self.mode {
            Mode::TitleScreen => {}
            Mode::Playing => {
                // update player, enemies, world events etc.
                // TODO: player gain XP to show status bar will change
                // self.player.gain_xp(1);
            }
            _ => {}
        }
    }
}

impl Screen for GameState {
    fn draw_screen<R: Render>(&self, renderer: &mut R) {
        self.mode.draw_interface(&self.map, &self.player, renderer);
    }

    fn handle_input(&mut self, input: InputEvent, frame_time: f32) {
        match self.mode {
            Mode::TitleScreen => match events::title_screen::handle_title_screen_input(input) {
                Event::Continue => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Playing => match events::playing::handle_playing_input(input) {
                Event::MovePlayerForward => self.player.x += self.player.speed * frame_time,
                Event::MovePlayerBackward => self.player.x -= self.player.speed * frame_time,
                Event::MovePlayerUp => self.player.y -= self.player.speed * frame_time,
                Event::MovePlayerDown => self.player.y += self.player.speed * frame_time,
                Event::LeftClicked { x, y } => {
                    if let Event::Craft = events::playing::handle_playing_click(&self.map, x, y) {
                        self.mode = Mode::Crafting
                    }
                }
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Crafting => match events::crafting::handle_crafting_input(input) {
                Event::Back => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            _ => todo!(),
        };
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
