use bevy::prelude::*;

#[derive(Component)]
struct Velocity(bevy::prelude::Vec2);

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
        for mut character in query.iter_mut() {
            character.0.0.y += 100.0;
        }
    }
}

fn gravity(mut query: Query<(&mut Velocity, &Transform)>) {
    for mut character in query.iter_mut() {
        if character.1.translation.y > 0.0 {
            character.0.0.y = character.0.0.y - 20.0;
        }
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for mut character in query.iter_mut() {
        character.1.translation.x += character.0.0.x;
        character.1.translation.y += character.0.0.y;
        println!("Velocity: {}, {}", character.0.0.x, character.0.0.y);
    }
}

pub fn spawn_character(mut commands: Commands) {
	commands.spawn(
        Character {
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(255.0, 255.0, 255.0),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                ..default()
            },
        },
    );
}