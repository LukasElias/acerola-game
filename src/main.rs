use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution, WindowTheme},
};
use character::CharacterPlugin;

mod character;
mod tilemap;

fn setup(mut commands: Commands) {
    // create the camera
    commands.spawn(
        Camera2dBundle::default()
    );

    character::spawn_character(commands);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    resolution: WindowResolution::new(500.0, 500.0),
                    visible: true,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
            CharacterPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
