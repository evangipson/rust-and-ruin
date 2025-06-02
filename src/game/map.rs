use crate::renderer::{color::Color, render::Render};

const MAX_MAP_WIDTH: usize = 255;
const MAX_MAP_HEIGHT: usize = 255;
const MAX_MAP_BUILDINGS: usize = 50;

#[derive(Clone, Copy)]
pub enum Tile {
    Floor,
    Wall,
}

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

/// [`Map`] represents an area in the game.
pub struct Map {
    pub tiles: [[Tile; MAX_MAP_WIDTH]; MAX_MAP_HEIGHT],
    pub buildings: [Building; MAX_MAP_BUILDINGS],
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Floor; MAX_MAP_WIDTH]; MAX_MAP_HEIGHT],
            buildings: [Building::default(); MAX_MAP_BUILDINGS],
        }
    }

    pub fn add_building(&mut self, building: Building) -> bool {
        // find the index of the first default building
        if let Some(index) = self
            .buildings
            .iter() // Use .iter() to get immutable references
            .position(|b| b.building_type == BuildingType::Default)
        {
            // if an index is found, replace the element at that index
            self.buildings[index] = building;
            true
        } else {
            // no default building found, array is full
            println!("Map: No space available for new building!");
            false
        }
    }

    pub fn get_building(&self, x: f32, y: f32) -> Option<Building> {
        self.buildings.into_iter().find(|b| {
            let within_horizontal_bounds =
                x.floor() >= b.x.floor() && x.floor() <= b.x.floor() + b.width.floor();
            let within_vertical_bounds =
                y.floor() >= b.y.floor() && y.floor() <= b.y.floor() + b.height.floor();
            within_horizontal_bounds && within_vertical_bounds
        })
    }

    pub fn add_tile(&mut self, tile: Tile, x: f32, y: f32) {
        self.tiles[x as usize][y as usize] = tile;
    }

    pub fn draw_map<R: Render>(&self, renderer: &mut R) {
        self.draw_tiles(renderer);
        self.draw_buildings(renderer);
    }

    fn draw_tiles<R: Render>(&self, renderer: &mut R) {
        let horizontal_tiles_to_draw =
            (renderer.get_screen_size().0 / renderer.get_tile_size()) as usize;
        let vertical_tiles_to_draw =
            (renderer.get_screen_size().1 / renderer.get_tile_size()) as usize;
        for x in 0..horizontal_tiles_to_draw {
            for y in 0..vertical_tiles_to_draw {
                let (char_to_draw, color) = match self.tiles[x][y] {
                    Tile::Floor => ('.', Color::DarkGrey),
                    Tile::Wall => ('#', Color::White),
                };
                renderer.draw_char(x as f32, y as f32, char_to_draw, color, Color::Black);
            }
        }
    }

    fn draw_buildings<R: Render>(&self, renderer: &mut R) {
        self.buildings
            .into_iter()
            .filter(|b| b.building_type != BuildingType::Default)
            .for_each(|b| {
                for x in 0..b.width as usize {
                    for y in 0..b.height as usize {
                        renderer.draw_char(
                            b.x + x as f32,
                            b.y + y as f32,
                            b.building_type.get_char(),
                            Color::Yellow,
                            Color::Black,
                        )
                    }
                }
            });
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}
