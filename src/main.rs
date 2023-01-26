use bevy::prelude::*;

#[derive(Component)]
struct Acceleration {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..default()
        },
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 0.0, y: 0.0 },
        Acceleration { x: 0.0, y: -600.0 },
        Player,
    ));
}

fn apply_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x * time.delta().as_secs_f64() as f32;
        position.y += velocity.y * time.delta().as_secs_f64() as f32;
    }
}

fn apply_acceleration(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.x += acceleration.x * time.delta().as_secs_f64() as f32;
        velocity.y += acceleration.y * time.delta().as_secs_f64() as f32;
    }
}

fn bounce(windows: Res<Windows>, mut query: Query<(&mut Position, &mut Velocity), With<Player>>) {
    let window = windows.get_primary().unwrap();
    let (mut position, mut velocity) = query.get_single_mut().unwrap();
    if position.y <= -((window.physical_height() / 2) as f32) {
        position.y = -((window.physical_height() / 2) as f32);
        velocity.y *= -1.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(apply_position)
        .add_system(apply_velocity)
        .add_system(apply_acceleration)
        .add_system(bounce)
        .run();
}
