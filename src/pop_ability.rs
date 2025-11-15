use crate::{
    data_model::{Action, ActionOnSelf, ActionTargeted, AreaEffect, Character, Condition, Reach},
    hex_grid::{Area, DistanceRange},
    precondition::optional_precondition_is_met,
};

pub enum PopAbilityResult {
    NoRemainingAbilities,
    NextAbilityDoesNotSatisfyPrecondition,
    Actions { actions: Vec<Action> },
}

pub fn pop_ability_ignore_unsatisfied<C>(
    source_character: &mut Character,
    characters: &C,
) -> Option<Vec<Action>>
where
    for<'c> &'c C: IntoIterator<Item = &'c Character>,
{
    loop {
        match pop_ability(source_character, characters) {
            PopAbilityResult::NoRemainingAbilities => break None,
            PopAbilityResult::NextAbilityDoesNotSatisfyPrecondition => {}
            PopAbilityResult::Actions { actions } => break Some(actions),
        }
    }
}

pub fn pop_ability<'a, C>(source_character: &'a mut Character, characters: C) -> PopAbilityResult
where
    C: IntoIterator<Item = &'a Character>,
{
    match source_character.remaining_abilities.pop() {
        Some(mut ability)
            if optional_precondition_is_met(
                ability.precondition.as_ref(),
                characters,
                source_character,
            ) =>
        {
            PopAbilityResult::Actions {
                actions: ability
                    .actions
                    .drain(..)
                    .map(|action| map_action(source_character, action))
                    .collect(),
            }
        }
        Some(_) => PopAbilityResult::NextAbilityDoesNotSatisfyPrecondition,
        None => PopAbilityResult::NoRemainingAbilities,
    }
}
/*
pub fn check_precondition_and_map_actions(ability: Ability, source_character: &mut Character, characters: C) -> Option<Vec<Action>>
where
    C: IntoIterator<Item = &'a Character>,
{

}
*/

// If a lot of mechanics require action mapping, a generic data model for mapping an action should be considered

pub fn map_action(character: &Character, action: Action) -> Action {
    if character.conditions.has(&Condition::Fury) {
        match filter_map_action_for_fury(&action) {
            Some(mapped_action) => mapped_action,
            None => action,
        }
    } else {
        action
    }
}

pub fn filter_map_action_for_fury(action: &Action) -> Option<Action> {
    match action {
        Action::OnSelf(_) => None,
        Action::Targeted(ActionTargeted {
            reach: Reach::Melee,
            effects,
        }) if effects
            .iter()
            .all(|area_effect| area_effect.area == Area::default()) =>
        {
            Some(Action::OnSelf(ActionOnSelf {
                effects: effects
                    .iter()
                    .map(|effect| AreaEffect {
                        area: Area::Disk(DistanceRange { from: 1, to: 2 }),
                        filter: effect.filter.clone(),
                        effects: effect.effects.clone(),
                    })
                    .collect(),
            }))
        }
        Action::Targeted(_) => None,
        Action::Movement(_) => None,
    }
}
