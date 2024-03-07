use bevy::prelude::*;

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
struct TilemapBundle {
	tiles: TileStorage,
	size: TilemapSize,
}

pub struct TilemapPlugin;