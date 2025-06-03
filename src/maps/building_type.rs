/// [`BuildingType`] represents different buildings in the game.
#[derive(Clone, Copy, PartialEq)]
pub enum BuildingType {
    /// [`BuildingType::CraftingBench`] represents a crafting bench [`BuildingType`].
    CraftingBench,
    /// [`BuildingType::Default`] represents the default [`BuildingType`].
    Default,
}

impl BuildingType {
    /// [`BuildingType::get_sprite_id`] gets a [`&str`](str) that represents an id for
    /// a sprite.
    pub fn get_sprite_id(&self) -> &str {
        match self {
            BuildingType::CraftingBench => "crafting_bench",
            _ => "unknown",
        }
    }
}
