use crate::renderer::{color::Color, render::Render};

const BASE_MOVEMENT_SPEED: f32 = 3.;

/// [`Player`] is the main character of the game.
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub character: char,
    pub color: Color,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: BASE_MOVEMENT_SPEED,
            character: '@',
            color: Color::White,
        }
    }

    pub fn draw_player<R: Render>(&self, renderer: &mut R) {
        renderer.draw_char(
            self.x,
            self.y,
            self.character,
            self.color,
            Color::Transparent,
        );
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Default::default(), Default::default())
    }
}
