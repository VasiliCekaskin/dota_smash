use bevy::{
    prelude::{
        App, AssetServer, Color, Commands, Component, Plugin, Res, Transform,
        Vec2, Vec3,
    },
    transform::TransformBundle,
};
use bevy_prototype_lyon::prelude::StrokeMode;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};
use bevy_rapier2d::prelude::{Collider, Friction, RigidBody};

use crate::WINDOW_WIDTH;

#[derive(Component)]
pub struct Platform;

pub struct MapOnePlugin;
impl Plugin for MapOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shape = shapes::Rectangle {
        extents: Vec2 {
            x: WINDOW_WIDTH,
            y: 50.0,
        },
        ..Default::default()
    };

    // ground
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: -700.0,
                    z: 0.0,
                },
                ..Default::default()
            },
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WINDOW_WIDTH / 2.0, 50.0 / 2.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        });

    // Platform
    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200.0, 50.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, -200.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 80.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 230.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, -50.0, 0.0,
        )));
}
