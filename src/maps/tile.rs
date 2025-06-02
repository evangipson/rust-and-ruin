/// [`Tile`] represents a discrete section of a [`Map`](super::map::Map).
#[derive(Clone, Copy)]
pub enum Tile {
    /// [`Tile::Floor`] represents a passable area of a [`Map`](super::map::Map).
    Floor,
    /// [`Tile::Wall`] represents an impassable area of a [`Map`](super::map::Map).
    Wall,
}
