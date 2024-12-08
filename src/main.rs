use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct Crustacean;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct OnGround(bool);

#[derive(Component)]
struct RigidBodyBundle {
    position: Position,
    velocity: Velocity,
    speed: Speed,
    on_ground: OnGround,
}

fn add_player(mut commands: Commands) {
    let player_pos = Vec2::new(0.0, 0.0);
    let player_vel = Vec2::new(0.0, 0.0);
    let player_speed: f32 = 100.0;

    commands.spawn(RigidBodyBundle {
        position: Position(player_pos),
        velocity: Velocity(player_vel),
        speed: Speed(player_speed),
        on_ground: OnGround(false),
    });
}

// handles jump for entities that have a position, speed and are a crustacean
fn handle_jump(mut query: Query<(&mut RigidBodyBundle), With<Crustacean>>) {
    for rigidBody in &mut query {}
}
