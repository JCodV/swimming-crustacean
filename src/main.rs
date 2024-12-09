use bevy::render::camera::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
const WINDOW_WIDTH: f32 = 512.0;
const WINDOW_HEIGHT: f32 = 600.0;

const GRAVITY_MAX: f32 = 10.0;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Swimming Rustacean".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_player_jump)
        .add_systems(Update, apply_gravity)
        .run();
}

#[derive(Component)]
struct Rustacean;

#[derive(Component)]
struct Gravity(f32);

impl Default for Gravity {
    fn default() -> Gravity {
        Gravity(-2.0)
    }
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct JumpVelocity(f32);

#[derive(Bundle)]
struct PlayerPhysicsBundle {
    velocity: Velocity,
    gravity: Gravity,
    jump_velocity: JumpVelocity,
}

impl Default for PlayerPhysicsBundle {
    fn default() -> PlayerPhysicsBundle {
        PlayerPhysicsBundle {
            velocity: Velocity(Vec2 { x: 0.0, y: 0.0 }),
            gravity: Gravity::default(),
            jump_velocity: JumpVelocity(80.0),
        }
    }
}

fn setup(mut commands: Commands) {
    let proj = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: 0.3,
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((Camera2d, proj));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Rustacean,
        Sprite::from_image(asset_server.load("default_rustacean.png")),
        PlayerPhysicsBundle::default(),
    ));
}

fn handle_player_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &JumpVelocity, &Rustacean)>,
) {
    for (mut velocity, jump_velocity, _rustacean) in &mut query {
        if keys.just_pressed(KeyCode::Space) {
            velocity.0.y = jump_velocity.0;
        }
    }
}

fn apply_gravity(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Gravity, &Rustacean)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut gravity, _rustacean) in &mut query {
        if gravity.0 > GRAVITY_MAX {
            gravity.0 = GRAVITY_MAX;
        }
        velocity.0.y += gravity.0;
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

#[derive(Component)]
struct FloorTile;

fn set_environment(mut commands: Commands, asset_server: Res<AssetServer>) {
    let num_sand_tiles = WINDOW_WIDTH as i32;
    for i in 0..num_sand_tiles {
        commands.spawn((
            Sprite::from_image(asset_server.load("sand_floor_tile.png")),
            FloorTile,
        ));
    }
}
