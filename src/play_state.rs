use crate::{
    data_model::{Action, ActionMovement, ActionOnSelf, ActionTargeted, CardData, Character},
    play::{begin_turn, end_turn, play_card_unchecked},
    pop_ability::pop_ability_ignore_unsatisfied,
    resolve_action::{
        ActionInputMovement, ActionInputOnSelf, ActionInputTargeted, resolve_action_movement,
        resolve_action_on_self, resolve_action_targeted,
    },
    single_out::single_out,
};

pub enum PlayCardOrEndTurn {
    PlayCard(CardData),
    EndTurn,
}

pub enum Cancelable<T> {
    Some(T),
    Canceled,
}

pub enum PendingInput<T> {
    Some(T),
    Pending,
}

#[rustfmt::skip]
pub trait Input {
    fn poll_action_input_on_self(&mut self, action: &ActionOnSelf) -> PendingInput<ActionInputOnSelf>;
    fn poll_action_input_targeted(&mut self, action: &ActionTargeted) -> PendingInput<ActionInputTargeted>;
    fn poll_action_input_movement(&mut self, action: &ActionMovement) -> PendingInput<ActionInputMovement>;
    fn poll_action_input_on_self_cancelable(&mut self, action: &ActionOnSelf) -> PendingInput<Cancelable<ActionInputOnSelf>>;
    fn poll_action_input_targeted_cancelable(&mut self, action: &ActionTargeted) -> PendingInput<Cancelable<ActionInputTargeted>>;
    fn poll_action_input_movement_cancelable(&mut self, action: &ActionMovement) -> PendingInput<Cancelable<ActionInputMovement>>;
    fn poll_play_card_or_end_turn(&mut self) -> PendingInput<PlayCardOrEndTurn>;
}

#[derive(Debug, Default)]
pub struct PlayState {
    active: usize,
    has_turn: usize,
    remaining_actions: Vec<Action>,
    cancelable: bool,
}

enum ExecutionState {
    Executed,
    Waiting,
    Canceled,
}

pub fn step_play_state(
    play_state: &mut PlayState,
    input: &mut impl Input,
    all_characters: &mut [Character],
) {
    let Some((active_character, mut characters)) = single_out(all_characters, play_state.active)
    else {
        debug_assert!(false);
        return;
    };

    match play_state.remaining_actions.first() {
        Some(action) => {
            let execution_state = execute_action(
                action,
                play_state.cancelable,
                input,
                active_character,
                &mut characters,
            );

            match execution_state {
                ExecutionState::Executed => {
                    play_state.remaining_actions.pop();
                    play_state.cancelable = false;
                }
                ExecutionState::Canceled => {
                    play_state.remaining_actions.clear();
                }
                ExecutionState::Waiting => {}
            }
        }
        None => {
            let last_active = play_state.active;
            loop {
                play_state.active = (play_state.active + 1) % all_characters.len();

                let Some((active_character, characters)) =
                    single_out(all_characters, play_state.active)
                else {
                    debug_assert!(false);
                    break;
                };

                match pop_ability_ignore_unsatisfied(active_character, &characters) {
                    Some(actions) => {
                        play_state.remaining_actions = actions;
                        play_state.cancelable = true;
                        break;
                    }
                    None => {
                        if play_state.active == last_active {
                            play_state.active = play_state.has_turn;
                            match input.poll_play_card_or_end_turn() {
                                PendingInput::Some(PlayCardOrEndTurn::PlayCard(card_data)) => {
                                    play_card_unchecked(active_character, card_data); // TODO: checks
                                }
                                PendingInput::Some(PlayCardOrEndTurn::EndTurn) => {
                                    end_turn(active_character);
                                    play_state.has_turn =
                                        (play_state.has_turn + 1) % all_characters.len();
                                    //TODO: End of turn triggers will have weird ordering with the beginning of turn effects in the current implementation.
                                    begin_turn(
                                        all_characters.get_mut(play_state.has_turn).unwrap(),
                                    );
                                }
                                PendingInput::Pending => {}
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
    input_state: &mut impl Input,
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
        (Action::OnSelf(action), true) => {
            match input_state.poll_action_input_on_self_cancelable(action) {
                PendingInput::Some(Cancelable::Some(_)) => {
                    resolve_action_on_self(action, active_character, characters);
                    ExecutionState::Executed
                }
                PendingInput::Some(Cancelable::Canceled) => ExecutionState::Canceled,
                PendingInput::Pending => ExecutionState::Waiting,
            }
        }
        (Action::Targeted(action), false) => match input_state.poll_action_input_targeted(action) {
            PendingInput::Some(input) => {
                resolve_action_targeted(action, &input, active_character, characters);
                ExecutionState::Executed
            }
            PendingInput::Pending => ExecutionState::Waiting,
        },
        (Action::Targeted(action), true) => {
            match input_state.poll_action_input_targeted_cancelable(action) {
                PendingInput::Some(Cancelable::Some(input)) => {
                    resolve_action_targeted(action, &input, active_character, characters);
                    ExecutionState::Executed
                }

                PendingInput::Some(Cancelable::Canceled) => ExecutionState::Canceled,
                PendingInput::Pending => ExecutionState::Waiting,
            }
        }
        (Action::Movement(action), false) => match input_state.poll_action_input_movement(action) {
            PendingInput::Some(input) => {
                resolve_action_movement(action, &input, active_character);
                ExecutionState::Executed
            }
            PendingInput::Pending => ExecutionState::Waiting,
        },
        (Action::Movement(action), true) => {
            match input_state.poll_action_input_movement_cancelable(action) {
                PendingInput::Some(Cancelable::Some(input)) => {
                    resolve_action_movement(action, &input, active_character);
                    ExecutionState::Executed
                }
                PendingInput::Some(Cancelable::Canceled) => ExecutionState::Canceled,
                PendingInput::Pending => ExecutionState::Waiting,
            }
        }
    }
}
