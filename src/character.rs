use {
    crate::{
        AppState,
        Level,
        LevelHandle,
        TileType,
    }, bevy::{
        prelude::*,
        sprite::Anchor,
    }, std::ops::Deref
};

#[derive(Component)]
pub struct Velocity(bevy::prelude::Vec2);

#[derive(Component)]
pub struct Fall(f32);

#[derive(Bundle)]
struct Character {
    fall: Fall,
    velocity: Velocity,
    sprite: SpriteBundle,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
            Update,
            (
                (
                    (jump, gravity, move_right, move_left).run_if(in_state(AppState::Level)),
                    update_position
                ).chain(),
                fall_damage_screen.run_if(in_state(AppState::Dead)),
                winning_screen.run_if(in_state(AppState::Won)),
                colliding_key.run_if(in_state(AppState::Level)),
            )
        );
	}
}

fn colliding_key(
    mut state: ResMut<NextState<AppState>>,
    mut levels: ResMut<Assets<Level>>,
    level: Res<LevelHandle>,
    query: Query<(&Velocity, &Transform)>,
) {
    let character = query.single();
    if let Some(level) = levels.get_mut(level.0.id()) {
        let tile_pos = level.screen_pos_to_tile_pos(character.1.translation.xy()).floor();

        let mut is_colliding: bool = false;
        
        let tile_0 = level.tile_storage.tiles.get((tile_pos.y * level.size.size.x + tile_pos.x) as usize);
        let tile_1 = level.tile_storage.tiles.get(((tile_pos.y + 1.0) * level.size.size.x + tile_pos.x) as usize);
        let tile_2 = level.tile_storage.tiles.get(((tile_pos.y + 1.0) * level.size.size.x + tile_pos.x + 1.0) as usize);
        let tile_3 = level.tile_storage.tiles.get((tile_pos.y * level.size.size.x + tile_pos.x + 1.0) as usize);

        if tile_0.is_some() {
            is_colliding = *tile_0.unwrap() == TileType::Key;
        }

        if tile_1.is_some() {
            is_colliding = is_colliding || *tile_1.unwrap() == TileType::Key;
        }

        if tile_2.is_some() {
            is_colliding = is_colliding || *tile_2.unwrap() == TileType::Key;
        }

        if tile_3.is_some() {
            is_colliding = is_colliding || *tile_3.unwrap() == TileType::Key;
        }

        if is_colliding {
            state.set(AppState::Won);
        }
    }
}

fn winning_screen(mut query: Query<(&mut Visibility, &Text)>) {
    for mut text in query.iter_mut() {
        if text.1.sections[0].value == String::from("You Won!") {
            *text.0 = Visibility::Visible;
        }
    }
}

fn fall_damage_screen(mut query: Query<(&mut Visibility, &Text)>) {
    for mut text in query.iter_mut() {
        if text.1.sections[0].value == String::from("Fall damage is a thing!\nAnd you are dead!") {
            *text.0 = Visibility::Visible;
        }
    }
}

fn move_right(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    if let Some(level) = levels.get(level.0.id()) {
        if !level.is_colliding_right(character.1.deref()) {
            if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
                character.0.0.x = 10.0;
            }
        } else {
            character.0.0.x = 0.0;
            character.1.translation.x -= 1.0;
        }
    }
}

fn move_left(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    if let Some(level) = levels.get(level.0.id()) {
        if !level.is_colliding_left(character.1.deref()) {
            if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
                character.0.0.x = -10.0;
            }
        } else {
            character.0.0.x = 0.0;
            character.1.translation.x += 1.0;
        }
    }
}

fn jump(
    mut query: Query<(&mut Velocity, &Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    if let Some(level) = levels.get(level.0.id()) {
        if (
            keyboard_input.just_pressed(KeyCode::Space) ||
            keyboard_input.just_pressed(KeyCode::KeyW) ||
            keyboard_input.just_pressed(KeyCode::ArrowUp)
        ) && level.is_colliding_bottom(character.1) {
            character.0.0.y += 15.0;
        }
    }
}

fn gravity(
    mut query: Query<(&mut Velocity, &mut Transform, &mut Fall)>,
    mut state: ResMut<NextState<AppState>>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    if let Some(level) = levels.get(level.0.id()) {
        if level.is_colliding_bottom(character.1.deref()) || level.screen_pos_to_tile_pos(character.1.translation.xy()).y > level.size.size.y {
            character.0.0.y = 0.0;
            
            if character.2.0 < -150.0 {
                state.set(AppState::Dead);
            }

            character.2.0 = 0.0;
        } else {
            character.0.0.y -= 1.0;

            if character.0.0.y < 0.0 {
                character.2.0 += character.0.0.y;
            }
        }
    }
}

fn update_position(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    character.1.translation.x += character.0.0.x;
    character.1.translation.y += character.0.0.y;

    if let Some(level) = levels.get(level.0.id()) {
        if character.0.0.y > 0.0 {
            if level.is_colliding_top(character.1.deref()) {
                character.0.0.y = 0.0;
            }
        }
    }

    if keyboard_input.just_released(KeyCode::KeyA) ||
        keyboard_input.just_released(KeyCode::ArrowLeft) ||
        keyboard_input.just_released(KeyCode::KeyD) ||
        keyboard_input.just_released(KeyCode::ArrowRight)
    {
        character.0.0.x = 0.0;
    }
}

pub fn spawn_character(commands: &mut Commands) {
	commands.spawn(
        Character {
            fall: Fall(0.0),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(255.0, 255.0, 255.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
        },
    );
}