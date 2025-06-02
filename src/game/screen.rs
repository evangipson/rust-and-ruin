use super::{mode::Mode, state::GameState};
use crate::{
    events::{self, event::Event, input::InputEvent},
    renderer::render::Render,
    ui,
};

/// [`Screen`] will be implemented to handle drawing a screen and handling that screen's input.
pub trait Screen {
    fn draw_screen<R: Render>(&self, renderer: &mut R);
    fn handle_input(&mut self, input: InputEvent);
}

impl Screen for GameState {
    fn draw_screen<R: Render>(&self, renderer: &mut R) {
        match self.mode {
            Mode::TitleScreen => ui::main_menu::draw_title_screen(renderer),
            Mode::Playing => ui::playing::draw_playing_screen(&self.map, &self.player, renderer),
            Mode::Inventory => todo!(),
            Mode::Crafting => ui::crafting::draw_crafting_screen(renderer),
        }
    }

    fn handle_input(&mut self, input: InputEvent) {
        match self.mode {
            Mode::TitleScreen => match events::title_screen::handle_title_screen_input(input) {
                Event::Continue => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Playing => match events::playing::handle_playing_input(input) {
                Event::MovePlayerForward => self.player.x += 1.,
                Event::MovePlayerBackward => self.player.x -= 1.,
                Event::MovePlayerUp => self.player.y -= 1.,
                Event::MovePlayerDown => self.player.y += 1.,
                Event::LeftClicked { x, y } => {
                    if let Event::Craft = events::playing::handle_playing_click(&self.map, x, y) {
                        self.mode = Mode::Crafting
                    }
                }
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            Mode::Crafting => match events::crafting::handle_crafting_input(input) {
                Event::Back => self.mode = Mode::Playing,
                Event::Quit => self.quit_game = true,
                _ => {}
            },
            _ => todo!(),
        };
    }
}
