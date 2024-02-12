use bevy::prelude::*;

use crate::actions::models::{MeleeAttackAction, WalkAction};
use crate::actions::Action;
use crate::vectors::Vector2Int;

pub trait Card: Send + Sync {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>>;
    fn get_label(&self) -> String;
}

#[derive(Component)]
pub struct CardHolder {
    pub card: Box<dyn Card>,
}

pub struct WalkCard;

impl Card for WalkCard {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>> {
        Some(Box::new(WalkAction {
            entity: owner,
            target_position: target?,
        }))
    }

    fn get_label(&self) -> String {
        "Walk".to_string()
    }
}

pub struct MeleeCard {
    pub damage: u32,
}

impl Card for MeleeCard {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>> {
        Some(Box::new(MeleeAttackAction {
            attacker: owner,
            target_position: target?,
            damage: self.damage,
        }))
    }

    fn get_label(&self) -> String {
        format!("Melee\n{} dmg", self.damage)
    }
}
