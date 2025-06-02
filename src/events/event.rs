/// [`Event`] rerpresents the result of an [`InputEvent`](super::input::InputEvent), and
/// typically informs the [`GameState`](crate::game::state::GameState) what to do next.
pub enum Event {
    Continue,
    Quit,
    MovePlayerForward,
    MovePlayerBackward,
    MovePlayerUp,
    MovePlayerDown,
    LeftClicked { x: f32, y: f32 },
    RightClicked { x: f32, y: f32 },
    Craft,
    Back,
    Nothing,
    Unknown,
}
