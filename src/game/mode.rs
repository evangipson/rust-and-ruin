/// [`Mode`] represents the current mode of the game.
pub enum Mode {
    TitleScreen,
    Playing,
    Inventory,
    Crafting,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::TitleScreen => write!(f, "v0.0.1"),
            Mode::Playing => write!(f, "Playing"),
            Mode::Inventory => write!(f, "Inventory"),
            Mode::Crafting => write!(f, "Crafting"),
        }
    }
}
