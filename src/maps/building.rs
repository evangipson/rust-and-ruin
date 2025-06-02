#[derive(Clone, Copy, PartialEq)]
pub enum BuildingType {
    CraftingBench,
    Default,
}

impl BuildingType {
    pub fn get_char(&self) -> char {
        match self {
            Self::CraftingBench => 'C',
            _ => '?',
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Building {
    pub building_type: BuildingType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Building {
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
