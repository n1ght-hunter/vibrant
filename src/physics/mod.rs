mod physics_replace_proxies;
mod utils;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_gltf_blueprints::GltfBlueprintsSet;

use physics_replace_proxies::physics_replace_proxies;

use self::physics_replace_proxies::AutoAABBCollider;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            #[cfg(feature = "debug")]
            PhysicsDebugPlugin::default(),
        ))
        .register_type::<AutoAABBCollider>()
        .register_type::<physics_replace_proxies::Collider>()
        .add_systems(
            Update,
            physics_replace_proxies.after(GltfBlueprintsSet::AfterSpawn),
        );
    }
}
