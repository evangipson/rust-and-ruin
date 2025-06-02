use super::building_type::BuildingType;

/// [`Building`] represents different buildings and their display properties in the game.
#[derive(Clone, Copy, PartialEq)]
pub struct Building {
    /// [`Building::building_type`] is the [`BuildingType`] of a [`Building`].
    pub building_type: BuildingType,
    /// [`Building::x`] is the horizontal location of a [`Building`].
    pub x: f32,
    /// [`Building::y`] is the vertical location of a [`Building`].
    pub y: f32,
    /// [`Building::width`] is the horizontal real estate of a [`Building`].
    pub width: f32,
    /// [`Building::height`] is the vertical real estate of a [`Building`].
    pub height: f32,
}

impl Building {
    /// [`Building::new`] creates a new [`Building`].
    pub fn new(building_type: BuildingType, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            building_type,
            x,
            y,
            width,
            height,
        }
    }
}

impl Default for Building {
    fn default() -> Self {
        Building::new(
            BuildingType::Default,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}
