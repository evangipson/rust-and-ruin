use super::{
    map::{Building, BuildingType, Map},
    mode::Mode,
    player::Player,
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

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
