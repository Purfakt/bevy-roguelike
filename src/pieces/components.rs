use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
pub struct Walk;

pub struct PrioritizedAction {
    pub action: Box<dyn Action>,
    pub priority: i32,
}

#[derive(Component, Default)]
pub struct Actor {
    pub potential_actions: Vec<PrioritizedAction>,
}

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct MeleeAttack {
    pub damage: u32,
}

#[derive(Component)]
pub struct Occupier;
