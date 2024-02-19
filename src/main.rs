use bevy::prelude::*;

const KARL_SIZE: f32 = 80.;
const KARL_SPEED: f32 = 200.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_player_direction, update_positions).chain())
        .run();
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component, PartialEq)]
struct Movable {
    direction: Vec2,
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());
    // karl
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("testing/karl.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(KARL_SIZE)),
                ..default()
            },
            ..default()
        },
        Movable {
            speed: KARL_SPEED,
            ..default()
        },
        Player::default(),
    ));
    // UI
    commands.spawn(
        TextBundle::from_section(
            "testing",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

// ------------------------------------------------------------------------------------------------

fn update_player_direction(
    mut players: Query<&mut Movable, With<Player>>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    let mut dir = Vec2::new(0.0, 0.0);
    if keycode.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        dir += Vec2::new(0.0, 1.0)
    }
    if keycode.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        dir += Vec2::new(0.0, -1.0)
    }
    if keycode.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        dir += Vec2::new(-1.0, 0.0)
    }
    if keycode.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        dir += Vec2::new(1.0, 0.0)
    }
    for mut player in &mut players {
        player.direction = dir;
    }
}

fn update_positions(mut ent: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (dir, mut transform) in &mut ent {
        let norm_dir = dir.direction.normalize_or_zero();
        transform.translation.x += norm_dir.x * dt * dir.speed;
        transform.translation.y += norm_dir.y * dt * dir.speed;
    }
}
