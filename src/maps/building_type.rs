/// [`BuildingType`] represents different buildings in the game.
#[derive(Clone, Copy, PartialEq)]
pub enum BuildingType {
    /// [`BuildingType::CraftingBench`] represents a crafting bench [`BuildingType`].
    CraftingBench,
    /// [`BuildingType::Default`] represents the default [`BuildingType`].
    Default,
}

impl BuildingType {
    /// [`BuildingType::get_char`] gets a [`char`] that represents a [`BuildingType`].
    pub fn get_char(&self) -> char {
        match self {
            BuildingType::CraftingBench => 'C',
            _ => '?',
        }
    }
}
