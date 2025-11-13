use crate::{
    apply_area_effects::apply_area_effects,
    data_model::{ActionMovement, ActionOnSelf, ActionTargeted, Character},
    hex_grid::Pos,
    movement::perform_movement_unchecked,
};

#[derive(Debug, Clone)]
pub struct ActionInputOnSelf {}

#[derive(Debug, Clone)]
pub struct ActionInputTargeted {
    pub target: Pos,
}

#[derive(Debug, Clone)]
pub struct ActionInputMovement {
    pub path: Vec<Pos>,
}

pub fn resolve_action_movement(
    action: &ActionMovement,
    input: &ActionInputMovement,
    character: &mut Character,
) {
    let _ = action;
    perform_movement_unchecked(character, input.path.iter());
}

pub fn resolve_action_targeted<C>(
    action: &ActionTargeted,
    input: &ActionInputTargeted,
    character: &mut Character,
    characters: &mut C,
) where
    for<'c> &'c mut C: IntoIterator<Item = &'c mut Character>,
{
    apply_area_effects(action.effects.iter(), &input.target, characters, character);
}

pub fn resolve_action_on_self<C>(
    action: &ActionOnSelf,
    character: &mut Character,
    characters: &mut C,
) where
    for<'c> &'c mut C: IntoIterator<Item = &'c mut Character>,
{
    let target = character.pos.clone();
    apply_area_effects(action.effects.iter(), &target, characters, character);
}
