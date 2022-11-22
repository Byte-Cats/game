use bevy::prelude::*;

mod controller;
use controller::keyboard_input_system;

mod fujifilm;
use fujifilm::*;

mod spreet;
use spreet::*;

mod discord;

mod ui;

use discord::discord;

// function to get the user gooming
pub fn goom() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Byte Cats".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            width: WINDOW_HEIGHT * RESOLUTION,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        // Init Default Plugins
        .add_plugins(DefaultPlugins)
        // Keyboard input
        .add_system(keyboard_input_system)
        // 2D Camera for the game
        .add_startup_system(fujifilm)
        .add_startup_system(spawn_sprite)
        // Discord Rich Presence Plugin
        .add_plugin(discord())
        // Preload textures for sprites from the asset server (spreet)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite_sheet)
        .run();
}
