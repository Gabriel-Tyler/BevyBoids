use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct DotPlugin;

#[derive(Component)]
pub struct Dot;
#[derive(Component)]
pub struct Velocity(pub Vec2);
#[derive(Component)]
pub struct Group(pub usize);

#[derive(Bundle)]
pub struct DotBundle {
    pub dot: Dot,
    pub appearance: MaterialMesh2dBundle<ColorMaterial>,
    pub velocity: Velocity,
    pub group: Group,
}
