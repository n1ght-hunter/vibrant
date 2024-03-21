use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::{
    math::*,
    parry::shape::{Shape, TypedShape},
    prelude::*,
};
use leafwing_input_manager::action_state::ActionState;

use super::keybinds::PlayerAction;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(
                Update,
                (
                    handle_input,
                    update_grounded,
                    movement,
                    apply_movement_damping,
                    fps_controller_render,
                )
                    .chain(),
            )
            .add_systems(Update, manage_cursor);
    }
}

/// camera config
#[derive(Component)]
pub struct CameraConfig {
    pub height_offset: f32,
    pub radius_scale: f32,
}

/// Connects a camera to a player entity.
#[derive(Component, Deref, DerefMut)]
pub struct RenderPlayer(pub Entity);

/// View component for a first person camera.
#[derive(Component)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A settings Component for player entities.
#[derive(Component)]
pub struct PlayerSettings {
    /// capture mouse input
    enable_input: bool,
    /// camera sensitivity
    sensitivity: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            enable_input: false,
            sensitivity: 0.001,
        }
    }
}

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
    player_settings: PlayerSettings,
    view: View,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Direction3d::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
            player_settings: PlayerSettings::default(),
            view: View {
                yaw: 0.0,
                pitch: 0.0,
            },
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

const ANGLE_EPSILON: f32 = 0.001953125;

/// Sends [`MovementAction`] events based on inputs.
fn handle_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<(&mut View, &PlayerSettings)>,
) {
    if action_state.pressed(&PlayerAction::Move) {
        let axis_pair = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .unwrap()
            .xy();

        if axis_pair != Vector2::ZERO {
            movement_event_writer.send(MovementAction::Move(axis_pair));
        }
    }

    if action_state.pressed(&PlayerAction::Jump) {
        movement_event_writer.send(MovementAction::Jump);
    }
    let camera_pan_vector = action_state.axis_pair(&PlayerAction::Look).unwrap();

    for (mut view, settings) in query.iter_mut() {
        let mut mouse_delta = camera_pan_vector.xy();

        mouse_delta *= settings.sensitivity;

        view.pitch = (view.pitch - mouse_delta.y)
            .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
        view.yaw -= mouse_delta.x;
        if view.yaw.abs() > PI {
            view.yaw = view.yaw.rem_euclid(TAU);
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

// ██████╗ ███████╗███╗   ██╗██████╗ ███████╗██████╗
// ██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔════╝██╔══██╗
// ██████╔╝█████╗  ██╔██╗ ██║██║  ██║█████╗  ██████╔╝
// ██╔══██╗██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗
// ██║  ██║███████╗██║ ╚████║██████╔╝███████╗██║  ██║
// ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝

pub fn fps_controller_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    logical_query: Query<
        (&Transform, &Collider, &View, &CameraConfig),
        (With<CharacterController>, Without<RenderPlayer>),
    >,
) {
    for (mut render_transform, render_player) in render_query.iter_mut() {
        if let Ok((logical_transform, collider, controller, camera_config)) =
            logical_query.get(render_player.0)
        {
            if let TypedShape::Capsule(capsule) = collider.shape().as_typed_shape() {
                let camera_height = capsule.segment.b.y
                    + capsule.radius * camera_config.radius_scale
                    + camera_config.height_offset;
                render_transform.translation =
                    logical_transform.translation + Vec3::Y * camera_height;
                render_transform.rotation =
                    Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
            }
        }
    }
}

//  ███████╗██╗  ██╗████████╗██████╗  █████╗
//  ██╔════╝╚██╗██╔╝╚══██╔══╝██╔══██╗██╔══██╗
//  █████╗   ╚███╔╝    ██║   ██████╔╝███████║
//  ██╔══╝   ██╔██╗    ██║   ██╔══██╗██╔══██║
//  ███████╗██╔╝ ██╗   ██║   ██║  ██║██║  ██║
//  ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut PlayerSettings>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}
