use std::fs::File;

use bevy::{core_pipeline::core_2d::graph::input, prelude::*};
use leafwing_input_manager::{prelude::*, user_input::InputKind};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

pub struct PlayerKeyBindsPlugin;

impl Plugin for PlayerKeyBindsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<ActionState<PlayerAction>>()
            .insert_resource(PlayerAction::load_keybinds());
    }
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(
    Reflect,
    Actionlike,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
pub enum PlayerAction {
    /// Move the player
    Move,
    /// look around
    Look,
    /// make the player run
    Run,
    /// make the player jump
    Jump,
    /// make the player crouch
    Crouch,
    /// player primary action
    Primary,
    /// player secondary action
    Secondary,
    /// player open score board
    ScoreBoard,
    /// player melee attack
    Melee,
    /// player reload weapon
    Reload,
    /// interact with the world
    Interact,
}

impl PlayerAction {
    pub fn load_keybinds() -> InputMap<Self> {
        if let Ok(keybinds) = File::open("config/keybinds.ron") {
            if let Ok(config) = from_reader(keybinds) {
                return config;
            }
        }

        let mut input_map = InputMap::default();

        Self::kvm(&mut input_map);
        Self::gamepad(&mut input_map);

        input_map
    }

    // ███╗   ███╗ ██████╗ ██╗   ██╗███████╗███████╗     █████╗ ███╗   ██╗██████╗     ██╗  ██╗███████╗██╗   ██╗██████╗  ██████╗  █████╗ ██████╗ ██████╗
    // ████╗ ████║██╔═══██╗██║   ██║██╔════╝██╔════╝    ██╔══██╗████╗  ██║██╔══██╗    ██║ ██╔╝██╔════╝╚██╗ ██╔╝██╔══██╗██╔═══██╗██╔══██╗██╔══██╗██╔══██╗
    // ██╔████╔██║██║   ██║██║   ██║███████╗█████╗      ███████║██╔██╗ ██║██║  ██║    █████╔╝ █████╗   ╚████╔╝ ██████╔╝██║   ██║███████║██████╔╝██║  ██║
    // ██║╚██╔╝██║██║   ██║██║   ██║╚════██║██╔══╝      ██╔══██║██║╚██╗██║██║  ██║    ██╔═██╗ ██╔══╝    ╚██╔╝  ██╔══██╗██║   ██║██╔══██║██╔══██╗██║  ██║
    // ██║ ╚═╝ ██║╚██████╔╝╚██████╔╝███████║███████╗    ██║  ██║██║ ╚████║██████╔╝    ██║  ██╗███████╗   ██║   ██████╔╝╚██████╔╝██║  ██║██║  ██║██████╔╝
    // ╚═╝     ╚═╝ ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝     ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝

    fn kvm(input_mape: &mut InputMap<Self>) {
        input_mape.insert(Self::Move, VirtualDPad::wasd());
        input_mape.insert(Self::Look, DualAxis::mouse_motion());
        input_mape.insert(Self::Run, KeyCode::ShiftLeft);
        input_mape.insert(Self::Jump, KeyCode::Space);
        input_mape.insert(Self::Crouch, KeyCode::ControlLeft);
        input_mape.insert(Self::Primary, MouseButton::Left);
        input_mape.insert(Self::Secondary, MouseButton::Right);
        input_mape.insert(Self::ScoreBoard, KeyCode::Tab);
        input_mape.insert(Self::Melee, KeyCode::KeyV);
        input_mape.insert(Self::Reload, KeyCode::KeyR);
        input_mape.insert(Self::Interact, KeyCode::KeyF);
    }

    //  ██████╗  █████╗ ███╗   ███╗███████╗    ██████╗  █████╗ ██████╗
    // ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔══██╗██╔══██╗██╔══██╗
    // ██║  ███╗███████║██╔████╔██║█████╗      ██████╔╝███████║██║  ██║
    // ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██╔═══╝ ██╔══██║██║  ██║
    // ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ██║     ██║  ██║██████╔╝
    //  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝    ╚═╝     ╚═╝  ╚═╝╚═════╝

    fn gamepad(input_mape: &mut InputMap<Self>) {
        input_mape.insert(Self::Move, DualAxis::left_stick());
        input_mape.insert(Self::Look, DualAxis::right_stick());
        input_mape.insert(Self::Run, GamepadButtonType::LeftThumb);
        input_mape.insert(Self::Jump, GamepadButtonType::South);
        input_mape.insert(Self::Crouch, GamepadButtonType::East);
        input_mape.insert(Self::Primary, GamepadButtonType::RightTrigger);
        input_mape.insert(Self::Secondary, GamepadButtonType::LeftTrigger);
        input_mape.insert(Self::ScoreBoard, GamepadButtonType::Select);
        input_mape.insert(Self::Melee, GamepadButtonType::RightThumb);
        input_mape.insert(Self::Reload, GamepadButtonType::West);
        input_mape.insert(Self::Interact, GamepadButtonType::North);
    }
}
