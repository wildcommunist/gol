use bevy::{prelude::*, time::FixedTimestep, app::AppExit};
use crate::input::MainCamera;
use crate::ui::{GameExitEvent, StartSimulationEvent, StopSimulationEvent};

const CELL_SIZE: f32 = 32.0;
const GRID_SIZE: i32 = 100;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.39, 0.58, 0.93)))
            .insert_resource(MousePositionDraw(None))
            .insert_resource(MousePositionErase(None))
            .insert_resource(IsSimulationRunning(false))
            .add_system(exit_game)
            .add_system(stop_simulation)
            .add_system(start_simulation)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.016))
                    .with_system(
                        set_cursor_world_position
                            .label(CellInteraction::Input)
                    )
                    .with_system(
                        cell_interaction
                            .label(CellInteraction::Setting)
                            .after(CellInteraction::Input)
                    )
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.06))
                    .with_system(
                        simulation_step
                            .label(CellInteraction::Simulation)
                            .after(CellInteraction::Setting)
                    )
            )
            .add_startup_system(setup);
        //.add_startup_system(setup_samples);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let (ass, state) = match check_samples(x, y) {
                true => {
                    ("sprites/alive_cell.png", CellState::Alive)
                }
                false => {
                    ("sprites/empty_cell.png", CellState::Empty)
                }
            };
            commands
                .spawn(
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new((x as f32) * CELL_SIZE, (y as f32) * CELL_SIZE, 0.0),
                            scale: Vec3::ONE,
                            ..Default::default()
                        },
                        sprite: Sprite {
                            ..Default::default()
                        },
                        texture: asset_server.load(ass),
                        ..default()
                    }
                )
                .insert(Cell {
                    state,
                });
        }
    }

    commands
        .insert_resource(SpriteImages {
            empty_cell: asset_server.load("sprites/empty_cell.png"),
            alive_cell: asset_server.load("sprites/alive_cell.png"),
            dead_cell: asset_server.load("sprites/dead_cell.png"),
        });
}

