use bevy::{prelude::*, sprite::{Material2d, MaterialMesh2dBundle}};

#[derive(Component)]
enum TileType {
	Air,
	Ground,
}

#[derive(Component)]
struct TileIndex(i32);

#[derive(Component)]
struct TileStorage {
	tiles: Vec<Entity>,
}

#[derive(Component)]
struct TilemapSize(IVec2);

#[derive(Component)]
struct TileParent(Entity);

#[derive(Bundle)]
struct TileBundle {
	tiletype: TileType,
	index: TileIndex,
	tilemap: TileParent,
}

#[derive(Bundle)]
struct TilemapBundle<M: Material2d> {
	tiles: TileStorage,
	size: TilemapSize,
	mesh: MaterialMesh2dBundle<M>
	// TODO: Make a function to load a tilemap from a bmp or png file
}

pub struct TilemapPlugin;