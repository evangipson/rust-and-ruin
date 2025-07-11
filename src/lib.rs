pub mod events {
    pub mod crafting;
    pub mod event;
    pub mod event_listener;
    pub mod input;
    pub mod mouse;
    pub mod playing;
    pub mod title_screen;
}

pub mod game {
    pub mod mode;
    pub mod player;
    pub mod screen;
    pub mod state;
}

pub mod items {
    pub mod currency;
    pub mod inventory;
    pub mod item;
}

pub mod maps {
    pub mod building;
    pub mod building_type;
    pub mod map;
    pub mod tile;
}

pub mod renderer {
    pub mod color;
    pub mod graphics;
    pub mod render;
}

pub mod shaders {
    pub mod starfield;
}

pub mod ui {
    pub mod crafting;
    pub mod dialog;
    pub mod hud;
    pub mod interface;
    pub mod inventory;
    pub mod main_menu;
    pub mod playing;
    pub mod status_bar;
}
