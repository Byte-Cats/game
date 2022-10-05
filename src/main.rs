use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_discord_presence::config::{RPCConfig, RPCPlugin};

pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
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
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(RPCPlugin(RPCConfig {
            app_id: 388338871475240965,
            show_time: true,
        }))
        .run();
}


fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.left = -1.0 *RESOLUTION;
    camera.projection.right = 1.0 *RESOLUTION;
    camera.projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}
