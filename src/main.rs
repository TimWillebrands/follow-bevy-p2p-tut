use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.47, 0.47, 0.47)))
        .add_startup_systems((setup, spawn_player))
        .add_system(move_player)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn spawn_player(mut commands: Commands) {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.53, 1.0),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    };
    commands.spawn((Player, sprite_bundle));
}

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

#[derive(Component)]
struct Player;
