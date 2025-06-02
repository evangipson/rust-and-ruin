use super::color::Color;
use crate::events::input::InputEvent;

/// [`Render`] will be implemented by a rendering system to draw the game.
pub trait Render {
    fn clear_screen(&mut self);
    fn draw_char(&mut self, x: f32, y: f32, character: char, fg_color: Color, bg_color: Color);
    fn draw_sprite(&mut self, x: f32, y: f32, sprite_id: &str);
    fn draw_text(&mut self, x: f32, y: f32, text: &str, fg_color: Color, bg_color: Color);
    fn draw_menu(&mut self, width: f32, height: f32, title: &str, description: &str);
    fn poll_input(&mut self) -> Vec<InputEvent>;
    fn get_screen_size(&self) -> (f32, f32);
    fn get_tile_size(&self) -> f32;
    fn get_text_width(&self, text: &str) -> f32;
}