pub fn check_samples(
    x: i32, y: i32,
) -> bool {
    let shape = vec![
        (50, 50),
        (50, 51),
        (49, 51),
        (50, 52),
        (51, 52),
    ];

    for (sx, sy) in shape {
        if sx == x && sy == y {
            return true;
        }
    }
    false
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CellInteraction {
    Input,
    Setting,
    Simulation,
}

fn set_cursor_world_position(
    windows: Res<Windows>,
    main_camera: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_world_pos_draw: ResMut<MousePositionDraw>,
    mut mouse_world_pos_erase: ResMut<MousePositionErase>,
    is_running: Res<IsSimulationRunning>,
) {
    let window = windows.get_primary().unwrap();
    if !is_running.0 {
        // only accept mouse clicks when not running
        if let Some(pos) = window.cursor_position() {
            let (transform, proj) = main_camera.single();
            let world_pos = get_mouse_world_coord(pos, transform, window, proj);

            if mouse_button.pressed(MouseButton::Left) {
                *mouse_world_pos_draw = MousePositionDraw(Some((world_pos.x, world_pos.y)));
            }

            if mouse_button.pressed(MouseButton::Right) {
                *mouse_world_pos_erase = MousePositionErase(Some((world_pos.x, world_pos.y)));
            }
        }
    }
}

fn get_mouse_world_coord(
    pos: Vec2,
    main_transform: &Transform,
    window: &Window,
    proj: &OrthographicProjection,
) -> Vec3 {
    let center = main_transform.translation.truncate();
    let half_width = (window.width() / 2.0) * proj.scale;
    let half_height = (window.height() / 2.0) * proj.scale;
    let left = center.x - half_width;
    let bottom = center.y - half_height;

    Vec3::new(
        left + pos.x * proj.scale,
        bottom + pos.y * proj.scale,
        0.0,
    )
}

fn cell_interaction(
    mut cells: Query<(&mut Cell, &mut Handle<Image>, &Transform)>,
    mut mouse_world_pos_draw: ResMut<MousePositionDraw>,
    mut mouse_world_pos_erase: ResMut<MousePositionErase>,
    sprite_images: Res<SpriteImages>,
    is_running: Res<IsSimulationRunning>,
) {
    let mouse_draw = mouse_world_pos_draw.0.take();
    let mouse_erase = mouse_world_pos_erase.0.take();
    if !is_running.0 {
        for (mut cell, mut sprite, transform) in cells.iter_mut() {
            if let Some(mouse_world_pos) = mouse_draw {
                if is_in_cell_bounds((mouse_world_pos.0, mouse_world_pos.1), (transform.translation.x, transform.translation.y), (16.0, 16.0)) {
                    cell.state = CellState::Alive;
                    *sprite = sprite_images.alive_cell.clone();
                }
            }

            if let Some(mouse_world_pos) = mouse_erase {
                if is_in_cell_bounds((mouse_world_pos.0, mouse_world_pos.1), (transform.translation.x, transform.translation.y), (16.0, 16.0)) {
                    cell.state = CellState::Empty;
                    *sprite = sprite_images.empty_cell.clone();
                }
            }
        }
    }
}

fn is_in_cell_bounds(xy: (f32, f32), center: (f32, f32), dims: (f32, f32)) -> bool {
    xy.0 >= center.0 - dims.0 && xy.0 < center.0 + dims.0 && xy.1 >= center.1 - dims.1 && xy.1 < center.1 + dims.1
}

fn exit_game(
    mut exit: EventWriter<AppExit>,
    mut event_reader: EventReader<GameExitEvent>,
) {
    if event_reader.iter().next().is_some() {
        exit.send(AppExit)
    }
}

fn start_simulation(
    mut event_reader: EventReader<StartSimulationEvent>,
    mut start: ResMut<IsSimulationRunning>,
) {
    if event_reader.iter().next().is_some() {
        start.0 = true;
    }
}

fn stop_simulation(
    mut event_reader: EventReader<StopSimulationEvent>,
    mut start: ResMut<IsSimulationRunning>,
) {
    if event_reader.iter().next().is_some() {
        start.0 = false;
    }
}

#[derive(Resource)]
struct MousePositionDraw(Option<(f32, f32)>);

#[derive(Resource)]
struct MousePositionErase(Option<(f32, f32)>);

#[derive(Component)]
struct Cell {
    state: CellState,
}

enum CellState {
    Alive,
    Dead,
    Empty,
}

#[derive(Component)]
struct MarkForDeath {
    generation: u8,
}

#[derive(Resource)]
struct IsSimulationRunning(bool);

#[derive(Resource)]
struct SpriteImages {
    empty_cell: Handle<Image>,
    alive_cell: Handle<Image>,
    dead_cell: Handle<Image>,
}

fn simulation_step(
    mut commands: Commands,
    mut cells: Query<(Entity, &mut Cell, &mut Handle<Image>, Option<&MarkForDeath>), With<Cell>>,
    is_running: Res<IsSimulationRunning>,
    sprite_images: Res<SpriteImages>,
) {
    if is_running.0 {
        let mut life_grid: Vec<bool> = Vec::new();
        for (_, cell, _sprite, mfd) in cells.iter_mut() {
            life_grid.push(match cell.state {
                CellState::Alive => {
                    if mfd.is_none() {
                        true
                    } else {
                        false
                    }
                }
                CellState::Dead | CellState::Empty => false,
            });
        }

        for (ind, (entity, mut cell, mut sprite, mfd)) in cells.iter_mut().enumerate() {
            let mut neighbour_cnt = 0;
            let x = ind as i32 % GRID_SIZE;
            let y = ind as i32 / GRID_SIZE;

            for xi in (x - 1)..(x + 2) {
                for yi in (y - 1)..(y + 2) {
                    if (xi != x || yi != y) && xi >= 0 && xi < GRID_SIZE && yi >= 0 && yi < GRID_SIZE {
                        let lin_ind = xi + yi * GRID_SIZE;
                        if life_grid[lin_ind as usize] {
                            neighbour_cnt += 1;
                        }
                    }
                }
            }

            if neighbour_cnt < 2 || neighbour_cnt > 3 {
                match cell.state {
                    CellState::Alive => {
                        if mfd.is_none() {
                            //commands.entity(entity).insert(MarkForDeath { generation: 3 });
                            cell.state = CellState::Dead;
                            *sprite = sprite_images.dead_cell.clone();
                        }
                    }
                    CellState::Dead | CellState::Empty => {}
                }
            }

            if neighbour_cnt == 3 {
                if mfd.is_none() {
                    cell.state = CellState::Alive;
                    *sprite = sprite_images.alive_cell.clone();
                }
            }
        }
    }
}