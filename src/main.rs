mod fps_temp;
pub mod physics;
pub mod player;

use std::f32::consts::TAU;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    gltf::Gltf,
    log::LogPlugin,
    prelude::*,
    render::{camera::Exposure, view::NoFrustumCulling},
    window::{CursorGrabMode, PrimaryWindow, WindowTheme},
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_dev_console::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_framepace::{FramepacePlugin, FramepaceSettings};
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_gltf_save_load::SaveLoadPlugin;
use bevy_rapier3d::prelude::*;
use bevy_registry_export::ExportRegistryPlugin;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
// use controller::{
//     CameraConfig, FpsController, FpsControllerInput, FpsControllerPlugin, LogicalPlayer,
//     RenderPlayer,
// };
use fps_temp::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};

struct BlenderPlugins;

impl Plugin for BlenderPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExportRegistryPlugin::default(),
            SaveLoadPlugin::default(),
            BlueprintsPlugin {
                legacy_mode: false,
                library_folder: "models/library".into(),
                format: GltfFormat::GLB,
                aabbs: true,
                ..Default::default()
            },
        ));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "models/map1.glb")]
    map1: Handle<Gltf>,
    #[asset(path = "models/library/person.glb")]
    person: Handle<Gltf>,
}

struct ExtraPlugins;

impl Plugin for ExtraPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DevConsolePlugin,
            NoCameraPlayerPlugin,
            AtmospherePlugin,
            HookPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            FramepacePlugin,
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((
            ConsoleLogPlugin::default(),
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vibrant".into(),
                        name: Some("Vibrant".into()),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        // visible: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            BlenderPlugins,
            ExtraPlugins,
            physics::PhysicsPlugins,
            player::PlayerPlugin,
            #[cfg(feature = "editor")]
            bevy_editor_pls::EditorPlugin::default(),
        ))
        .register_type::<Player>()
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<MyAssets>(),
        )
        .add_systems(
            OnEnter(MyStates::Next),
            (spawn_world, spawn_camera, setup_fps_counter),
        )
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .run();
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Player;

// fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
//     let player = (
//         HookedSceneBundle {
//             scene: SceneBundle {
//                 scene: assets.load("models\\map1.glb#Scene0"),
//                 ..default()
//             },
//             hook: SceneHook::new(|entity, commands| {
//                 if entity.get::<Handle<Mesh>>().is_some() {
//                     commands.insert(NoFrustumCulling);
//                 }
//             }),
//         },
//         Player,
//     );

//     commands.spawn(player);
// }

fn spawn_camera(mut commands: Commands) {
    // let camera = (
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     FlyCam,
    //     AtmosphereCamera::default(),
    // );
    // commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    assets: Res<MyAssets>,
    models: Res<Assets<Gltf>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0 * 10000.0,
            // shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0 * 1000.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 5.0, 0.0),
    //     ..default()
    // });

    let person = models.get(assets.person.clone()).unwrap();

    // commands.spawn((
    //     HookedSceneBundle {
    //         scene: SceneBundle {
    //             scene: person.scenes[0].clone(),
    //             ..default()
    //         },
    //         hook: SceneHook::new(|entity, commands| {
    //             if entity.get::<Handle<Mesh>>().is_some() {
    //                 commands.insert(NoFrustumCulling);
    //             }
    //         }),
    //     },
    //     Name::new("person"),
    // ));

    let mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)));

    let id = mesh.id();

    commands.spawn(PbrBundle {
        mesh: mesh,
        material: materials.add(Color::DARK_GREEN),
        ..default()
    });

    commands.spawn((
        Collider::from_bevy_mesh(&meshes.get(id).unwrap(), &ComputedColliderShape::TriMesh)
            .unwrap(),
        RigidBody::Fixed,
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    let mesh = meshes.add(Mesh::from(Cuboid::new(
        ground_size,
        ground_height,
        ground_size,
    )));

    let m = meshes.get(&mesh);

    let floor_c = Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).unwrap();

    let floor = PbrBundle {
        mesh: mesh,
        material: materials.add(Color::DARK_GREEN),
        ..default()
    };

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
            floor,
        ))
        .insert(floor_c);

    /*
     * Create the cubes
     */
    let num = 8;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;

                // Build the rigid body.
                commands.spawn((
                    TransformBundle::from(Transform::from_xyz(x, y, z)),
                    RigidBody::Dynamic,
                    Collider::cuboid(rad, rad, rad),
                ));
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}

pub fn cast_ray(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            QueryFilter::only_dynamic(),
        );

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            // will get an event.
            let color = Color::BLUE;
            commands.entity(entity).insert(ColliderDebugColor(color));
        }
    }
}
