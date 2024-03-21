mod controller;
mod keybinds;

use std::f32::consts::TAU;

use controller::*;

use bevy::{prelude::*, render::camera::Exposure, window::CursorGrabMode};
use bevy_atmosphere::plugin::AtmosphereCamera;

use bevy_xpbd_3d::{math::*, prelude::*};

use crate::MyStates;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((keybinds::PlayerKeyBindsPlugin, CharacterControllerPlugin))
            .add_systems(OnEnter(MyStates::Next), spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let e = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..default()
            },
            CharacterControllerBundle::new(Collider::capsule( 1.5, 0.4)).with_movement(
                30.0,
                0.92,
                7.0,
                (30.0 as Scalar).to_radians(),
            ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(2.0),
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            exposure: Exposure::SUNLIGHT,
            ..default()
        },
        RenderPlayer(e),
        AtmosphereCamera::default(),
    ));
}
