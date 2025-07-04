use super::{event::Event, input::InputEvent};
use crate::{
    game::player::Player,
    maps::{building_type::BuildingType, map::Map, tile::Tile},
};

pub fn handle_playing_input(event: InputEvent) -> Event {
    match event {
        InputEvent::Quit => Event::Quit,
        InputEvent::Left => Event::MovePlayerBackward,
        InputEvent::Right => Event::MovePlayerForward,
        InputEvent::Up => Event::MovePlayerUp,
        InputEvent::Down => Event::MovePlayerDown,
        InputEvent::MouseClick {
            x,
            y,
            button: super::mouse::MouseButton::Left,
        } => Event::LeftClicked { x, y },
        InputEvent::MouseClick {
            x,
            y,
            button: super::mouse::MouseButton::Right,
        } => Event::RightClicked { x, y },
        _ => Event::Nothing,
    }
}

pub fn handle_playing_click(map: &Map, mouse_x: f32, mouse_y: f32) -> Event {
    if let Some(building) = map.get_building(mouse_x, mouse_y) {
        match building.building_type {
            BuildingType::CraftingBench => Event::Craft,
            _ => Event::Nothing,
        }
    } else {
        Event::Nothing
    }
}

pub fn move_player(player: &mut Player, map: &Map, frame_time: f32, new_position: (f32, f32)) {
    if let Some(Tile::Floor) = map.get_tile(new_position.0, new_position.1) {
        player.x += player.speed * frame_time
    }
}
