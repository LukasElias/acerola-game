use {
    bevy::{
        prelude::*,
        sprite::Material2dPlugin,
        window::{
            PresentMode,
            WindowMode,
            WindowResolution,
            WindowTheme,
        },
    },
    bevy_common_assets::ron::RonAssetPlugin,
    character::*,
    tilemap::*,
};

mod character;
mod tilemap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Level,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // create the camera
    commands.spawn(
        Camera2dBundle::default()
    );

    spawn_character(&mut commands);

    let level_1 = LevelHandle(asset_server.load("levels/1.level.ron"));
    let tile_texture = ImageHandle(asset_server.load("textures/tile-textures.png"));

    commands.insert_resource(level_1);
    commands.insert_resource(tile_texture);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    resizable: false,
                    resolution: WindowResolution::new(800.0, 480.0),
                    visible: true,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<TilemapMaterial>::default(),
            RonAssetPlugin::<Level>::new(&["level.ron"]),
            CharacterPlugin,
            LevelPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
