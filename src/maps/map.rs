use super::{building::Building, building_type::BuildingType, tile::Tile};
use crate::renderer::{color::Color, render::Render};

const MAX_MAP_WIDTH: usize = 255;
const MAX_MAP_HEIGHT: usize = 255;
const MAX_MAP_BUILDINGS: usize = 50;

/// [`Map`] represents an area in the game.
pub struct Map {
    /// [`Map::tiles`] is a collection of discrete areas of a [`Map`].
    pub tiles: [[Tile; MAX_MAP_WIDTH]; MAX_MAP_HEIGHT],
    /// [`Map::buildings`] is a collection of buildings in a [`Map`].
    pub buildings: [Building; MAX_MAP_BUILDINGS],
}

impl Map {
    /// [`Map::new`] creates a new [`Map`] with default [`Map::tiles`]
    /// and an empty collection of [`Map::buildings`].
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Floor; MAX_MAP_WIDTH]; MAX_MAP_HEIGHT],
            buildings: [Building::default(); MAX_MAP_BUILDINGS],
        }
    }

    /// [`Map::add_building`] will attempt to add a `building` to a [`Map`],
    /// and return the result of the attempt.
    pub fn add_building(&mut self, building: Building) -> bool {
        if let Some(index) = self
            .buildings
            .iter()
            .position(|b| b.building_type == BuildingType::Default)
        {
            self.buildings[index] = building;
            true
        } else {
            println!("Map: No space available for new building!");
            false
        }
    }

    /// [`Map::get_building`] will query a set of (`x`, `y`) coordinates of a [`Map`]
    /// and return [`Some`] [`Building`] if one is found, and [`None`] otherwise.
    pub fn get_building(&self, x: f32, y: f32) -> Option<Building> {
        self.buildings.into_iter().find(|b| {
            let within_horizontal_bounds =
                x.floor() >= b.x.floor() && x.floor() <= b.x.floor() + b.width.floor();
            let within_vertical_bounds =
                y.floor() >= b.y.floor() && y.floor() <= b.y.floor() + b.height.floor();
            within_horizontal_bounds && within_vertical_bounds
        })
    }

    /// [`Map::add_building`] will add a [`Tile`] to an (`x`, `y`) coordinate
    /// of a [`Map`].
    pub fn add_tile(&mut self, tile: Tile, x: f32, y: f32) {
        self.tiles[x as usize][y as usize] = tile;
    }

    /// [`Map::draw_map`] will render a map using a [`Render`] implementation.
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
                let (char_to_draw, fg_color, bg_color) = match self.tiles[x][y] {
                    Tile::Floor => (' ', Color::DarkGrey, Color::Green),
                    Tile::Wall => (' ', Color::White, Color::Black),
                };
                renderer.draw_char(x as f32, y as f32, char_to_draw, fg_color, bg_color);
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
                            Color::Transparent,
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
