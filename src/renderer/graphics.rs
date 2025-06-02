use super::{color::Color, render::Render};
use crate::events::{input::InputEvent, mouse};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
// use std::collections::HashMap;

pub struct GraphicsRenderer {
    // textures: HashMap<String, Texture2D>,
    // sprite_rects: HashMap<String, Rect>,
    tile_size: f32,
}

impl GraphicsRenderer {
    pub fn new(tile_size: f32) -> Self {
        Self {
            // textures: HashMap::new(),
            // sprite_rects: HashMap::new(),
            tile_size,
        }
    }

    // [`GraphicsRenderer::load_graphics_assets`] will load all required graphics assets.
    pub async fn load_graphics_assets(&mut self) -> Result<(), macroquad::prelude::FileError> {
        // load a default font for `draw_char` and `draw_text`
        // required: .ttf font file in `assets/` directory
        // for example:
        // `assets/monogram.ttf`
        // let font_data = load_file("assets/monogram.ttf").await?;
        // self.font = Some(load_ttf_font_from_bytes(font_data.as_slice())?);

        // load a sprite sheet if you're using one for sprites
        // e.g., let sheet = load_texture("assets/spritesheet.png").await?;
        // self.textures.insert("main_sheet".to_string(), sheet);
        // self.sprite_rects.insert("player".to_string(), Rect::new(0.0, 0.0, 16.0, 16.0));
        // ... define more sprite regions

        Ok(())
    }

    fn map_color_to_macroquad(&self, color: Color) -> macroquad::prelude::Color {
        match color {
            Color::Black => BLACK,
            Color::White => WHITE,
            Color::Red => RED,
            Color::Green => GREEN,
            Color::Blue => BLUE,
            Color::Yellow => YELLOW,
            Color::Magenta => MAGENTA,
            Color::DarkGrey => DARKGRAY,
            // TODO: find out how to implement cyan and brown
            _ => WHITE,
        }
    }
}

impl Render for GraphicsRenderer {
    fn clear_screen(&mut self) {
        clear_background(BLACK);
    }

    fn draw_char(&mut self, x: f32, y: f32, character: char, fg_color: Color) {
        let mq_fg_color = self.map_color_to_macroquad(fg_color);

        // draw the character (pixel font), for now use macroquad's default font
        draw_text(
            &character.to_string(),
            x * self.tile_size,
            y * self.tile_size + self.tile_size,
            self.tile_size,
            mq_fg_color,
        );
    }

    fn draw_sprite(&mut self, x: f32, y: f32, _sprite_id: &str) {
        // draw textures from your sprite sheet based on `sprite_id`
        self.draw_char(x, y, 'S', Color::Magenta);
        // TODO: real implementation... something like:
        /*
        if let Some(texture) = self.textures.get("main_sheet") {
            if let Some(rect) = self.sprite_rects.get(sprite_id) {
                draw_texture_ex(
                    *texture,
                    x as f32 * self.tile_size,
                    y as f32 * self.tile_size,
                    WHITE, // Apply texture color white
                    DrawTextureParams {
                        source: Some(*rect),
                        dest_size: Some(vec2(self.tile_size, self.tile_size)),
                        ..Default::default()
                    },
                );
            } else {
                self.draw_char(x,y, '?', Color::Red, Color::Black); // Sprite not found
            }
        } else {
            self.draw_char(x,y, '?', Color::Red, Color::Black); // Sheet not loaded
        }
        */
    }

    fn draw_text(&mut self, x: f32, y: f32, text: &str, fg_color: Color, bg_color: Color) {
        let mq_fg_color = self.map_color_to_macroquad(fg_color);
        let mq_bg_color = self.map_color_to_macroquad(bg_color);
        draw_rectangle(x, y, self.tile_size, self.tile_size, mq_bg_color);
        draw_text(text, x, y, self.tile_size, mq_fg_color);
    }

    fn draw_menu(&mut self, width: f32, height: f32, title: &str, description: &str) {
        let menu_width = self.tile_size * width;
        let menu_height = self.tile_size * height;
        widgets::Window::new(
            hash!(),
            vec2(self.tile_size * 2., self.tile_size * 2.),
            vec2(menu_width, menu_height),
        )
        .label(title)
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            ui.label(Vec2::new(self.tile_size, self.tile_size), description);
            ui.button(
                Vec2::new(self.tile_size, menu_height - self.tile_size * 3.),
                "close",
            );
        });
    }

    fn get_screen_size(&self) -> (f32, f32) {
        (
            macroquad::window::screen_width(),
            macroquad::window::screen_height(),
        )
    }

    fn get_tile_size(&self) -> f32 {
        self.tile_size
    }

    fn get_text_width(&self, text: &str) -> f32 {
        (text.len() as f32 * self.tile_size) / 2.
    }

    fn poll_input(&mut self) -> Vec<InputEvent> {
        let mut events: Vec<InputEvent> = Vec::new();

        // known keyboard character input
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            events.push(InputEvent::Up);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            events.push(InputEvent::Down);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            events.push(InputEvent::Left);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            events.push(InputEvent::Right);
        }
        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
            events.push(InputEvent::Interact);
        }
        if is_key_pressed(KeyCode::Escape) {
            events.push(InputEvent::Quit);
        }

        // general keyboard character input
        if let Some(c) = get_char_pressed() {
            // filter out non-control chars
            if c.is_alphanumeric() {
                events.push(InputEvent::Character(c.to_ascii_lowercase()));
            }
        }

        // mouse input
        let (mouse_x_pixels, mouse_y_pixels) = mouse_position();
        let (screen_width_chars, screen_height_chars) = self.get_screen_size();
        let tile_size = self.get_tile_size();
        if mouse_x_pixels < screen_width_chars && mouse_y_pixels < screen_height_chars {
            if is_mouse_button_pressed(MouseButton::Left) {
                events.push(InputEvent::MouseClick {
                    x: mouse_x_pixels / tile_size,
                    y: mouse_y_pixels / tile_size,
                    button: mouse::MouseButton::Left,
                });
            }
            if is_mouse_button_pressed(MouseButton::Right) {
                events.push(InputEvent::MouseClick {
                    x: mouse_x_pixels / tile_size,
                    y: mouse_y_pixels / tile_size,
                    button: mouse::MouseButton::Right,
                });
            }
            if is_mouse_button_pressed(MouseButton::Middle) {
                events.push(InputEvent::MouseClick {
                    x: mouse_x_pixels / tile_size,
                    y: mouse_y_pixels / tile_size,
                    button: mouse::MouseButton::Middle,
                });
            }
        }

        events
    }
}
