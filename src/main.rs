use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, movement)
        .run();
}

#[derive(Component)]
struct Box;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let rect = meshes.add(Rectangle::new(50., 50.));
    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(materials.add(Color::srgb_u8(255, 255, 0))),
        Box,
    ));
}

const SPEED: f32 = 100.;

fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Box>>,
    time: Res<Time>,
) {
    let mut direction = 0.;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.;
    }

    transform.translation.x += direction * SPEED * time.delta_secs();
}