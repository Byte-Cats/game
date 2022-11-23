// use bevy::app::AppExit;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;

// add exit_key as  combo ctrl + q
pub const EXIT_KEY2: KeyCode = KeyCode::Q;
pub const EXIT_KEY1: KeyCode = KeyCode::LControl;

// function to detect if user is on mac returns true or false
pub fn using_mac(){
    // check if user is using mac osx
    println!("checking system type")
}

// This system checks key states each frame and does something if the specified key is pressed
pub(crate) fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    // exit program if control + q is pressed
    // wait for key up event to prevent multiple exits

    if keyboard_input.pressed(EXIT_KEY1) && keyboard_input.just_pressed(EXIT_KEY2) {
        // message to console that program is exiting
        println!("Exiting program");
        // exit the process
        //TODO: bring up a dialog box to confirm exit or not

        std::process::exit(0);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // pause the bevy game
        // spawn a pause menu
        println!("Paused");
    }
}
