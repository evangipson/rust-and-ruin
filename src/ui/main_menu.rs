use super::status_bar;
use crate::{
    game::mode::Mode,
    renderer::{color::Color, render::Render},
};
use macroquad::{
    color::WHITE,
    math::vec2,
    prelude::{Material, gl_use_default_material, gl_use_material},
    texture::{DrawTextureParams, FilterMode, draw_texture_ex, render_target},
};

const TITLE: &str = "RUST & RUIN";
const TITLE_LINE: &str = "===========";
const INSTRUCTIONS: &str = "Press Enter to Start";
const QUIT_INFO: &str = "Press ESC to Quit";

/// [`draw_title_screen`] will draw the title screen using a [`Render`] implementation.
pub fn draw_title_screen<R: Render>(renderer: &mut R) {
    let (w, h) = renderer.get_screen_size();
    let tile_size = renderer.get_tile_size();
    let starfield_shader_material = renderer.get_shader_material(0).unwrap();
    draw_starfield_shader(starfield_shader_material, w, h);
    renderer.draw_centered_text(TITLE, Color::White, w, h, -2., tile_size);
    renderer.draw_centered_text(TITLE_LINE, Color::Yellow, w, h, -1., tile_size);
    renderer.draw_centered_text(INSTRUCTIONS, Color::Green, w, h, 2., tile_size);
    renderer.draw_centered_text(QUIT_INFO, Color::DarkGrey, w, h, 4., tile_size);
    draw_decorations(w, h, tile_size, renderer);

    status_bar::draw_status_bar(Mode::TitleScreen, "pre-alpha", renderer);
}

fn draw_starfield_shader(material: Material, screen_w: f32, screen_h: f32) {
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    material.set_uniform("iResolution", (screen_w, screen_h));
    material.set_uniform("direction_modifier", 0f32);
    gl_use_material(material);
    draw_texture_ex(
        render_target.texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_w, screen_h)),
            ..Default::default()
        },
    );
    gl_use_default_material();
}

fn draw_decorations<R: Render>(screen_w: f32, screen_h: f32, tile_size: f32, renderer: &mut R) {
    for y_offset in 0..3 {
        renderer.draw_sprite(
            (screen_w / 2.) - ((renderer.get_text_width(TITLE) / 2.) - (4. * tile_size)),
            (screen_h / 2.) - ((2. + y_offset as f32) * tile_size),
            "title_char",
        );
        renderer.draw_sprite(
            (screen_w / 2.) + ((renderer.get_text_width(TITLE) / 2.) + (4. * tile_size)),
            (screen_h / 2.) - ((2. + y_offset as f32) * tile_size),
            "title_char",
        );
    }
}
