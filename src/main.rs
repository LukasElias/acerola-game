use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        ShaderRef,
    },
    sprite::{
        MaterialMesh2dBundle,
        Mesh2dHandle,
        Material2d,
        Material2dPlugin,
    },
    window::{
        PresentMode,
        WindowMode,
        WindowTheme,
    },
};
use character::CharacterPlugin;

mod character;
mod tilemap;

#[derive(AsBindGroup, Asset, TypePath, Clone)]
struct TilemapMaterial {
    #[uniform(0)]
    tilemap_size: tilemap::TilemapSize,
    #[storage(1, read_only)]
    tile_storage: Vec<u32>,
    #[texture(2)]
    #[sampler(3)]
    tile_texture: Option<Handle<Image>>,
}

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/fragment.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // create the camera
    commands.spawn(
        Camera2dBundle::default()
    );

    
    let mut tile_storage: Vec<u32> = vec![];
    
    for y in 0..8 {
        for x in 0..8 {
            tile_storage.push(((1 - y as i32 % 2) * 1 - x as i32 % 2).abs() as u32);
        }
    }
    
    // character::spawn_character(&mut commands);

    let tilemap_material = TilemapMaterial {
        tilemap_size: tilemap::TilemapSize { size: Vec2::new(8.0, 8.0) },
        tile_texture: Some(asset_server.load("textures/tile-textures.png")),
        tile_storage,
    };

    let tiles = tilemap::TileStorage {
        tiles: tilemap::TileStorage::from_vec_u32(&mut commands, &tilemap_material.tile_storage),
    };
    
    commands.spawn(
        tilemap::TilemapBundle {
            tiles,
            size: tilemap_material.tilemap_size,
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(tilemap_material.tilemap_size.size.x * 8.0, tilemap_material.tilemap_size.size.y * 8.0))),
                material: materials.add(tilemap_material),
                ..default()
            },
        }
    );
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<TilemapMaterial>::default(),
            CharacterPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
