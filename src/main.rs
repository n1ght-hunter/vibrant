use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup_cam)
        .add_systems(Startup, spawn_cubes)
        .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..Camera3dBundle::default()
        },
        FlyCam,
    ));
}

fn spawn_cubes(mut commands: Commands, mut materials: ResMut<Assets<Mesh>>) {
    let mesh = materials.add(Cuboid::new(1.0, 1.0, 1.0));

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * 2.,
                    y as f32 * 2.,
                    0.0,
                )),
                ..Default::default()
            });
        }
    }
}
