use {
	serde::Deserialize,
	crate::{
		AppState,
		Velocity,
	},
	bevy::{
		prelude::*,
		render::render_resource::{
			AsBindGroup, ShaderRef, ShaderType
		},
		sprite::{
			Material2d,
			MaterialMesh2dBundle,
            Mesh2dHandle,
		},
	},
};

#[derive(Component, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TileType {
	Air,
	Grass,
	GrassL,
	GrassR,
	Wall,
	Key,
}

impl TileType {
	fn to_u32(&self) -> u32 {
		match self {
			Self::Air => 0,
			Self::Grass => 1,
			Self::GrassL => 2,
			Self::GrassR => 3,
			Self::Wall => 4,
			Self::Key => 5,
		}
	}

	pub fn is_wall(&self) -> bool {
		match self {
			Self::Air => false,
			Self::Key => false,
			_ => true,
		}
	}
}

#[derive(Component, Deserialize, Clone)]
pub struct TileStorage {
	pub tiles: Vec<TileType>,
}

impl TileStorage {
	fn to_vec_u32(&self) -> Vec<u32> {
		Vec::from_iter(self.tiles.iter().map(|tile_type: &TileType| {
			tile_type.to_u32()
		}))
	}
}

#[derive(Component, ShaderType, Clone, Copy, Deserialize)]
pub struct TilemapSize {
	pub size: Vec2,
}

#[derive(Bundle)]
pub struct TilemapBundle<M: Material2d> {
	pub tile_storage: TileStorage,
	pub size: TilemapSize,
	pub material_mesh: MaterialMesh2dBundle<M>,
}

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct TilemapMaterial {
    #[uniform(0)]
    pub tilemap_size: TilemapSize,
    #[storage(1, read_only)]
    pub tile_storage: Vec<u32>,
    #[texture(2)]
    #[sampler(3)]
    pub tile_texture: Option<Handle<Image>>,
}

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/fragment.wgsl".into()
    }
}

#[derive(Asset, TypePath, Deserialize, Clone)]
pub struct Level {
	pub size: TilemapSize,
	pub tile_storage: TileStorage,
	pub tilemap_entity: Option<Entity>,
	start_tile: UVec2,
}

impl Level {
	fn get_start_tile_screen_pos(&self) -> Vec2 {
		let size = self.size.size;
		let mid_tile = size / Vec2::splat(2.0);

		(self.start_tile.as_vec2() - mid_tile) * Vec2::new(16.0, -16.0)
	}

	pub fn screen_pos_to_tile_pos(&self, screen_pos: Vec2) -> Vec2 {
		let size = self.size.size;
		let mid_tile = size / Vec2::splat(2.0);

		screen_pos / Vec2::new(16.0, -16.0) + mid_tile
	}

	pub fn is_colliding_bottom(&self, query: &Transform) -> bool {
        let tile_pos = self.screen_pos_to_tile_pos(query.translation.xy()).as_uvec2();

		let tile_index = (tile_pos.y + 2) as usize * self.size.size.x as usize + tile_pos.x as usize;

		let tiletype_0 = self.tile_storage.tiles.get(tile_index);
		let tiletype_1 = self.tile_storage.tiles.get(tile_index + 1);
		let mut result: bool = false;

		if tiletype_0.is_some() {
			result = tiletype_0.unwrap().is_wall();
		}

		if tiletype_1.is_some() {
			result = result || tiletype_1.unwrap().is_wall();
		}

		result
	}

	pub fn is_colliding_top(&self, query: &Transform) -> bool {
		let tile_pos = self.screen_pos_to_tile_pos(query.translation.xy()).as_uvec2();

		let tile_index = (tile_pos.y - 1) as usize * self.size.size.x as usize + tile_pos.x as usize;

		let tiletype_0 = self.tile_storage.tiles.get(tile_index);
		let tiletype_1 = self.tile_storage.tiles.get(tile_index + 1);
		let mut result: bool = false;

		if tiletype_0.is_some() {
			result = tiletype_0.unwrap().is_wall();
		}

		if tiletype_1.is_some() {
			result = result || tiletype_1.unwrap().is_wall();
		}

		result
	}

	pub fn is_colliding_right(&self, query: &Transform) -> bool {
		let tile_pos = self.screen_pos_to_tile_pos(query.translation.xy()).as_uvec2();
		
		let tile_index = tile_pos.y as usize * self.size.size.x as usize + tile_pos.x as usize + 2;

		let tiletype_0 = self.tile_storage.tiles.get(tile_index);
		let tiletype_1 = self.tile_storage.tiles.get(tile_index + self.size.size.x as usize);
		let mut result: bool = false;

		if tiletype_0.is_some() {
			result = tiletype_0.unwrap().is_wall();
		}

		if tiletype_1.is_some() {
			result = result || tiletype_1.unwrap().is_wall();
		}

		result
	}
	
	pub fn is_colliding_left(&self, query: &Transform) -> bool {
		let tile_pos = self.screen_pos_to_tile_pos(query.translation.xy()).as_uvec2();

		let tile_index = tile_pos.y as usize * self.size.size.x as usize + tile_pos.x as usize;

		let tiletype_0 = self.tile_storage.tiles.get(tile_index);
		let tiletype_1 = self.tile_storage.tiles.get(tile_index + self.size.size.x as usize);
		let mut result: bool = false;

		if tiletype_0.is_some() {
			result = tiletype_0.unwrap().is_wall();
		}
		
		if tiletype_1.is_some() {
			result = result || tiletype_1.unwrap().is_wall();
		}

		result
	}
}

#[derive(Resource)]
pub struct LevelHandle(pub Handle<Level>);

#[derive(Resource)]
pub struct ImageHandle(pub Handle<Image>);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update, spawn_level.run_if(in_state(AppState::Loading))
		);
	}
}

pub fn spawn_level(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
	mut state: ResMut<NextState<AppState>>,
	mut query: Query<&mut Transform, With<Velocity>>,
    mut levels: ResMut<Assets<Level>>,
	tile_texture: Res<ImageHandle>,
	level: Res<LevelHandle>,
) {
	if let Some(level) = levels.get_mut(level.0.id()) {
		let tilemap_material: TilemapMaterial = TilemapMaterial {
			tilemap_size: level.size,
			tile_storage: level.tile_storage.to_vec_u32(),
			tile_texture: Some(tile_texture.0.to_owned()),
		};

		let tilemap_entity = commands.spawn(
			TilemapBundle {
				tile_storage: level.tile_storage.to_owned(),
				size: level.size,
				material_mesh: MaterialMesh2dBundle {
					mesh: Mesh2dHandle(meshes.add(Rectangle::new(tilemap_material.tilemap_size.size.x * 16.0, tilemap_material.tilemap_size.size.y * 16.0))),
					material: materials.add(tilemap_material),
					..default()
				},
			}
		).id();

		level.tilemap_entity = Some(tilemap_entity);

		let mut character = query.single_mut();
		let tile_screen_pos = level.get_start_tile_screen_pos();

		character.translation = Vec3::new(tile_screen_pos.x, tile_screen_pos.y, 1.0);

		state.set(AppState::Level);
	}
}