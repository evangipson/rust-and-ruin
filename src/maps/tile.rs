/// [`Tile`] represents a discrete section of a [`Map`](super::map::Map).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    /// [`Tile::Floor`] represents a passable area of a [`Map`](super::map::Map).
    Floor,
    /// [`Tile::Wall`] represents an impassable area of a [`Map`](super::map::Map).
    Wall,
    /// [`Tile::Building`] represents an impassable area of a [`Building`](super::building::Building).
    Building,
}
