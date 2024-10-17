use crate::config::{MAX_DEPTH, ROTATE_SPEED};
use crate::rules::{should_decay, should_spawn};

use crate::types::RotateTimer;
use crate::{
    config::{alive_color, DEAD_COLOR, DECAY_TICKS, MAX_HEIGHT, MAX_WIDTH},
    types::{Board, FrameTimer, Position},
};

use bevy::prelude::*;

fn count_cell_neighbors(board: &mut ResMut<Board>, x: usize, y: usize, z: usize) {
    let compare_xyz: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    board.0[x][y][z].neighbors = 0;
    for xyz in compare_xyz {
        let adjacent_x = x as i32 + xyz.0;
        if adjacent_x < 0 || adjacent_x >= MAX_WIDTH as i32 {
            continue;
        }
        let adjacent_y = y as i32 + xyz.1;
        if adjacent_y < 0 || adjacent_y >= MAX_HEIGHT as i32 {
            continue;
        }
        let adjacent_z = z as i32 + xyz.2;
        if adjacent_z < 0 || adjacent_z >= MAX_DEPTH as i32 {
            continue;
        }

        if !board.0[adjacent_x as usize][adjacent_y as usize][adjacent_z as usize].dead {
            board.0[x][y][z].neighbors += 1;
        }
    }
}

fn count_neighbors(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            for z in 0..MAX_DEPTH {
                count_cell_neighbors(board, x, y, z);
            }
        }
    }
}

fn spawn_cells(
    board: &mut ResMut<Board>,
    material_handles: &mut Query<(&Handle<StandardMaterial>, &Position)>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            for z in 0..MAX_DEPTH {
                let cell = &mut board.0[x][y][z];
                if !cell.dead {
                    continue;
                }

                if should_spawn(cell.neighbors) {
                    cell.decaying = false;
                    cell.dead = false;
                    cell.decaying_ticks = DECAY_TICKS;
                    let (material_handle, _) = material_handles
                        .iter_mut()
                        .find(|(_, position)| -> bool {
                            return position.x == x && position.y == y && position.z == z;
                        })
                        .expect("Cell sprite not found");
                    if let Some(material) = materials.get_mut(material_handle) {
                        let color = alive_color(z);
                        material.base_color = color;
                    }
                }
            }
        }
    }
}

fn decay_cells(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            for z in 0..MAX_DEPTH {
                let cell = &mut board.0[x][y][z];
                if (should_decay(cell.neighbors)) && !cell.decaying {
                    cell.decaying = true;
                }
                if cell.decaying && cell.decaying_ticks > 0 {
                    cell.decaying_ticks -= 1;
                }
            }
        }
    }
}

fn kill_cells(
    board: &mut ResMut<Board>,
    material_handles: &mut Query<(&Handle<StandardMaterial>, &Position)>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            for z in 0..MAX_DEPTH {
                let cell = &mut board.0[x][y][z];
                if cell.dead {
                    continue;
                }
                if cell.decaying && cell.decaying_ticks == 0 {
                    cell.dead = true;
                    let (material_handle, _) = material_handles
                        .iter_mut()
                        .find(|(_, position)| -> bool {
                            return position.x == x && position.y == y && position.z == z;
                        })
                        .expect("Material not found");
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = DEAD_COLOR;
                    }
                }
            }
        }
    }
}

pub fn process_tick(
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    mut board: ResMut<Board>,
    mut material_handles: Query<(&Handle<StandardMaterial>, &Position)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    count_neighbors(&mut board);
    spawn_cells(&mut board, &mut material_handles, &mut materials);
    decay_cells(&mut board);
    kill_cells(&mut board, &mut material_handles, &mut materials);
}

pub fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut timer: ResMut<RotateTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut transform = query.single_mut();

    transform.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_y(time.delta_seconds() * ROTATE_SPEED),
    );
}
