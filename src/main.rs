use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, shape_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    LEFT,
    RIGHT,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Direction::RIGHT,
    ));
}

fn shape_movement(time: Res<Time>, mut shape_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut dir, mut transform) in shape_position.iter_mut() {
        match *dir {
            Direction::LEFT => transform.translation.x -= 100.0 * time.delta_seconds(),
            Direction::RIGHT => transform.translation.x += 100.0 * time.delta_seconds(),
        }

        if transform.translation.x > 200.0 {
            *dir = Direction::LEFT;
        } else if transform.translation.x < -200.0 {
            *dir = Direction::RIGHT;
        }
    }
}
