use crate::{game::player::Player, maps::map::Map, renderer::render::Render};

/// [`Interface`] will be implemented to draw an area on the game screen.
pub trait Interface {
    /// [`Interface::draw_interface`] will draw an area on the game screen using the
    /// provided [`Render`] implementation.
    fn draw_interface<R: Render>(&self, map: &Map, player: &Player, renderer: &mut R);
}
