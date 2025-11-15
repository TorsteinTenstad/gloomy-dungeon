use crate::{
    apply_area_effects::push_triggered_abilities,
    data_model::Character,
    hex_grid::{Pos, distance},
    turn_stats::TurnStat,
};

pub fn perform_movement_unchecked<'a, P>(character: &mut Character, path: P)
where
    P: Iterator<Item = &'a Pos>,
{
    // TODO: Check path is clear enough
    for pos in path {
        debug_assert_eq!(distance(&character.pos, pos), 1); // TODO: We could represent a path as a series of directions to eliminate this failure case
        character.pos = pos.clone();
        *character.turn_stats.get_current_mut(TurnStat::SpacesMoved) += 1;
        // TODO: Resolve environment effects
    }
    push_triggered_abilities(character, |x| x.movement_action);
}
