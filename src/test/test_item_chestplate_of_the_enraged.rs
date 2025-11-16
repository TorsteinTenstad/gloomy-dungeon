#![cfg(test)]
use std::iter;

use crate::{
    cards::Card,
    data_model::{Character, Condition},
    items::Item,
    single_out::single_out,
    test::tools::{play_card_with_inputs, resolve_remaining_abilities},
};

#[test]
pub fn test_item_cloak_of_invisibility() {
    let without_item_damaged = Character {
        equipped_items: vec![],
        ..Default::default()
    };

    let with_item_damaged = Character {
        equipped_items: vec![Item::ChestplateOfTheEnraged],
        ..Default::default()
    };

    let with_item_not_damaged = Character {
        equipped_items: vec![Item::ChestplateOfTheEnraged],
        ..Default::default()
    };

    let source = Character {
        stamina_current: 10,
        equipped_items: vec![],
        ..Default::default()
    };

    let mut all_characters = [
        source,
        without_item_damaged,
        with_item_damaged,
        with_item_not_damaged,
    ];

    *all_characters[3].conditions.get_mut(Condition::Fortified) = 1;

    {
        let (source, mut characters) = single_out(&mut all_characters, 0).unwrap();
        play_card_with_inputs(Card::Whirlwind, source, &mut characters, iter::empty()).unwrap();
    }

    resolve_remaining_abilities(&mut all_characters[1], &mut vec![], iter::empty()).unwrap();
    resolve_remaining_abilities(&mut all_characters[2], &mut vec![], iter::empty()).unwrap();
    resolve_remaining_abilities(&mut all_characters[3], &mut vec![], iter::empty()).unwrap();

    assert_eq!(all_characters[1].conditions.get(&Condition::Strong), 0);
    assert_eq!(all_characters[2].conditions.get(&Condition::Strong), 1);
    assert_eq!(all_characters[3].conditions.get(&Condition::Strong), 0);
}
