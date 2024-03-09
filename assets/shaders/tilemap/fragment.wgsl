#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TilemapSize {
	size: vec2<f32>,
}

@group(2) @binding(0) var<uniform> tilemap_size: TilemapSize;
@group(2) @binding(1) var<storage> tile_storage: array<u32>;
@group(2) @binding(2) var tile_texture: texture_2d<f32>;
@group(2) @binding(3) var tile_texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tile_pos = mesh.uv * tilemap_size.size;

	let texture_dimensions = textureDimensions(tile_texture);

	let texture_offset = f32(tile_storage[i32(floor(tile_pos.y)) * i32(tilemap_size.size.x) + i32(floor(tile_pos.x))]) * f32(texture_dimensions.y) / f32(texture_dimensions.x);

	return textureSample(
		tile_texture,
		tile_texture_sampler,
		vec2<f32>(texture_offset, 0.0) + (tile_pos - floor(tile_pos)) * f32(texture_dimensions.y) / f32(texture_dimensions.x)
	);
}