use bevy::prelude::*;

mod fujifilm;
use fujifilm::*;

mod spreet;
use spreet::*;

mod discord;
use discord::discord;

pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const WINDOW_HEIGHT: f32 = 600.0;

// Code for gooming
pub fn goom() {

    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Byte Cats".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            width: WINDOW_HEIGHT* RESOLUTION,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(fujifilm)
        .add_startup_system(spawn_sprite)
        .add_plugins(DefaultPlugins)
        .add_plugin(discord())

        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite_sheet)
        .run();
}


