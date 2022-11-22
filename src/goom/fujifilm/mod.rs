use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::Commands;
use bevy::render::camera::ScalingMode;
use bevy::window;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const SCALE_FACTOR: f32 = 1.0;
pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

// Spawning the mmm mmm classic rpg camera
pub fn fujifilm(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}

// function to set CLEAR_COLOR
pub fn set_clear_color(color: Color) {
    CLEAR_COLOR = color;
}

// function to choose clear color based on int argument
pub fn choose_clear_color(i8: i) {
    match i {
        0 => set_clear_color(Color::rgb(0.1, 0.1, 0.1)),
        1 => set_clear_color(Color::rgb(0.2, 0.2, 0.2)),
        2 => set_clear_color(Color::rgb(0.3, 0.3, 0.3)),
        3 => set_clear_color(Color::rgb(0.4, 0.4, 0.4)),
        4 => set_clear_color(Color::rgb(0.5, 0.5, 0.5)),
        5 => set_clear_color(Color::rgb(0.6, 0.6, 0.6)),
        6 => set_clear_color(Color::rgb(0.7, 0.7, 0.7)),
        7 => set_clear_color(Color::rgb(0.8, 0.8, 0.8)),
        8 => set_clear_color(Color::rgb(0.9, 0.9, 0.9)),
        9 => set_clear_color(Color::rgb(1.0, 1.0, 1.0)),
        _ => set_clear_color(Color::rgb(0.1, 0.1, 0.1)),
    }
}

// function to set window scale factor
fn set_scale_factor(scale_factor: f32) {
    // set the scale factor
    SCALE_FACTOR = scale_factor;
}
fn set_resolution(resolution: f32) {
    // set the resolution
    RESOLUTION = resolution;
}
fn set_window_height(window_height: f32) {
    // set the window height
    WINDOW_HEIGHT = window_height;
}

// function to set new window size based on int argument
pub fn choose_window_size(i8: i) {
    // match the int argument
    match i {
        0 => set_window_size(640, 360),
        1 => set_window_size(800, 450),
        2 => set_window_size(960, 540),
        3 => set_window_size(1024, 576),
        4 => set_window_size(1280, 720),
        5 => set_window_size(1366, 768),
        6 => set_window_size(1600, 900),
        7 => set_window_size(1920, 1080),
        8 => set_window_size(2560, 1440),
        9 => set_window_size(3840, 2160),
        _ => set_window_size(640, 360),
    }
}
