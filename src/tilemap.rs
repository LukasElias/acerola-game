use bevy::{
	prelude::*, render::render_resource::ShaderType, sprite::{Material2d, MaterialMesh2dBundle}
};

#[derive(Component)]
enum TileType {
	Air,
	Ground,
}

#[derive(Component)]
struct TileIndex(u32);

#[derive(Component)]
pub struct TileStorage {
	pub tiles: Vec<Entity>,
}

impl TileStorage {
	pub fn from_vec_u32(commands: &mut Commands, storage: &Vec<u32>) -> Vec<Entity> {
		let mut tiles = vec![];

		for (index, tiletype) in storage.iter().enumerate() {
			tiles.push(
				commands.spawn(
					TileBundle {
						tiletype: match tiletype {
							0 => TileType::Air,
							_ => TileType::Ground,
						},
						index: TileIndex(index as u32),
					}
				).id()
			);
		}

		tiles
	}
}

#[derive(Component, ShaderType, Clone, Copy)]
pub struct TilemapSize {
	pub size: Vec2,
}

#[derive(Bundle)]
struct TileBundle {
	tiletype: TileType,
	index: TileIndex,
}

#[derive(Bundle)]
pub struct TilemapBundle<M: Material2d> {
	pub tiles: TileStorage,
	pub size: TilemapSize,
	pub material_mesh: MaterialMesh2dBundle<M>,
	// TODO: Make a function to load a tilemap from a ron file
}

pub struct TilemapPlugin;