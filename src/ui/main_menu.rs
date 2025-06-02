use super::status_bar;
use crate::{
    game::mode::Mode,
    renderer::{color::Color, render::Render},
};

pub fn draw_title_screen<R: Render>(renderer: &mut R) {
    let (screen_width, screen_height) = renderer.get_screen_size();
    let tile_size = renderer.get_tile_size();

    let title_line1 = "RUST & RUIN";
    let title_line2 = "===========";
    let instructions = "Press 'S' or Enter to Start";
    let quit_info = "Press ESC to Quit";

    // main title
    renderer.draw_text(
        (screen_width / 2.) - (renderer.get_text_width(title_line1) / 2.),
        (screen_height / 2.) - (2. * tile_size),
        title_line1,
        Color::White,
        Color::Black,
    );
    renderer.draw_text(
        (screen_width / 2.) - (renderer.get_text_width(title_line2) / 2.),
        (screen_height / 2.) - tile_size,
        title_line2,
        Color::Yellow,
        Color::Black,
    );

    // instructions
    renderer.draw_text(
        (screen_width / 2.) - (renderer.get_text_width(instructions) / 2.),
        (screen_height / 2.) + (2. * tile_size),
        instructions,
        Color::Green,
        Color::Black,
    );
    renderer.draw_text(
        (screen_width / 2.) - (renderer.get_text_width(quit_info) / 2.),
        (screen_height / 2.) + (4. * tile_size),
        quit_info,
        Color::DarkGrey,
        Color::Black,
    );

    // decorative elements
    for y_offset in 0..3 {
        renderer.draw_sprite(
            (screen_width / 2.) - ((renderer.get_text_width(title_line1) / 2.) - (4. * tile_size)),
            (screen_height / 2.) - ((2. + y_offset as f32) * tile_size),
            "title_char",
        );
        renderer.draw_sprite(
            (screen_width / 2.) + ((renderer.get_text_width(title_line1) / 2.) + (4. * tile_size)),
            (screen_height / 2.) - ((2. + y_offset as f32) * tile_size),
            "title_char",
        );
    }

    status_bar::draw_status_bar(Mode::TitleScreen, "", renderer);
}
