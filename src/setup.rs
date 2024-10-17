use crate::{
    config::{alive_color, DEAD_COLOR, DECAY_TICKS, MAX_DEPTH, MAX_HEIGHT, MAX_WIDTH},
    types::{Board, Cell, Position},
};

use bevy::prelude::*;

use std::fs;

fn should_start_alive(x: usize, y: usize, z: usize, alive_board: &Vec<Vec<Vec<bool>>>) -> bool {
    return alive_board[x][y][z];
}

fn read_alive_board() -> Vec<Vec<Vec<bool>>> {
    let alive_file_string = fs::read_to_string("alive.csv").expect("Unable to read file");

    let mut alive_board: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; MAX_DEPTH]; MAX_HEIGHT]; MAX_WIDTH];

    let mut line_number = 0;
    for line in alive_file_string.lines() {
        line_number += 1;
        let xy = line.split(';').collect::<Vec<&str>>();
        if xy.len() < 2 || xy.len() > 3 {
            println!(
                "Warning: Invalid alive cell position: {}, on line {} of alive.csv",
                line, line_number
            );
            continue;
        }
        let x = xy[0].parse::<usize>();
        let x = if x.is_err() {
            println!(
                "Warning: Invalid alive cell position: {}, on line {} of alive.csv",
                line, line_number
            );
            continue;
        } else {
            x.unwrap()
        };
        let y = xy[1].parse::<usize>();
        let y = if y.is_err() {
            println!(
                "Warning: Invalid alive cell position: {}, on line {} of alive.csv",
                line, line_number
            );
            continue;
        } else {
            y.unwrap()
        };
        let z = if xy.len() == 3 {
            xy[2].parse::<usize>().unwrap()
        } else {
            0
        };
        if x >= MAX_WIDTH || y >= MAX_HEIGHT || z >= MAX_DEPTH {
            panic!("Invalid alive cell position: ({}, {}, {})", x, y, z);
        }
        alive_board[x][y][z] = true;
    }
    return alive_board;
}

pub fn create_board() -> Board {
    let alive_board = read_alive_board();
    let mut board: Board = Board(vec![]);
    for x in 0..MAX_WIDTH {
        let mut column: Vec<Vec<Cell>> = vec![];
        for y in 0..MAX_HEIGHT {
            let mut row: Vec<Cell> = vec![];
            for z in 0..MAX_DEPTH {
                let mut cell = Cell {
                    decaying: true,
                    dead: true,
                    decaying_ticks: 0,
                    neighbors: 0,
                };
                if should_start_alive(x, y, z, &alive_board) {
                    cell.decaying = false;
                    cell.dead = false;
                    cell.decaying_ticks = DECAY_TICKS;
                }
                row.push(cell);
            }
            column.push(row);
        }
        board.0.push(column);
    }

    return board;
}

pub fn setup(
    mut commands: Commands,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SCALE: f32 = 10.0;
    const X_SIZE: f32 = SCALE / MAX_WIDTH as f32;
    const Y_SIZE: f32 = SCALE / MAX_HEIGHT as f32;
    const Z_SIZE: f32 = SCALE / MAX_DEPTH as f32;
    println!("X_SIZE: {}, Y_SIZE: {}", X_SIZE, Y_SIZE);

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(-Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // init board sprites
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            for z in 0..MAX_DEPTH {
                let color = if board.0[x][y][z].dead {
                    DEAD_COLOR
                } else {
                    alive_color(z)
                };
                let x_3d = x as f32 * X_SIZE - SCALE / 2.0;
                let y_3d = y as f32 * Y_SIZE - SCALE / 2.0;
                let z_3d = z as f32 * Z_SIZE - SCALE / 2.0;
                let cube_size = Vec3::new(X_SIZE, Y_SIZE, Z_SIZE);
                // plane
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::default().mesh().scaled_by(cube_size)),
                        transform: Transform::from_translation(Vec3::new(x_3d, y_3d, z_3d)),
                        material: materials.add(color),
                        ..default()
                    },
                    Position { x, y, z },
                ));
            }
        }
    }
}
