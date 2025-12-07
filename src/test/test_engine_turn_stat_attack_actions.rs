#![cfg(test)]
use crate::{
    cards::Card,
    data_model::Character,
    hex_grid::PosAxial,
    play::end_turn,
    test::tools::{play_card_with_inputs, single_targeted_input},
    turn_stats::TurnStat,
};

#[test]
pub fn test_engine_turn_stat_attack_actions() {
    let character = &mut Character {
        stamina_current: 10,
        ..Default::default()
    };
    let target_pos = PosAxial { r: 0, q: 1 };
    let other_characters = &mut [Character {
        pos: target_pos.clone(),
        ..Default::default()
    }];

    assert_eq!(character.turn_stats.get(0, &TurnStat::AttackActions), 0);

    play_card_with_inputs(
        Card::Cut,
        character,
        other_characters,
        single_targeted_input(target_pos.clone()).iter(),
    )
    .unwrap();

    assert_eq!(character.turn_stats.get(0, &TurnStat::AttackActions), 1);

    play_card_with_inputs(
        Card::Cut,
        character,
        other_characters,
        single_targeted_input(target_pos.clone()).iter(),
    )
    .unwrap();

    assert_eq!(character.turn_stats.get(0, &TurnStat::AttackActions), 2);

    end_turn(character);

    assert_eq!(character.turn_stats.get(0, &TurnStat::AttackActions), 0);
    assert_eq!(character.turn_stats.get(1, &TurnStat::AttackActions), 2);
}
