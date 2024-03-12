use {
    std::ops::Deref,
    crate::{
        Level, LevelHandle
    },
    bevy::{
        prelude::*,
        sprite::Anchor,
    },
};

#[derive(Component)]
pub struct Velocity(bevy::prelude::Vec2);

#[derive(Bundle)]
struct Character {
    velocity: Velocity,
    sprite: SpriteBundle,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
            Update, ((jump, gravity, move_right, move_left), update_position).chain()
        );
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
    mut query: Query<(&mut Velocity, &mut Transform)>,
    levels: Res<Assets<Level>>,
    level: Res<LevelHandle>,
) {
    let mut character = query.single_mut();
    if let Some(level) = levels.get(level.0.id()) {
        if level.is_colliding_bottom(character.1.deref()) {
            character.0.0.y = 0.0;
        } else {
            character.0.0.y -= 1.0;
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