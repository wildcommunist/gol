use bevy::{prelude::*, time::FixedTimestep};

const CAMERA_MOVE_SPEED: f32 = 15.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct Movement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_move)
            .add_system(camera_zoom);
    }
}

fn setup(
    mut commands: Commands
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(Movement {
            plane_speed: Vec3::new(0.0, 0.0, 0.0),
            zoom_speed: 0.0,
        });
}

fn camera_move(
    mut query: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    let direction = direction.normalize_or_zero();

    let (mut transform, mut movement) = query
        .iter_mut()
        .next()
        .expect("No transform on main camera");

    movement.plane_speed = (movement.plane_speed + direction)
        .clamp(
            Vec3::new(-CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED),
            Vec3::new(CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED),
        );

    if keyboard_input.pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::ZERO;
    }

    transform.translation += movement.plane_speed;
}

fn camera_zoom(
    mut query: Query<(&mut Movement, &mut Camera2dBundle), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {}