use bevy::window::close_on_esc;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_boids::configs::*;

use bevy_boids::dots::*;
use bevy_boids::gui::GuiPlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                title: "bevy-boids".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    for group_id in 0..N_GROUPS {
        for i in 0..GROUP_COUNTS[group_id] {
            let x = rng.gen_range(-WORLD_WIDTH..=WORLD_WIDTH);
            let y = rng.gen_range(-WORLD_HEIGHT..=WORLD_HEIGHT);

            commands.spawn(DotBundle {
                dot: Dot,
                appearance: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(DOT_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(GROUP_COLORS[group_id])),
                    transform: Transform::from_translation(Vec3 { x, y, z: 0.0 }),
                    ..default()
                },
                velocity: Velocity(Vec2::ZERO),
                group: Group(group_id),
            });
        }
    }
}

/*
fn shape_movement(time: Res<Time>, mut shape_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut dir, mut transform) in shape_position.iter_mut() {}
}
*/
