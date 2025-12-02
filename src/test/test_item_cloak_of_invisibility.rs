#![cfg(test)]
use std::iter;

use crate::{
    cards::Card,
    data_model::{Character, CharacterTeam, Condition},
    hex_grid::PosAxial,
    items::Item,
    test::tools::{
        end_and_begin_turn, play_card_with_inputs, resolve_remaining_abilities,
        single_targeted_input,
    },
};

#[test]
pub fn test_item_cloak_of_invisibility() {
    let without_item_alone = &mut Character {
        pos: PosAxial::new(-2, 0),
        team: CharacterTeam::Player,
        stamina_current: 10,
        equipped_items: vec![],
        ..Default::default()
    };

    let with_item_alone = &mut Character {
        pos: PosAxial::new(-4, 0),
        team: CharacterTeam::Player,
        stamina_current: 10,
        equipped_items: vec![Item::CloakOfInvisibility],
        ..Default::default()
    };

    let with_item_not_alone = &mut Character {
        pos: PosAxial::new(0, 0),
        team: CharacterTeam::Player,
        stamina_current: 10,
        equipped_items: vec![Item::CloakOfInvisibility],
        ..Default::default()
    };

    let enemy_pos = PosAxial::new(1, 0);
    let enemy = &mut [Character {
        team: CharacterTeam::Monster,
        pos: enemy_pos.clone(),
        equipped_items: vec![],
        ..Default::default()
    }];

    let input = single_targeted_input(enemy_pos);
    play_card_with_inputs(Card::SteadyShot, without_item_alone, enemy, input.iter()).unwrap();
    play_card_with_inputs(Card::SteadyShot, with_item_alone, enemy, input.iter()).unwrap();
    play_card_with_inputs(Card::SteadyShot, with_item_not_alone, enemy, input.iter()).unwrap();

    resolve_remaining_abilities(without_item_alone, enemy, iter::empty()).unwrap();
    resolve_remaining_abilities(with_item_alone, enemy, iter::empty()).unwrap();
    resolve_remaining_abilities(with_item_not_alone, enemy, iter::empty()).unwrap();

    assert_eq!(without_item_alone.conditions.get(&Condition::Fragile), 0);
    assert_eq!(with_item_alone.conditions.get(&Condition::Fragile), 1);
    assert_eq!(with_item_not_alone.conditions.get(&Condition::Fragile), 1);

    end_and_begin_turn(without_item_alone);
    end_and_begin_turn(with_item_alone);
    end_and_begin_turn(with_item_not_alone);

    resolve_remaining_abilities(without_item_alone, enemy, iter::empty()).unwrap();
    resolve_remaining_abilities(with_item_alone, enemy, iter::empty()).unwrap();
    resolve_remaining_abilities(with_item_not_alone, enemy, iter::empty()).unwrap();

    assert_eq!(without_item_alone.conditions.get(&Condition::Invisible), 0);
    assert_eq!(with_item_alone.conditions.get(&Condition::Invisible), 1);
    assert_eq!(with_item_not_alone.conditions.get(&Condition::Invisible), 0);
}
