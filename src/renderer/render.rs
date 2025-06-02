use super::color::Color;
use crate::events::input::InputEvent;

/// [`Render`] will be implemented to draw things on the game screen.
pub trait Render {
    /// [`Render::clear_screen`] flushes the screen with [`Color::Black`].
    fn clear_screen(&mut self);

    /// [`Render::draw_char`] draws a character on the screen.
    fn draw_char(&mut self, x: f32, y: f32, character: char, fg_color: Color, bg_color: Color);

    /// [`Render::draw_sprite`] draws a sprite on the screen by `sprite_id`.
    fn draw_sprite(&mut self, x: f32, y: f32, sprite_id: &str);

    /// [`Render::draw_text`] draws some `text` on the screen.
    fn draw_text(&mut self, x: f32, y: f32, text: &str, fg_color: Color, bg_color: Color);

    /// [`Render::draw_centered_text`] draws some `text` centered on the screen.
    fn draw_centered_text(
        &mut self,
        text: &str,
        color: Color,
        screen_width: f32,
        screen_height: f32,
        y_offset: f32,
        tile_size: f32,
    );

    /// [`Render::draw_menu`] draws a menu with `title` and `description` on the screen.
    fn draw_menu(&mut self, width: f32, height: f32, title: &str, description: &str);

    /// [`Render::poll_input`] returns a collection of [`InputEvent`] based on input
    /// to the screen.
    fn poll_input(&mut self) -> Vec<InputEvent>;

    /// [`Render::get_screen_size`] returns two measurements of the screen in order:
    /// the width and the height.
    fn get_screen_size(&self) -> (f32, f32);

    /// [`Render::get_tile_size`] returns the size of a tile on the screen.
    fn get_tile_size(&self) -> f32;

    /// [`Render::get_text_width`] returns the width of some `text` on the screen.
    fn get_text_width(&self, text: &str) -> f32;
}
