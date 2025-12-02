#![cfg(test)]
use crate::{
    cards::Card,
    data_model::Character,
    hex_grid::PosAxial,
    play::end_turn,
    test::tools::{play_card_with_inputs, single_movement_input},
    turn_stats::TurnStat,
};

#[test]
pub fn test_engine_turn_stat_moved_tiles() {
    let character = &mut Character {
        stamina_current: 10,
        ..Default::default()
    };

    assert_eq!(character.turn_stats.get(0, &TurnStat::SpacesMoved), 0);

    play_card_with_inputs(
        Card::Step,
        character,
        &mut vec![],
        single_movement_input(vec![]).iter(),
    )
    .unwrap();

    assert_eq!(character.turn_stats.get(0, &TurnStat::SpacesMoved), 0);

    play_card_with_inputs(
        Card::Step,
        character,
        &mut vec![],
        single_movement_input(vec![PosAxial::new(0, 1), PosAxial::new(1, 0)]).iter(),
    )
    .unwrap();

    assert_eq!(character.turn_stats.get(0, &TurnStat::SpacesMoved), 2);

    end_turn(character);

    assert_eq!(character.turn_stats.get(0, &TurnStat::SpacesMoved), 0);
    assert_eq!(character.turn_stats.get(1, &TurnStat::SpacesMoved), 2);
}
