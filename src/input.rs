use bevy::{prelude::*, time::FixedTimestep};

const CAMERA_MOVE_SPEED: f32 = 25.0;
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
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.033))
                    .with_system(camera_move)
                    .with_system(camera_zoom)
            );
    }
}

fn setup(
    mut commands: Commands
) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 3.5,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1570.0, 1680.0, 0.0),
                ..default()
            },
            ..default()
        })
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
    let mut direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.7;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.7;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.7;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.7;
    }
    let direction = direction.normalize_or_zero();

    let (mut transform, mut movement) = query.single_mut();

    movement.plane_speed = (movement.plane_speed + direction)
        .clamp(
            Vec3::new(-CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED),
            Vec3::new(CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED),
        );

    if keyboard_input.pressed(KeyCode::R) {
        movement.plane_speed = Vec3::new(0.0, 0.0, 0.0);
    }

    transform.translation += movement.plane_speed;
}

fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut zoom_direction = 0.0;

    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = 0.01;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction -= 0.01;
    }

    let (mut movement, mut orto_proj) = camera.single_mut();
    movement.zoom_speed = (movement.zoom_speed + zoom_direction)
        .clamp(-CAMERA_ZOOM_SPEED, CAMERA_ZOOM_SPEED);

    orto_proj.scale = (orto_proj.scale + movement.zoom_speed).clamp(1.0, 6.0);

    if (orto_proj.scale - 1.0).abs() < 0.0001 || (orto_proj.scale - 6.0).abs() < 0.0001 || keyboard_input.pressed(KeyCode::R) {
        movement.zoom_speed = 0.0;
    }
}