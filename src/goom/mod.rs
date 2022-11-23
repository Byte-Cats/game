use bevy::prelude::*;

use controller::keyboard_input_system;
use discord::discord;
use fujifilm::*;
use spreet::*;

mod controller;

mod fujifilm;

mod spreet;

mod discord;

mod player;
mod ui;

// function to get the user gooming
pub fn goom() {
    App::add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Byte Cats".to_string(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    width: WINDOW_HEIGHT * RESOLUTION,
                    height: WINDOW_HEIGHT,
                    resizable: false,
                    ..default()
                },
                ..default()
            })
            // Init Default Plugins
            // Keyboard input
            .add_system(keyboard_input_system)
            // 2D Camera for the game
            .add_startup_system(fujifilm)
            .add_startup_system(spawn_sprite)
            // Discord Rich Presence Plugin
            .add_plugin(discord())
            // Preload textures for sprites from the asset server (spreet)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite_sheet)
            .run(),
        (),
    );
}
