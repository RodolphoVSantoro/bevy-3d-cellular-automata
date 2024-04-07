use bevy::prelude::Color;

pub const MAX_WIDTH: usize = 100; // how many cells wide the board is
pub const MAX_HEIGHT: usize = 60; // how many cells tall the board is

pub const TICK_TIME: f32 = 0.1; // how many seconds until next tick is processed

pub const ROTATE_TIME: f32 = 0.16; // how many seconds until the camera rotates
pub const ROTATE_SPEED: f32 = 0.1; // how fast the camera rotates

pub const DECAY_TICKS: u32 = 10; // how many ticks a cell will decay for

pub const DEAD_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.1);
pub const ALIVE_COLOR: Color = Color::rgba(0.75, 0.75, 0.75, 1.0);
