use bevy::prelude::*;

use crate::{
    board::{components::Position, CurrentBoard},
    pieces::components::{Health, Occupier},
    vectors::Vector2Int,
};

use super::Action;

pub struct WalkAction {
    pub entity: Entity,
    pub target_position: Vector2Int,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.v == self.target_position)
        {
            return Err(());
        };
        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return Err(());
        };
        if !board.tiles.contains_key(&self.target_position) {
            return Err(());
        };

        let Some(mut position) = world.get_mut::<Position>(self.entity) else {
            return Err(());
        };
        position.v = self.target_position;
        Ok(vec![])
    }
}

pub struct MeleeAttackAction {
    pub attacker: Entity,
    pub target_position: Vector2Int,
    pub damage: u32,
}

impl Action for MeleeAttackAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;

        if attacker_position.v.manhattan(self.target_position) > 1 {
            return Err(());
        }

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, p)| p.v == self.target_position)
            .collect::<Vec<_>>();

        if target_entities.is_empty() {
            return Err(());
        };

        let result = target_entities
            .iter()
            .map(|e| {
                Box::new(DamageAction {
                    target: e.0,
                    damage: self.damage,
                }) as Box<dyn Action>
            })
            .collect::<Vec<_>>();
        Ok(result)
    }
}

pub struct DamageAction {
    pub target: Entity,
    pub damage: u32,
}

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let mut health = world.get_mut::<Health>(self.target).ok_or(())?;
        health.value = health.value.saturating_sub(self.damage);
        if health.value == 0 {
            world.despawn(self.target);
        }
        Ok(vec![])
    }
}
