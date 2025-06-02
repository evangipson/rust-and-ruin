use crate::renderer::{color::Color, render::Render};

/// [`Player`] is the main character of the game.
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub character: char,
    pub color: Color,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            character: '@',
            color: Color::White,
        }
    }

    pub fn draw_player<R: Render>(&self, renderer: &mut R) {
        renderer.draw_char(self.x, self.y, self.character, self.color, Color::Black);
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Default::default(), Default::default())
    }
}
