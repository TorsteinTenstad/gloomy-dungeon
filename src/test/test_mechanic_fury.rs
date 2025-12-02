#![cfg(test)]
use std::iter;

use crate::{
    cards::Card, data_model::Character, hex_grid::PosAxial, test::tools::play_card_with_inputs,
};

#[test]
pub fn test_fury() {
    const STARTING_HP: usize = 8;
    const EXPECTED_HP_AFTER_DAMAGE: usize = 0; // LargeStrike does 8 damage;
    let character = &mut Character {
        health_current: STARTING_HP,
        stamina_current: 20,
        ..Default::default()
    };
    let other_characters = &mut [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        // Below characters are out of range
        (1, 1),
        (3, 1),
        (-100, 0),
    ]
    .iter()
    .map(|(r, q)| Character {
        pos: PosAxial::new(*r, *q),
        health_current: STARTING_HP,
        ..Default::default()
    })
    .collect::<Vec<_>>();

    play_card_with_inputs(Card::Fury, character, other_characters, iter::empty()).unwrap();

    play_card_with_inputs(
        Card::LargeStrike,
        character,
        other_characters,
        iter::empty(),
    )
    .unwrap();

    assert_eq!(other_characters[0].health_current, EXPECTED_HP_AFTER_DAMAGE);
    assert_eq!(other_characters[1].health_current, EXPECTED_HP_AFTER_DAMAGE);
    assert_eq!(other_characters[2].health_current, EXPECTED_HP_AFTER_DAMAGE);
    assert_eq!(other_characters[3].health_current, EXPECTED_HP_AFTER_DAMAGE);
    assert_eq!(other_characters[4].health_current, EXPECTED_HP_AFTER_DAMAGE);
    assert_eq!(other_characters[5].health_current, EXPECTED_HP_AFTER_DAMAGE);
    // Below characters where out of range/area, so should be at STARTING_HP
    assert_eq!(other_characters[6].health_current, STARTING_HP);
    assert_eq!(other_characters[7].health_current, STARTING_HP);
    assert_eq!(other_characters[8].health_current, STARTING_HP);
    assert_eq!(character.health_current, STARTING_HP);
}
