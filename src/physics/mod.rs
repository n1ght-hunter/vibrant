use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugins;

impl Plugin for PhysicsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            #[cfg(feature = "debug")]
            RapierDebugRenderPlugin::default(),
        ));
    }
}
