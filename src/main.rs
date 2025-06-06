use macroquad::time;
use macroquad::window::Conf;
use rust_and_ruin::game::screen::Screen;
use rust_and_ruin::renderer::graphics::GraphicsRenderer;
use rust_and_ruin::renderer::render::TILE_SIZE;
use rust_and_ruin::{game::state::GameState, renderer::render::Render};

// TODO: asset loading
// main game loop function, generic over any Render implementation
async fn run_game<R: Render>(mut renderer: R) -> Result<(), Box<dyn std::error::Error>> {
    let mut game_state = GameState::new();
    while !game_state.quit_game {
        // get the frame time to animate entities smoothly
        let delta_time = time::get_frame_time();

        // handle any input from the player
        renderer
            .poll_input()
            .into_iter()
            .for_each(|i| game_state.handle_input(i, delta_time));

        // update the game state
        game_state.update();

        // draw the game screen
        game_state.draw_screen(&mut renderer);

        // wait until the next frame is drawn
        macroquad::prelude::next_frame().await;
    }
    Ok(())
}

fn window_config() -> Conf {
    Conf {
        window_title: "Rust & Ruin".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut renderer = GraphicsRenderer::new(TILE_SIZE);
    if let Err(e) = renderer.load_graphics_assets().await {
        panic!("Rust & Ruin encountered an issue loading art assets: {e}");
    }
    if let Err(e) = run_game(renderer).await {
        panic!("Rust & Ruin encountered an error: {e}");
    }
}
