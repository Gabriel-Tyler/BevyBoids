use bevy::prelude::*;

use crate::configs::{WORLD_HEIGHT, WORLD_WIDTH};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_world_boundary);
    }
}

fn draw_world_boundary(mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2::ZERO,
        0.0,
        Vec2 {
            x: WORLD_WIDTH * 2.0,
            y: WORLD_HEIGHT * 2.0,
        },
        Color::WHITE,
    );
}
