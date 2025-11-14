use crate::{
    data_model::{ActionMovement, ActionOnSelf, ActionTargeted},
    play_state::{Cancelable, Input, PendingInput, PlayCardOrEndTurn, PlayState, step_play_state},
    resolve_action::{ActionInputMovement, ActionInputOnSelf, ActionInputTargeted},
};
mod apply_area_effects;
mod cards;
mod character_filter;
mod data_model;
mod enum_map;
mod hex_grid;
mod items;
mod movement;
mod play;
mod play_state;
mod pop_ability;
mod precondition;
mod resolve_action;
mod single_out;
mod test;

fn main() {
    let mut all_characters = vec![];
    let mut input = InputDummy {};
    let mut play_state = PlayState::default();
    loop {
        step_play_state(&mut play_state, &mut input, &mut all_characters);
    }
}

struct InputDummy {}

impl Input for InputDummy {
    fn poll_action_input_on_self(
        &mut self,
        action: &ActionOnSelf,
    ) -> PendingInput<ActionInputOnSelf> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_targeted(
        &mut self,
        action: &ActionTargeted,
    ) -> PendingInput<ActionInputTargeted> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_movement(
        &mut self,
        action: &ActionMovement,
    ) -> PendingInput<ActionInputMovement> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_on_self_cancelable(
        &mut self,
        action: &ActionOnSelf,
    ) -> PendingInput<Cancelable<ActionInputOnSelf>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_targeted_cancelable(
        &mut self,
        action: &ActionTargeted,
    ) -> PendingInput<Cancelable<ActionInputTargeted>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_movement_cancelable(
        &mut self,
        action: &ActionMovement,
    ) -> PendingInput<Cancelable<ActionInputMovement>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_play_card_or_end_turn(&mut self) -> PendingInput<PlayCardOrEndTurn> {
        PendingInput::Pending
    }
}
