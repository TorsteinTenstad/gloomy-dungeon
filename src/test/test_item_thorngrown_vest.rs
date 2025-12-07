#![cfg(test)]
use std::iter;

use crate::{
    cards::Card,
    data_model::{Character, Condition},
    hex_grid::PosAxial,
    items::Item,
    play::end_turn,
    test::tools::{play_card_with_inputs, resolve_remaining_abilities, single_targeted_input},
};

#[test]
pub fn test_item_thorngrown_vest() {
    let item_attack = &mut Character {
        equipped_items: vec![Item::ThorngrownVest],
        stamina_current: 10,
        ..Default::default()
    };
    let item_no_attack = &mut Character {
        equipped_items: vec![Item::ThorngrownVest],
        stamina_current: 10,
        ..Default::default()
    };
    let no_item_no_attack = &mut Character {
        equipped_items: vec![],
        stamina_current: 10,
        ..Default::default()
    };
    let target_pos = PosAxial { r: 0, q: 1 };
    let other_characters = &mut [Character {
        pos: target_pos.clone(),
        ..Default::default()
    }];

    play_card_with_inputs(
        Card::Cut,
        item_attack,
        other_characters,
        single_targeted_input(target_pos.clone()).iter(),
    )
    .unwrap();

    end_turn(item_attack);
    end_turn(item_no_attack);
    end_turn(no_item_no_attack);

    resolve_remaining_abilities(item_attack, other_characters, iter::empty()).unwrap();
    resolve_remaining_abilities(item_no_attack, other_characters, iter::empty()).unwrap();
    resolve_remaining_abilities(no_item_no_attack, other_characters, iter::empty()).unwrap();

    assert_eq!(item_attack.conditions.get(&Condition::Retaliate), 0);
    assert_eq!(item_no_attack.conditions.get(&Condition::Retaliate), 2);
    assert_eq!(no_item_no_attack.conditions.get(&Condition::Retaliate), 0);
}
