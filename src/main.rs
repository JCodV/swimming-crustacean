use bevy::{input::common_conditions::input_just_pressed, prelude::*};

const WINDOW_WIDTH: f32 = 512.0;
const WINDOW_HEIGHT: f32 = 600.0;

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
        Gravity(2.0)
    }
}
#[derive(Component)]
struct IsJumping(bool);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct JumpVelocity(f32);

#[derive(Component)]
struct OnGround(bool);

#[derive(Bundle)]
struct PlayerPhysics {
    velocity: Velocity,
    gravity: Gravity,
    is_jumping: IsJumping,
    on_ground: OnGround,
    jump_velocity: JumpVelocity,
}

impl Default for PlayerPhysics {
    fn default() -> PlayerPhysics {
        PlayerPhysics {
            velocity: Velocity(Vec2 { x: 0.0, y: 0.0 }),
            gravity: Gravity::default(),
            is_jumping: IsJumping(false),
            on_ground: OnGround(false),
            jump_velocity: JumpVelocity(10.0),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Rustacean,
        Sprite::from_image(asset_server.load("default_rustacean.png")),
        PlayerPhysics::default(),
    ));
}

fn handle_player_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut OnGround,
        &mut IsJumping,
        &JumpVelocity,
        &Rustacean,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut on_ground, mut is_jumping, jump_velocity, _rustacean) in
        &mut query
    {
        if keys.pressed(KeyCode::Space) && on_ground.0 == true {
            velocity.0.y = jump_velocity.0;
            on_ground.0 = false;
            is_jumping.0 = true;
            transform.translation.y += velocity.0.y * time.delta_secs();
        }
    }
}

fn apply_gravity(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut OnGround,
        &Gravity,
        &Rustacean,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, on_ground, gravity, _rustacean) in &mut query {
        if on_ground.0 == false {
            velocity.0.y += gravity.0;
            transform.translation.y -= velocity.0.y * time.delta_secs();
        }
    }
}

#[derive(Component)]
struct FloorTile;

fn set_environment(mut commands: Commands) {
    let num_sand_tiles: u32 = WINDOW_WIDTH as u32 / 32;
}
