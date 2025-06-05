use super::{color, render::Render};
use crate::events::{input::InputEvent, mouse};
use crate::shaders::starfield;
use macroquad::color::Color;
use macroquad::prelude::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use std::collections::HashMap;

/// [`GraphicsRenderer`] is an implementation of [`Render`] that draws basic 2d graphics.
pub struct GraphicsRenderer {
    textures: HashMap<String, Texture2D>,
    sprite_rects: HashMap<String, Rect>,
    animations: HashMap<String, AnimatedSprite>,
    tile_size: f32,
    shaders: [Material; 1],
}

impl GraphicsRenderer {
    pub fn new(tile_size: f32) -> Self {
        Self {
            textures: HashMap::new(),
            sprite_rects: HashMap::new(),
            animations: HashMap::new(),
            tile_size,
            shaders: [starfield::create_starfield_shader()],
        }
    }

    // [`load_graphics_assets`] will load all required graphics assets.
    pub async fn load_graphics_assets(&mut self) -> Result<(), macroquad::prelude::FileError> {
        // load a default font for `draw_char` and `draw_text`
        // required: .ttf font file in `assets/` directory
        // for example:
        // `assets/monogram.ttf`
        // let font_data = load_file("assets/monogram.ttf").await?;
        // self.font = Some(load_ttf_font_from_bytes(font_data.as_slice())?);

        let main_sheet = load_texture("assets/sprites/main_sheet.png").await?;
        self.textures.insert("main_sheet".to_string(), main_sheet);

        let character_walk = load_texture("assets/sprites/character_walk.png").await?;
        self.textures
            .insert("character_walk".to_string(), character_walk);

        self.sprite_rects
            .insert("crafting_bench".to_string(), Rect::new(0., 0., 121., 48.));
        self.sprite_rects.insert(
            "player_base".to_string(),
            self.map_sprite_coords_to_rect(0., 49., 26., 62., 3, 2),
        );
        self.sprite_rects.insert(
            "player_back".to_string(),
            self.map_sprite_coords_to_rect(0., 49., 26., 62., 1, 2),
        );
        self.sprite_rects.insert(
            "player_left".to_string(),
            self.map_sprite_coords_to_rect(0., 49., 26., 62., 4, 2),
        );
        self.sprite_rects.insert(
            "player_right".to_string(),
            self.map_sprite_coords_to_rect(0., 49., 26., 62., 2, 2),
        );
        self.sprite_rects.insert(
            "player_interact".to_string(),
            self.map_sprite_coords_to_rect(0., 49., 26., 62., 3, 2),
        );

        self.animations.insert(
            "character_walk".to_string(),
            AnimatedSprite::new(
                48,
                64,
                &[
                    Animation {
                        name: "player_walk".to_string(),
                        row: 2,
                        frames: 3,
                        fps: 5,
                    },
                    Animation {
                        name: "player_walk_left".to_string(),
                        row: 3,
                        frames: 3,
                        fps: 5,
                    },
                    Animation {
                        name: "player_walk_right".to_string(),
                        row: 1,
                        frames: 3,
                        fps: 5,
                    },
                    Animation {
                        name: "player_walk_up".to_string(),
                        row: 0,
                        frames: 3,
                        fps: 5,
                    },
                ],
                false,
            ),
        );

        Ok(())
    }

    fn map_sprite_coords_to_rect(
        &self,
        start_x: f32,
        start_y: f32,
        w: f32,
        h: f32,
        row: u8,
        col: u8,
    ) -> Rect {
        Rect::new(
            start_x + ((col as f32 - 1.) * w),
            start_y + ((row as f32 - 1.) * h),
            w,
            h,
        )
    }

    fn map_animation_to_index(&self, animation: &str) -> usize {
        match animation {
            "player_walk" => 0,
            "player_walk_left" => 1,
            "player_walk_right" => 2,
            "player_walk_up" => 3,
            _ => 0,
        }
    }

