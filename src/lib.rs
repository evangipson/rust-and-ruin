pub mod events {
    pub mod crafting;
    pub mod event;
    pub mod input;
    pub mod mouse;
    pub mod playing;
    pub mod title_screen;
}

pub mod game {
    pub mod map;
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

pub mod renderer {
    pub mod color;
    pub mod graphics;
    pub mod render;
}

pub mod ui {
    pub mod crafting;
    pub mod dialog;
    pub mod hud;
    pub mod inventory;
    pub mod main_menu;
    pub mod playing;
    pub mod status_bar;
}
