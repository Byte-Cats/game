use bevy::app::AppExit;
use bevy::prelude::*;

mod fujifilm;
use fujifilm::*;

mod spreet;
use spreet::*;

mod discord;
use discord::discord;

pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const WINDOW_HEIGHT: f32 = 600.0;
// add exit_key as  combo ctrl + q
pub const EXIT_KEY1: KeyCode = KeyCode::Q;
pub const EXIT_KEY2: KeyCode = KeyCode::LControl;

// Code for gooming
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
        // keybinding for exit key combo ctrl + q and apply within main process
        .add_system(keyboard_input_system)
        .add_startup_system(fujifilm)
        .add_startup_system(spawn_sprite)
        .add_plugins(DefaultPlugins)
        .add_plugin(discord())
        // If user presses ctrl + q, exit the process
        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite_sheet)
        .run();
}

/// This system checks key states each frame and does something if the specified key is pressed
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    // exit program if control + q is pressed
    // wait for key up event to prevent multiple exits
    if keyboard_input.pressed(EXIT_KEY2) && keyboard_input.just_pressed(EXIT_KEY1) {
        // message to console that program is exiting
        println!("Exiting program");
        // exit the process
        std::process::exit(0);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // pause the bevy game
        // spawn a pause menu
        println("Paused");
    }
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    // exit program
    exit.send(AppExit);
}
