use bevy::{log::LogPlugin, prelude::*};
use bevy_dev_console::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::ExportRegistryPlugin;
use bevy_gltf_save_load::SaveLoadPlugin;

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

fn main() {
    App::new()
        .add_plugins((
            ConsoleLogPlugin::default(),
            DefaultPlugins.build().disable::<LogPlugin>(),
            BlenderPlugins,
            DevConsolePlugin,
            NoCameraPlayerPlugin,
        ))
        .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
        .run();
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("girl.glb#Scene0"),
            // scene: assets.load("girl.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    );

    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
        material: materials.add(Color::DARK_GREEN),
        ..default()
    };

    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(floor);
    commands.spawn(light);
}
