use bevy::{gltf::Gltf, prelude::*};


use crate::{MyAssets, MyStates};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Next), spawn_world);
    }
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

    // let id = mesh.id();

    // commands.spawn(PbrBundle {
    //     mesh: mesh,
    //     material: assets.floor_material.clone(),
    //     ..default()
    // });\

    let world = models.get(assets.map1.clone()).unwrap();
    let scene = world.scenes[0].clone();


    commands.spawn((
        SceneBundle {
            scene: world.scenes[0].clone(),
            ..default()
        },
        Name::new("world"),
    ));


    // commands.spawn((
    //     Collider::from_bevy_mesh(&meshes.get(id).unwrap(), &ComputedColliderShape::TriMesh)
    //         .unwrap(),
    //     RigidBody::Fixed,
    // ));
}
