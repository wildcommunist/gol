use bevy::app::{App};
use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, default, Entity, Mesh, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Transform, With};
use bevy::time::{Time, Timer, TimerMode};

const CELL_SIZE: f32 = 10.0;
const WORLD_SIZE_WIDTH: u16 = 200;
const WORLD_SIZE_HEIGHT: u16 = 15;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Position(u16, u16);

#[derive(Component)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Component)]
struct MarkedForDeath(u8);


#[derive(Resource)]
struct GreetTimer(Timer);

fn spawn_cells(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..WORLD_SIZE_WIDTH {
        for y in 0..WORLD_SIZE_HEIGHT {
            if let Ok(c) = spawn_cell(x, y, CellState::Dead) {
                commands.spawn(c);
            }
        }
    }

    if let Ok(c) = spawn_cell(1, 1, CellState::Alive) {
        commands.spawn(c);
    }
    if let Ok(c) = spawn_cell(1, 2, CellState::Alive) {
        commands.spawn(c);
    }
    if let Ok(c) = spawn_cell(2, 1, CellState::Alive) {
        commands.spawn(c);
    }
    if let Ok(c) = spawn_cell(2, 2, CellState::Alive) {
        commands.spawn(c);
    }
}

fn spawn_cell(x: u16, y: u16, state: CellState) -> Result<(SpriteBundle, Cell, Position), String> {
    match (x, y) {
        (0..=WORLD_SIZE_WIDTH, 0..=WORLD_SIZE_HEIGHT) => {}
        _ => {
            return Err(format!("Invalid world coordinates"));
        }
    }
    let pos_float = (x as f32 * CELL_SIZE, y as f32 * CELL_SIZE);

    match state {
        CellState::Dead => {
            return Err(format!("Dead cell"));
        }
        CellState::Alive => {}
    }

    Ok(
        (SpriteBundle {
            transform: Transform::from_xyz(pos_float.0, pos_float.1, 0.0),
            sprite: cell_sprite(),
            ..default()
        },
         Cell,
         Position(x, y))
    )
}

fn cell_sprite() -> Sprite {
    Sprite {
        color: Color::rgb(255.0, 0.0, 0.0),
        flip_x: false,
        flip_y: false,
        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
        rect: None,
        anchor: Default::default(),
    }
}

fn greet_cells(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Position, With<Cell>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for pos in query.iter() {
            println!("Cell spawned at x:{} y:{}", pos.0, pos.1);
        }
    }
}

fn perform_life(
    time: Res<Time>,
    mut pos: Query<(Entity, &mut Position, &mut Transform), With<Cell>>,
) {
    for (_, mut pos, mut transform) in &mut pos {
        //transform.translation.x += 1. * time.delta_seconds() * CELL_SIZE;
        //transform.translation.y += 1. * time.delta_seconds() * CELL_SIZE;
    }
}

pub struct GameOfLife;

impl Plugin for GameOfLife {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(spawn_cells)
            .add_system(perform_life);
    }
}