use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::ActorQueue,
    board::components::Position,
    pieces::components::{Actor, Health, Occupier, Piece, PrioritizedAction},
    states::MainState,
    vectors::Vector2Int,
};

use self::cards::CardHolder;

mod cards;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeckEvent>()
            .add_event::<PlayerActionEvent>()
            .add_systems(OnEnter(MainState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    select_card.run_if(on_event::<DeckEvent>()),
                    dispatch_card.run_if(on_event::<DeckEvent>()),
                ),
            );
    }
}

pub enum DeckEventKind {
    // emit from the input system to mark active card in the deck
    SelectCard(Entity),
    // emit from the input system to use the card with optional target coordinate
    UseCard(Option<Vector2Int>),
}

#[derive(Event)]
pub struct DeckEvent(pub DeckEventKind);

#[derive(Event)]
pub struct PlayerActionEvent;

#[derive(Component)]
pub struct Player;

#[derive(Default, Resource)]
pub struct Deck {
    pub cards: Vec<Entity>,
    pub current_card: Option<Entity>,
}

fn spawn_player(mut commands: Commands) {
    let walk_card = commands
        .spawn(CardHolder {
            card: Box::new(cards::WalkCard),
        })
        .id();
    let melee_card = commands
        .spawn(CardHolder {
            card: Box::new(cards::MeleeCard { damage: 1 }),
        })
        .id();

    commands.insert_resource(Deck {
        cards: vec![walk_card, melee_card],
        ..Default::default()
    });
    commands.spawn((
        Actor::default(),
        Health { value: 10 },
        Occupier,
        Piece {
            kind: "Player".to_string(),
        },
        Player,
        Position {
            v: Vector2Int { x: 0, y: 0 },
        },
    ));
}

pub fn select_card(mut event_deck: EventReader<DeckEvent>, mut deck: ResMut<Deck>) {
    for event in event_deck.read() {
        if let DeckEvent(DeckEventKind::SelectCard(entity)) = event {
            deck.current_card = Some(*entity);
        }
    }
}

pub fn dispatch_card(
    mut event_deck: EventReader<DeckEvent>,
    mut event_action: EventWriter<PlayerActionEvent>,
    deck: Res<Deck>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    card_query: Query<&CardHolder>,
    mut queue: ResMut<ActorQueue>,
) {
    for event in event_deck.read() {
        if let DeckEvent(DeckEventKind::UseCard(target)) = event {
            let Ok((entity, mut actor)) = player_query.get_single_mut() else {
                return;
            };
            let Some(card_entity) = deck.current_card else {
                return;
            };
            let Ok(card_holder) = card_query.get(card_entity) else {
                continue;
            };
            let Some(action) = card_holder.card.get_action(entity, *target) else {
                continue;
            };

            actor.potential_actions = vec![PrioritizedAction { action, priority: 0 }];

            queue.0 = VecDeque::from([entity]);
            event_action.send(PlayerActionEvent);
        }
    }
}
