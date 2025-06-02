use super::status_bar;
use crate::{
    game::mode::Mode,
    renderer::{color::Color, render::Render},
};

const TITLE: &str = "RUST & RUIN";
const TITLE_LINE: &str = "===========";
const INSTRUCTIONS: &str = "Press Enter to Start";
const QUIT_INFO: &str = "Press ESC to Quit";

pub fn draw_title_screen<R: Render>(renderer: &mut R) {
    let (screen_w, screen_h) = renderer.get_screen_size();
    let tile_size = renderer.get_tile_size();
    draw_title(screen_w, screen_h, tile_size, renderer);
    draw_instructions(screen_w, screen_h, tile_size, renderer);
    draw_decorations(screen_w, screen_h, tile_size, renderer);

    status_bar::draw_status_bar(Mode::TitleScreen, "pre-alpha", renderer);
}

fn draw_title<R: Render>(screen_w: f32, screen_h: f32, tile_size: f32, renderer: &mut R) {
    renderer.draw_text(
        (screen_w / 2.) - (renderer.get_text_width(TITLE) / 2.),
        (screen_h / 2.) - (2. * tile_size),
        TITLE,
        Color::White,
        Color::Black,
    );
    renderer.draw_text(
        (screen_w / 2.) - (renderer.get_text_width(TITLE_LINE) / 2.),
        (screen_h / 2.) - tile_size,
        TITLE_LINE,
        Color::Yellow,
        Color::Black,
    );
}

fn draw_instructions<R: Render>(screen_w: f32, screen_h: f32, tile_size: f32, renderer: &mut R) {
    renderer.draw_text(
        (screen_w / 2.) - (renderer.get_text_width(INSTRUCTIONS) / 2.),
        (screen_h / 2.) + (2. * tile_size),
        INSTRUCTIONS,
        Color::Green,
        Color::Black,
    );
    renderer.draw_text(
        (screen_w / 2.) - (renderer.get_text_width(QUIT_INFO) / 2.),
        (screen_h / 2.) + (4. * tile_size),
        QUIT_INFO,
        Color::DarkGrey,
        Color::Black,
    );
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
