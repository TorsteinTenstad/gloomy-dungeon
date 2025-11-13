use crate::{
    data_model::{Ability, Action, Card, Character},
    play::{end_turn, play_card_unchecked},
    precondition::precondition_is_met,
    resolve_action::{
        ActionInputMovement, ActionInputOnSelf, ActionInputTargeted, resolve_action_movement,
        resolve_action_on_self, resolve_action_targeted,
    },
    single_out::single_out,
};

#[derive(Default, Debug)]
pub struct CharacterAbilityState {
    remaining_abilities: Vec<Ability>,
    remaining_actions: Vec<Action>,
    cancelable: bool,
}

pub struct InputState {}

pub enum PlayCardOrEndTurn {
    PlayCard(Card),
    EndTurn,
}

impl InputState {
    pub fn advance_on_self(&mut self) -> Option<ActionInputOnSelf> {
        todo!()
    }
    pub fn advance_targeted(&mut self) -> Option<ActionInputTargeted> {
        todo!()
    }
    pub fn advance_movement(&mut self) -> Option<ActionInputMovement> {
        todo!()
    }
    pub fn advance_on_self_cancelable(&mut self) -> Option<Option<ActionInputOnSelf>> {
        todo!()
    }
    pub fn advance_targeted_cancelable(&mut self) -> Option<Option<ActionInputTargeted>> {
        todo!()
    }
    pub fn advance_movement_cancelable(&mut self) -> Option<Option<ActionInputMovement>> {
        todo!()
    }
    pub fn advance_play_card_or_end_turn(&mut self) -> Option<PlayCardOrEndTurn> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct PlayState {
    characters: Vec<Character>,
    active: usize,
    has_turn: usize,
    last_character_performing_action: usize,
    non_turn_abilities_unresolved: bool,
    remaining_actions: Vec<Action>,
    cancelable: bool,
}

enum ExecutionState {
    Executed,
    Waiting,
    Canceled,
}

pub fn game_loop() {
    let mut input_state = InputState {};
    let mut play_state = PlayState::default();
    loop {
        match single_out(&mut play_state.characters, play_state.active) {
            None => {
                debug_assert!(false);
                play_state.active = 0;
            }
            Some((active_character, mut characters)) => {
                match play_state.remaining_actions.first() {
                    Some(action) => {
                        let execution_state = execute_action(
                            action,
                            play_state.cancelable,
                            &mut input_state,
                            active_character,
                            &mut characters,
                        );

                        match execution_state {
                            ExecutionState::Executed => {
                                play_state.remaining_actions.pop();
                                play_state.non_turn_abilities_unresolved = true;
                                play_state.last_character_performing_action = play_state.active;
                                play_state.cancelable = false;
                            }
                            ExecutionState::Canceled => {
                                play_state.remaining_actions.clear();
                            }
                            ExecutionState::Waiting => {}
                        }
                    }
                    None => {
                        if play_state.non_turn_abilities_unresolved {
                            play_state.active =
                                (play_state.active + 1) % play_state.characters.len();
                            loop {
                                let (active_character, characters) =
                                    single_out(&mut play_state.characters, play_state.active)
                                        .unwrap();
                                match active_character.remaining_abilities.pop() {
                                    Some(ability) => {
                                        if ability.precondition.is_none_or(|precondition| {
                                            precondition_is_met(
                                                &precondition,
                                                &characters,
                                                active_character,
                                            )
                                        }) {
                                            play_state.remaining_actions = ability.actions;
                                            play_state.cancelable = true;
                                        }
                                    }
                                    None => {
                                        play_state.active =
                                            (play_state.active + 1) % play_state.characters.len();
                                    }
                                }
                            }
                        } else {
                            match active_character.remaining_abilities.pop() {
                                Some(ability) => {
                                    if ability.precondition.is_none_or(|precondition| {
                                        precondition_is_met(
                                            &precondition,
                                            &characters,
                                            active_character,
                                        )
                                    }) {
                                        play_state.remaining_actions = ability.actions;
                                        play_state.cancelable = true;
                                    }
                                }
                                None => {
                                    if play_state.active == play_state.has_turn {
                                        match input_state.advance_play_card_or_end_turn() {
                                            Some(PlayCardOrEndTurn::PlayCard(card)) => {
                                                play_card_unchecked(active_character, card.data()); // TODO: checks
                                            }
                                            Some(PlayCardOrEndTurn::EndTurn) => {
                                                end_turn(active_character);
                                                play_state.has_turn = (play_state.has_turn + 1)
                                                    % play_state.characters.len()
                                            }
                                            None => {}
                                        }
                                    } else {
                                        todo!()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn execute_action<C>(
    action: &Action,
    cancelable: bool,
    input_state: &mut InputState,
    active_character: &mut Character,
    characters: &mut C,
) -> ExecutionState
where
    for<'c> &'c mut C: IntoIterator<Item = &'c mut Character>,
{
    match (action, cancelable) {
        (Action::OnSelf(action), false) => {
            resolve_action_on_self(action, active_character, characters);
            ExecutionState::Executed
        }
        (Action::OnSelf(action), true) => match input_state.advance_on_self_cancelable() {
            Some(Some(_)) => {
                resolve_action_on_self(action, active_character, characters);
                ExecutionState::Executed
            }
            Some(None) => ExecutionState::Canceled,
            None => ExecutionState::Waiting,
        },
        (Action::Targeted(action), false) => match input_state.advance_targeted() {
            Some(input) => {
                resolve_action_targeted(action, &input, active_character, characters);
                ExecutionState::Executed
            }
            None => ExecutionState::Waiting,
        },
        (Action::Targeted(action), true) => match input_state.advance_targeted_cancelable() {
            Some(Some(input)) => {
                resolve_action_targeted(action, &input, active_character, characters);
                ExecutionState::Executed
            }
            Some(None) => ExecutionState::Canceled,
            None => ExecutionState::Waiting,
        },
        (Action::Movement(action), false) => match input_state.advance_movement() {
            Some(input) => {
                resolve_action_movement(action, &input, active_character);
                ExecutionState::Executed
            }
            None => ExecutionState::Waiting,
        },
        (Action::Movement(action), true) => match input_state.advance_movement_cancelable() {
            Some(Some(input)) => {
                resolve_action_movement(action, &input, active_character);
                ExecutionState::Executed
            }
            Some(None) => ExecutionState::Canceled,
            None => ExecutionState::Waiting,
        },
    }
}
