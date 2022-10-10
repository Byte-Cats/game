use bevy::prelude::*;

mod fujifilm;
use fujifilm::fujifilm;

mod discord;
use discord::discord;

// The code to start gooming
pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const DISCORD_APP_ID: u64 = 388338871475240965;

pub fn goom() {
    let height: f32 = 600.0;

    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Byte Cats".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            width: height * RESOLUTION,
            height,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(fujifilm())
        .add_plugins(DefaultPlugins)
        .add_plugin(discord())
        .run();
}