    fn map_color_to_macroquad(&self, color: color::Color) -> macroquad::prelude::Color {
        match color {
            color::Color::Black => BLACK,
            color::Color::White => WHITE,
            color::Color::Red => RED,
            color::Color::Green => DARKGREEN,
            color::Color::Blue => DARKBLUE,
            color::Color::Yellow => GOLD,
            color::Color::Magenta => MAGENTA,
            color::Color::DarkGrey => DARKGRAY,
            color::Color::Brown => Color::new(0., 1., 1., 1.),
            color::Color::Cyan => Color::new(0.54, 0.27, 0.07, 1.0),
            color::Color::Transparent => Color::new(0., 0., 0., 0.),
        }
    }
}

impl Render for GraphicsRenderer {
    fn clear_screen(&mut self) {
        clear_background(BLACK);
    }

    fn draw_char(
        &mut self,
        x: f32,
        y: f32,
        character: char,
        fg_color: color::Color,
        bg_color: color::Color,
    ) {
        let mq_fg_color = self.map_color_to_macroquad(fg_color);
        let mq_bg_color = self.map_color_to_macroquad(bg_color);
        draw_rectangle(
            x * self.tile_size,
            y * self.tile_size,
            self.tile_size,
            self.tile_size,
            mq_bg_color,
        );
        draw_text(
            &character.to_string(),
            x * self.tile_size,
            y * self.tile_size + self.tile_size,
            self.tile_size,
            mq_fg_color,
        );
    }

    // draw textures from the main sprite sheet based on `sprite_id`
    fn draw_sprite(&mut self, x: f32, y: f32, sprite_id: &str) {
        if let Some(texture) = self.textures.get("main_sheet") {
            if let Some(rect) = self.sprite_rects.get(sprite_id) {
                draw_texture_ex(
                    *texture,
                    x * self.tile_size,
                    y * self.tile_size,
                    WHITE,
                    DrawTextureParams {
                        source: Some(*rect),
                        ..Default::default()
                    },
                );
            } else {
                // sprite not found
                self.draw_char(x, y, '?', color::Color::Red, color::Color::Black);
            }
        } else {
            // sheet not loaded
            self.draw_char(x, y, '?', color::Color::Red, color::Color::Black);
        }
    }

    fn draw_animation(&mut self, x: f32, y: f32, sheet_id: &str, animation: &str) {
        let animation_index = self.map_animation_to_index(animation);
        let animation_sprite = self.animations.get_mut(sheet_id).unwrap();
        if animation_sprite.current_animation() != animation_index {
            animation_sprite.set_animation(animation_index);
            animation_sprite.playing = true;
        }
        animation_sprite.update();
        if let Some(animation) = self.animations.get(sheet_id)
            && let Some(texture) = self.textures.get(sheet_id)
        {
            draw_texture_ex(
                *texture,
                x * self.tile_size,
                y * self.tile_size,
                WHITE,
                DrawTextureParams {
                    source: Some(animation.frame().source_rect),
                    ..Default::default()
                },
            );
        };
    }

    fn stop_animation(&mut self, sheet_id: &str) {
        let animated_sprite = self.animations.get_mut(sheet_id).unwrap();
        animated_sprite.playing = false;
    }

    fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        fg_color: color::Color,
        bg_color: color::Color,
    ) {
        let mq_fg_color = self.map_color_to_macroquad(fg_color);
        let mq_bg_color = self.map_color_to_macroquad(bg_color);
        draw_rectangle(
            x,
            y - self.tile_size + (self.tile_size / 4.),
            self.get_text_width(text) - (self.tile_size),
            self.tile_size,
            mq_bg_color,
        );
        draw_text(
            text,
            x + (self.tile_size / 2.),
            y,
            self.tile_size,
            mq_fg_color,
        );
    }

    fn draw_centered_text(
        &mut self,
        text: &str,
        color: color::Color,
        screen_width: f32,
        screen_height: f32,
        y_offset: f32,
        tile_size: f32,
    ) {
        self.draw_text(
            (screen_width / 2.) - (self.get_text_width(text) / 2.),
            (screen_height / 2.) + (y_offset * tile_size),
            text,
            color,
            color::Color::Black,
        )
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

    fn get_shader_material(&self, shader_index: usize) -> Option<Material> {
        if self.shaders.len() >= shader_index {
            Some(self.shaders[shader_index])
        } else {
            None
        }
    }
}
