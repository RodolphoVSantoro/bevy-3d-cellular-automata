use crate::config::ROTATE_SPEED;
use crate::rules::{should_decay, should_spawn};

use crate::types::RotateTimer;
use crate::{
    config::{ALIVE_COLOR, DEAD_COLOR, DECAY_TICKS, MAX_HEIGHT, MAX_WIDTH},
    types::{Board, FrameTimer, Position},
};

use bevy::prelude::*;
use std::cmp;

/**
 * Returns a range that is limited to the positive range of \[0, max\]
 */
fn limited_positive_range(start: usize, end: usize, max: usize) -> std::ops::Range<usize> {
    return cmp::max(start, 0)..cmp::min(end + 1, max);
}

fn count_cell_neighbors(board: &mut ResMut<Board>, x: usize, y: usize) {
    board.0[x][y].neighbors = 0;
    for adjacent_x in limited_positive_range(x - 1, x + 1, MAX_WIDTH) {
        for adjacent_y in limited_positive_range(y - 1, y + 1, MAX_HEIGHT) {
            if adjacent_x == x && adjacent_y == y {
                continue;
            }

            if !board.0[adjacent_x][adjacent_y].dead {
                board.0[x][y].neighbors += 1;
            }
        }
    }
}

fn count_neighbors(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            count_cell_neighbors(board, x, y);
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
            let cell = &mut board.0[x][y];
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
                        return position.x == x && position.y == y;
                    })
                    .expect("Cell sprite not found");
                if let Some(material) = materials.get_mut(material_handle) {
                    material.base_color = ALIVE_COLOR;
                }
            }
        }
    }
}

fn decay_cells(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x][y];
            if (should_decay(cell.neighbors)) && !cell.decaying {
                cell.decaying = true;
            }
            if cell.decaying && cell.decaying_ticks > 0 {
                cell.decaying_ticks -= 1;
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
            let cell = &mut board.0[x][y];
            if cell.dead {
                continue;
            }
            if cell.decaying && cell.decaying_ticks == 0 {
                cell.dead = true;
                let (material_handle, _) = material_handles
                    .iter_mut()
                    .find(|(_, position)| -> bool {
                        return position.x == x && position.y == y;
                    })
                    .expect("Material not found");
                if let Some(material) = materials.get_mut(material_handle) {
                    material.base_color = DEAD_COLOR;
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
