use bevy::color::palettes::css::SKY_BLUE;
use bevy::prelude::*;
use bevy::render::camera::*;
use rand::prelude::*;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

const GRAVITY_MAX: f32 = 10.0;
const SPRITE_SIZE: f32 = 100.0;
const SAND_TOP_OF_FLOOR_Y: f32 = (-WINDOW_HEIGHT / 2.0) + SPRITE_SIZE;
const SAND_BOT_OF_FLOOR_Y: f32 = -WINDOW_HEIGHT / 2.0;

const GENERAL_SPAWN_POINT_X: f32 = WINDOW_WIDTH + 500.0;
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
        .add_plugins(ObstaclePlugin)
        .add_systems(Update, handle_player_obstacle_collision)
        .run();
}

#[derive(Resource)]
struct PlayerScore(i32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, handle_player_jump);
        app.add_systems(Update, apply_gravity);
        app.add_systems(Update, update_player_score);
    }
}

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_environment);
    }
}

pub struct ObstaclePlugin;
impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_timers);
        app.add_systems(Update, (spawn_obstacles, move_obstacles));
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
    let proj = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        //scale: 0.3,
        ..OrthographicProjection::default_2d()
    });

    clear_color.0 = SKY_BLUE.into();
    commands.spawn((Camera2d, proj));
    commands.insert_resource(PlayerScore(0));
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
                    y: (SAND_TOP_OF_FLOOR_Y + SAND_BOT_OF_FLOOR_Y) / 2.0,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

fn setup_timers(mut commands: Commands) {
    let obs_wait_time = 2.0;
    let shark_wait_time = 2.0;
    commands.insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
        obs_wait_time,
        TimerMode::Repeating,
    )));
    commands.insert_resource(SharkSpawnTimer(Timer::from_seconds(
        shark_wait_time,
        TimerMode::Repeating,
    )));
}
#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct ScorePillar {
    x: f32,
    passed: bool,
}

fn spawn_obstacles(
    mut commands: Commands,
    mut obs_timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    let coral_height = 2;
    let sea_weed_height = 3;
    let height;

    let coral_sprite = asset_server.load("pink_coral.png");
    let sea_weed_sprite = asset_server.load("seaweed.png");

    obs_timer.0.tick(time.delta());
    if obs_timer.0.finished() {
        let num: i32 = rng.gen_range(1..3);
        let sprite = if num == 1 {
            height = coral_height;
            coral_sprite.clone()
        } else {
            height = sea_weed_height;
            sea_weed_sprite.clone()
        };
        commands.spawn((
            Obstacle,
            Sprite {
                image: sprite,
                custom_size: Some(Vec2 {
                    x: SPRITE_SIZE,
                    y: SPRITE_SIZE * height as f32,
                }),
                ..default()
            },
            Transform {
                translation: Vec3 {
                    x: GENERAL_SPAWN_POINT_X,
                    y: SAND_TOP_OF_FLOOR_Y + (SPRITE_SIZE * height as f32) / 2.0,
                    ..default()
                },
                ..default()
            },
            ScorePillar {
                x: WINDOW_WIDTH + 500.0,
                passed: false,
            },
        ));
    }
}

fn move_obstacles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut ScorePillar), With<Obstacle>>,
) {
    for (mut transform, mut pillar) in &mut query {
        transform.translation.x -= 200.0 * time.delta_secs();
        pillar.x -= 200.0 * time.delta_secs();
    }
}

fn handle_player_obstacle_collision(
    player_query: Query<&Transform, With<Rustacean>>,
    obs_query: Query<(&Transform, &Sprite), With<Obstacle>>,
) {
    let p = &player_query.single();
    let p_xmin = p.translation.x - SPRITE_SIZE / 2.0;
    let p_xmax = p.translation.x + SPRITE_SIZE / 2.0;
    let p_ymin = p.translation.y - SPRITE_SIZE / 2.0;
    let p_ymax = p.translation.y + SPRITE_SIZE / 2.0;
    for (o, sprite) in &obs_query {
        let sizes = sprite.custom_size.unwrap();
        let o_xmin = o.translation.x - sizes[0] + 20.0;
        let o_xmax = o.translation.x + sizes[0] - 20.0;
        let o_ymin = o.translation.y - sizes[1];
        let o_ymax = o.translation.y + sizes[1] - 200.0;
        if p_xmin <= o_xmax && p_xmax >= o_xmin && p_ymin <= o_ymax && p_ymax >= o_ymin {
            //println!("Collision detected!");
        }
    }
}

fn update_player_score(
    mut score: ResMut<PlayerScore>,
    player_query: Query<&Transform, With<Rustacean>>,
    mut pillar: Query<&mut ScorePillar>,
) {
    let p = &player_query.single();

    for mut pil in &mut pillar {
        if p.translation.x >= pil.x && pil.passed == false {
            score.0 += 1;
            pil.passed = true;
        }
    }
    println!("{}", score.0);
}

#[derive(Component)]
struct Shark;

#[derive(Resource)]
struct SharkSpawnTimer(Timer);

fn spawn_sharks(
    mut commands: Commands,
    mut timer: ResMut<SharkSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let rng = rand::thread_rng();
    let upper_shark_y = WINDOW_HEIGHT / 2.0;
    let lower_shark_y = -(WINDOW_HEIGHT / 2.0) + SAND_BOT_OF_FLOOR_Y;
    let random_shark_y_pos: f32 = 0.0;
    let shark_sprite = asset_server.load("shark.png");
    // ******************************
    // add the random pick of y level
    timer.0.tick(time.delta());
    if timer.0.finished() {
        commands.spawn((
            Shark,
            Sprite {
                image: shark_sprite,
                custom_size: Some(Vec2 {
                    x: SPRITE_SIZE * 4.0,
                    y: SPRITE_SIZE * 2.0,
                }),
                ..default()
            },
            Transform {
                translation: Vec3 {
                    x: GENERAL_SPAWN_POINT_X,
                    y: random_shark_y_pos,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn move_sharks() {
    todo!();
}
