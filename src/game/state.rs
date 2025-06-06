use super::{mode::Mode, player::Player, screen::Screen};
use crate::{
    events::{self, event::Event, event_listener::EventListener, input::InputEvent},
    maps::{building::Building, building_type::BuildingType, map::Map, tile::Tile},
    renderer::render::{Render, TILE_SIZE},
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
    /// [`GameState::new`] will create a new [`GameState`].
    pub fn new() -> Self {
        let mut game_map = Map::new();
        let crafting_bench_width = 121. / TILE_SIZE;
        let crafting_bench_height = 48. / TILE_SIZE;
        let crafting_bench = Building::new(
            BuildingType::CraftingBench,
            20.,
            20.,
            crafting_bench_width,
            crafting_bench_height,
        );

        game_map.add_building(crafting_bench);
        for x in 0..crafting_bench_height as usize {
            for y in 0..crafting_bench_height as usize {
                game_map.add_tile(Tile::Wall, 20. + x as f32, 20. + y as f32);
            }
        }

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
                self.player.update();
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
            Mode::TitleScreen => match Mode::TitleScreen.handle_input(input) {
                Event::Continue => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Playing => match Mode::Playing.handle_input(input) {
                Event::MovePlayerForward => self
                    .player
                    .move_player(((self.player.speed * frame_time), 0.), &self.map),
                Event::MovePlayerBackward => self
                    .player
                    .move_player((-(self.player.speed * frame_time), 0.), &self.map),
                Event::MovePlayerUp => self
                    .player
                    .move_player((0., -(self.player.speed * 0.75 * frame_time)), &self.map),
                Event::MovePlayerDown => self
                    .player
                    .move_player((0., (self.player.speed * 0.75 * frame_time)), &self.map),
                Event::LeftClicked { x, y } => {
                    if let Event::Craft = events::playing::handle_playing_click(&self.map, x, y) {
                        self.mode = Mode::Crafting
                    }
                }
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Crafting => match Mode::Crafting.handle_input(input) {
                Event::Back => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Inventory => todo!(),
        };
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
