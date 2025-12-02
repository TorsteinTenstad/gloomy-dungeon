#![cfg(test)]

use crate::{
    cards::Card,
    data_model::{Ability, Action, Character},
    hex_grid::PosAxial,
    play::{begin_turn, end_turn, play_card_unchecked},
    pop_ability::{PopAbilityResult, pop_ability},
    resolve_action::{
        ActionInputMovement, ActionInputOnSelf, ActionInputTargeted, resolve_action_movement,
        resolve_action_on_self, resolve_action_targeted,
    },
};

#[derive(Debug, Clone)]
pub enum ActionInput {
    OnSelf(ActionInputOnSelf),
    Targeted(ActionInputTargeted),
    Movement(ActionInputMovement),
}

pub fn single_targeted_input(target: PosAxial) -> Vec<ActionInput> {
    vec![ActionInput::Targeted(ActionInputTargeted { target })]
}

pub fn single_movement_input(path: Vec<PosAxial>) -> Vec<ActionInput> {
    vec![ActionInput::Movement(ActionInputMovement { path })]
}

// To override "has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis"
#[allow(dead_code)]
#[derive(Debug)]
pub enum TestSetupError {
    TryingToResolveActionWithoutInput {
        action: Action,
    },
    TryingToResolveActionWithWrongInputType {
        action: Action,
        input: ActionInput,
    },
    PlayedCardWithRemainingAbilities {
        card: Card,
        remaining_abilities: Vec<Ability>,
    },
}

pub fn resolve_remaining_abilities<'a, C, I>(
    character: &mut Character,
    characters: &mut C,
    inputs: I,
) -> Result<(), TestSetupError>
where
    for<'b> &'b C: IntoIterator<Item = &'b Character>,
    for<'b> &'b mut C: IntoIterator<Item = &'b mut Character>,
    I: Iterator<Item = &'a ActionInput>,
{
    resolve_abilities(character, characters, inputs, usize::MAX)
}

pub fn resolve_abilities<'a, C, I>(
    character: &mut Character,
    characters: &mut C,
    inputs: I,
    ability_limit: usize,
) -> Result<(), TestSetupError>
where
    for<'b> &'b C: IntoIterator<Item = &'b Character>,
    for<'b> &'b mut C: IntoIterator<Item = &'b mut Character>,
    I: Iterator<Item = &'a ActionInput>,
{
    let mut inputs = inputs;
    let mut count = 0;
    while count < ability_limit {
        count += 1;
        match pop_ability(character, &(*characters)) {
            PopAbilityResult::NoRemainingAbilities => {
                break;
            }
            PopAbilityResult::NextAbilityDoesNotSatisfyPrecondition => {}
            PopAbilityResult::Actions { actions } => {
                for action in actions {
                    let action_clone = action.clone();
                    match action {
                        Action::OnSelf(action) => {
                            resolve_action_on_self(&action, character, characters);
                        }
                        Action::Targeted(action) => {
                            let input = match inputs.next() {
                                Some(ActionInput::Targeted(input)) => input,
                                Some(input) => {
                                    return Err(
                                        TestSetupError::TryingToResolveActionWithWrongInputType {
                                            action: action_clone,
                                            input: input.clone(),
                                        },
                                    );
                                }
                                None => {
                                    return Err(
                                        TestSetupError::TryingToResolveActionWithoutInput {
                                            action: action_clone,
                                        },
                                    );
                                }
                            };
                            resolve_action_targeted(&action, input, character, characters);
                        }
                        Action::Movement(action) => {
                            let input = match inputs.next() {
                                Some(ActionInput::Movement(input)) => input,
                                Some(input) => {
                                    return Err(
                                        TestSetupError::TryingToResolveActionWithWrongInputType {
                                            action: action_clone,
                                            input: input.clone(),
                                        },
                                    );
                                }
                                None => {
                                    return Err(
                                        TestSetupError::TryingToResolveActionWithoutInput {
                                            action: action_clone,
                                        },
                                    );
                                }
                            };
                            resolve_action_movement(&action, input, character);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn play_card_with_inputs<'a, C, I>(
    card: Card,
    character: &mut Character,
    characters: &mut C,
    inputs: I,
) -> Result<(), TestSetupError>
where
    for<'b> &'b C: IntoIterator<Item = &'b Character>,
    for<'b> &'b mut C: IntoIterator<Item = &'b mut Character>,
    I: Iterator<Item = &'a ActionInput>,
{
    if !character.remaining_abilities.is_empty() {
        return Err(TestSetupError::PlayedCardWithRemainingAbilities {
            card,
            remaining_abilities: character.remaining_abilities.clone(),
        });
    }
    let abilities_to_resolve = play_card_unchecked(character, card.data());
    resolve_abilities(character, characters, inputs, abilities_to_resolve)
}

pub fn end_and_begin_turn(character: &mut Character) {
    end_turn(character);
    begin_turn(character);
}
