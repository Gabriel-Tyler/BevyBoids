use bevy::prelude::Color;

pub const WINDOW_WIDTH: usize = 500;
pub const WINDOW_HEIGHT: usize = 500;

pub const WORLD_WIDTH: f32 = 200.0;
pub const WORLD_HEIGHT: f32 = 200.0;

pub const N_GROUPS: usize = 4;
pub const GROUP_COLORS: [Color; N_GROUPS] = [Color::RED, Color::GREEN, Color::BLUE, Color::WHITE];
pub const GROUP_COUNTS: [u32; N_GROUPS] = [20, 20, 20, 20];

pub const DOT_RADIUS: f32 = 2.0;
