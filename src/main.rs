use bevy::color::palettes::css::SKY_BLUE;
use bevy::prelude::*;
use bevy::render::camera::*;
use rand::prelude::*;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

const GRAVITY_MAX: f32 = 10.0;
const SPRITE_SIZE: f32 = 100.0;

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
        .add_systems(Startup, setup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ObstacleSpawnerPlugin)
        .run();
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, handle_player_jump);
        app.add_systems(Update, apply_gravity);
    }
}

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_environment);
    }
}

pub struct ObstacleSpawnerPlugin;
impl Plugin for ObstacleSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacle_timer);
        app.add_systems(Update, spawn_obstacles);
    }
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
            jump_velocity: JumpVelocity(150.0),
        }
    }
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    //setup camera
    let proj = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        //scale: 0.3,
        ..OrthographicProjection::default_2d()
    });

    clear_color.0 = SKY_BLUE.into();
    commands.spawn((Camera2d, proj));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load("default_rustacean.png");
    commands.spawn((
        Rustacean,
        Sprite {
            image: sprite,
            custom_size: Some(Vec2 {
                x: SPRITE_SIZE,
                y: SPRITE_SIZE,
            }),
            ..default()
        },
        PlayerPhysicsBundle::default(),
    ));
}

fn handle_player_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &JumpVelocity), With<Rustacean>>,
) {
    for (mut velocity, jump_velocity) in &mut query {
        if keys.just_pressed(KeyCode::Space) {
            velocity.0.y = jump_velocity.0;
        }
    }
}

fn apply_gravity(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Gravity), With<Rustacean>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut gravity) in &mut query {
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
    let num_sand_tiles = WINDOW_WIDTH as i32 / SPRITE_SIZE as i32;
    let sand_tile_asset = asset_server.load("sand_floor_tile.png");
    let sprite = Sprite {
        image: sand_tile_asset,
        custom_size: Some(Vec2 {
            x: SPRITE_SIZE,
            y: SPRITE_SIZE,
        }),
        ..default()
    };

    for i in 0..num_sand_tiles {
        commands.spawn((
            sprite.clone(),
            FloorTile,
            Transform {
                translation: Vec3 {
                    x: (-WINDOW_WIDTH / 2.0) + i as f32 * SPRITE_SIZE + SPRITE_SIZE / 2.0,
                    y: (-WINDOW_HEIGHT / 2.0) + SPRITE_SIZE / 2.0,
                    z: 0.0,
                },
                ..default()
            },
        ));
    }
}

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

fn spawn_obstacle_timer(mut commands: Commands) {
    let timer = Timer::from_seconds(2.0, TimerMode::Repeating);
    commands.insert_resource(ObstacleSpawnTimer(timer));
}

fn spawn_obstacles(
    mut commands: Commands,
    mut obs_timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    println!("obs timer time elapsed: {:?}", obs_timer.0);
    let mut rng = rand::thread_rng();
    let coral_height = 2;
    let sea_weed_height = 3;
    let mut height;
    let h_tiles = WINDOW_WIDTH as i32 / SPRITE_SIZE as i32;

    let coral_sprite = asset_server.load("coral.png");
    let sea_weed_sprite = asset_server.load("sea_weed.png");

    if obs_timer.0.finished() {
        // spawn top
        let mut num: i32 = rng.gen_range(1..3);
        let top_sprite = if num == 1 {
            height = coral_height;
            coral_sprite.clone()
        } else {
            height = sea_weed_height;
            sea_weed_sprite.clone()
        };
        commands.spawn(Sprite {
            image: top_sprite,
            flip_x: true,
            custom_size: Some(Vec2 {
                x: SPRITE_SIZE,
                y: height as f32,
            }),
            ..default()
        });
        // spawn bottom
        num = rng.gen_range(1..3);
        let bottom_sprite = if num == 1 {
            height = coral_height;
            coral_sprite.clone()
        } else {
            height = sea_weed_height;
            sea_weed_sprite.clone()
        };
        commands.spawn(Sprite {
            image: bottom_sprite,
            flip_x: false,
            custom_size: Some(Vec2 {
                x: SPRITE_SIZE,
                y: height as f32,
            }),
            ..default()
        });
    } else {
        obs_timer.0.tick(time.delta());
    }
}
