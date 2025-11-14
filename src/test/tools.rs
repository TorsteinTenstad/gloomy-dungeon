#![cfg(test)]

use crate::{
    data_model::{Ability, Action, Card, Character},
    hex_grid::Pos,
    play::play_card_unchecked,
    pop_ability::{PopAbilityResult, pop_ability},
    precondition::precondition_is_met,
    resolve_action::{
        ActionInputMovement, ActionInputOnSelf, ActionInputTargeted, resolve_action_movement,
        resolve_action_on_self, resolve_action_targeted,
    },
};

#[derive(Default)]
pub struct Inputs {
    pub input_movement: Vec<ActionInputMovement>,
    pub input_targeted: Vec<ActionInputTargeted>,
}

#[derive(Debug, Clone)]
pub enum ActionInput {
    OnSelf(ActionInputOnSelf),
    Targeted(ActionInputTargeted),
    Movement(ActionInputMovement),
}

pub fn single_targeted_input(target: Pos) -> Vec<ActionInput> {
    vec![ActionInput::Targeted(ActionInputTargeted { target })]
}

pub fn single_movement_input(path: Vec<Pos>) -> Vec<ActionInput> {
    vec![ActionInput::Movement(ActionInputMovement { path })]
}

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
