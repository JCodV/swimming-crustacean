use bevy::{input::common_conditions::input_just_pressed, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RigidBodyPlugin)
        .run();
}

pub struct RigidBodyPlugin;
impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                handle_jump.run_if(input_just_pressed(KeyCode::Space)),
                apply_gravity,
            ),
        );
    }
}

#[derive(Resource)]
struct Gravity(f32);

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -900. / 2. + i as f32 / (num_shapes - 1) as f32 * 900.,
                0.0,
                0.0,
            ),
        ));
    }
}

fn add_player(mut commands: Commands) {
    let player_pos = Vec2::new(0.0, 0.0);
    let player_vel = Vec2::new(0.0, 0.0);
    let player_speed: f32 = 100.0;
    commands
        .spawn(RigidBodyBundle {
            position: Position(player_pos),
            velocity: Velocity(player_vel),
            speed: Speed(player_speed),
            on_ground: OnGround(false),
        })
        .insert(Crustacean);
}

// handles jump to all rigid body crustaceans (aka player)
fn handle_jump(mut query: Query<&mut RigidBodyBundle, With<Crustacean>>) {
    for mut rigid_body in &mut query {
        if rigid_body.on_ground.0 == true {
            rigid_body.velocity.0.y = -1.0 * rigid_body.speed.0;
            rigid_body.on_ground.0 = false;
        }
    }
}

// applies gravity to all rigid body crustaceans (aka player)
fn apply_gravity(gravity: Res<Gravity>, mut query: Query<&mut RigidBodyBundle, With<Crustacean>>) {
    for mut rigid_body in &mut query {
        if rigid_body.on_ground.0 == false {
            rigid_body.velocity.0.y += 1.0 * gravity.0;
            rigid_body.position.0.y += rigid_body.velocity.0.y;
        }
    }
}
