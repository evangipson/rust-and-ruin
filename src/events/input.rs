use super::mouse::MouseButton;

/// [`InputEvent`] represents player input to the game, and usually will result in an
/// [`Event`](super::event::Event) to modify the [`GameState`](crate::game::state::GameState).
pub enum InputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
    Escape,
    Quit,
    Character(char),
    MouseClick { x: f32, y: f32, button: MouseButton },
    MouseMove { x: f32, y: f32 },
    // add more specific keys as needed:
    // tab, enter, space, delete, backspace, f keys (u8), ctrl, alt, shift...
    Unknown,
}
