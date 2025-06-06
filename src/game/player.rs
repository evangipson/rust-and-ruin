use crate::{
    maps::{map::Map, tile::Tile},
    renderer::{
        color::Color,
        render::{Render, TILE_SIZE},
    },
};

const BASE_MOVEMENT_SPEED: f32 = 4.;
const PLAYER_WIDTH: f32 = 51.;
const PLAYER_HEIGHT: f32 = 93.;
const RELATIVE_PLAYER_WIDTH: f32 = PLAYER_WIDTH / TILE_SIZE;
const RELATIVE_PLAYER_HEIGHT: f32 = PLAYER_HEIGHT / TILE_SIZE;

/// [`Player`] is the main character of the game.
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub sprite_id: String,
    pub animation: String,
    pub color: Color,
    is_walking: bool,
    last_x: f32,
    last_y: f32,
}

impl Player {
    /// [`Player::new`] creates a new [`Player`].
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: BASE_MOVEMENT_SPEED,
            sprite_id: "player_base".to_owned(),
            animation: "player_walk".to_owned(),
            color: Color::White,
            is_walking: false,
            last_x: x,
            last_y: y,
        }
    }

    /// [`Player::update`] will be called every frame to update the [`Player`] state.
    pub fn update(&mut self) {
        self.is_walking = self.last_x != self.x || self.last_y != self.y;
        self.last_x = self.x;
        self.last_y = self.y;
    }

    /// [`Player::draw_player`] will draw a [`Player`] on the game screen using
    /// a [`Render`] implementation.
    pub fn draw_player<R: Render>(&self, renderer: &mut R) {
        if self.is_walking {
            renderer.draw_animation(self.x, self.y, "character_walk", &self.animation);
        } else {
            renderer.stop_animation("character_walk");
            renderer.draw_sprite(self.x, self.y, &self.sprite_id);
        }
    }

    /// [`Player::move_player`] will move a player, as long as the [`Tile`] they
    /// are moving to is a passable tile.
    pub fn move_player(&mut self, new_position: (f32, f32), map: &Map) {
        if let Some(tile) = map.get_tile(
            if new_position.0 < 0. {
                self.x + new_position.0
            } else {
                self.x + RELATIVE_PLAYER_WIDTH + new_position.0
            },
            if new_position.1 < 0. {
                self.y - RELATIVE_PLAYER_HEIGHT + new_position.1
            } else {
                self.y + new_position.1
            },
        ) && tile.eq(&Tile::Floor)
        {
            self.update_sprite(new_position);
            self.x += new_position.0;
            self.y += new_position.1;
        }
    }

    fn update_sprite(&mut self, new_position: (f32, f32)) {
        if new_position.1 < 0. && self.sprite_id != "player_back" {
            self.animation = "player_walk_up".to_string();
            self.sprite_id = "player_back".to_string();
        } else if new_position.0 > 0. && self.sprite_id != "player_right" {
            self.animation = "player_walk_right".to_string();
            self.sprite_id = "player_right".to_string();
        } else if new_position.0 < 0. && self.sprite_id != "player_left" {
            self.animation = "player_walk_left".to_string();
            self.sprite_id = "player_left".to_string();
        } else if new_position.0 == 0. && new_position.1 >= 0. && self.sprite_id != "player_base" {
            self.animation = "player_walk".to_string();
            self.sprite_id = "player_base".to_string();
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Default::default(), Default::default())
    }
}
