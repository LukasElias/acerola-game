use bevy::{
    prelude::*,
    sprite::Anchor,
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
            Update, ((jump, gravity), update_position).chain()
        );
	}
}

fn jump(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Velocity, &Transform)>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut character = query.single_mut();
        character.0.0.y += 30.0;
    }
}

fn gravity(mut query: Query<(&mut Velocity, &Transform)>) {
    let mut character = query.single_mut();
    if character.1.translation.y > 0.0 {
        character.0.0.y = character.0.0.y - 3.0;
    } else {
        character.0.0.y = 0.0;
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    let mut character = query.single_mut();
    character.1.translation.x += character.0.0.x;
    character.1.translation.y += character.0.0.y;
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