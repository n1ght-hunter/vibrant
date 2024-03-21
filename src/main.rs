mod fps_temp;
pub mod physics;
pub mod player;
pub mod world;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    gltf::Gltf,
    log::LogPlugin,
    prelude::*,
    window::{PrimaryWindow, WindowTheme},
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_atmosphere::{plugin::AtmosphereCamera, prelude::AtmospherePlugin};
use bevy_dev_console::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_framepace::FramepacePlugin;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_gltf_save_load::SaveLoadPlugin;

use bevy_registry_export::ExportRegistryPlugin;
use bevy_scene_hook::HookPlugin;
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
    #[asset(standard_material)]
    #[asset(path = "textures/stone.png")]
    floor_material: Handle<StandardMaterial>,
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
        .init_state::<MyStates>()
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
            physics::PhysicsPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            #[cfg(feature = "editor")]
            bevy_editor_pls::EditorPlugin::default(),
        ))
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<MyAssets>(),
        )
        .add_systems(OnEnter(MyStates::Next), (setup_fps_counter))
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .run();
}



// fn spawn_camera(mut commands: Commands) {
//     let camera = (
//         Camera3dBundle {
//             transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//             ..default()
//         },
//         FlyCam,
//         AtmosphereCamera::default(),
//     );
//     commands.spawn(camera);
// }

// pub fn cast_ray(
//     mut commands: Commands,
//     windows: Query<&Window, With<PrimaryWindow>>,
//     rapier_context: Res<RapierContext>,
//     cameras: Query<(&Camera, &GlobalTransform)>,
// ) {
//     let window = windows.single();

//     let Some(cursor_position) = window.cursor_position() else {
//         return;
//     };

//     // We will color in read the colliders hovered by the mouse.
//     for (camera, camera_transform) in &cameras {
//         // First, compute a ray from the mouse position.
//         let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
//             return;
//         };

//         // Then cast the ray.
//         let hit = rapier_context.cast_ray(
//             ray.origin,
//             ray.direction.into(),
//             f32::MAX,
//             true,
//             QueryFilter::only_dynamic(),
//         );

//         if let Some((entity, _toi)) = hit {
//             // Color in blue the entity we just hit.
//             // Because of the query filter, only colliders attached to a dynamic body
//             // will get an event.
//             let color = Color::BLUE;
//             commands.entity(entity).insert(ColliderDebugColor(color));
//         }
//     }
// }
