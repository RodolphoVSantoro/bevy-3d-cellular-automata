use bevy::prelude::Color;

pub const MAX_WIDTH: usize = 40; // how many cells wide the board is
pub const MAX_HEIGHT: usize = 40; // how many cells tall the board is
pub const MAX_DEPTH: usize = 40; // how many cells deep the board is

pub const TICK_TIME: f32 = 0.0; // how many seconds until next tick is processed

pub const ROTATE_TIME: f32 = 0.16; // how many seconds until the camera rotates
pub const ROTATE_SPEED: f32 = 0.4; // how fast the camera rotates

pub const DECAY_TICKS: u32 = 3; // how many ticks a cell will decay for

pub const DEAD_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.0);
// pub const ALIVE_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 0.5);
pub fn alive_color(z: usize) -> Color {
    let r = 1.0 - (z as f32 / MAX_DEPTH as f32);
    let b = z as f32 / MAX_DEPTH as f32;
    return Color::rgba(r, 0.0, b, 0.1);
}
