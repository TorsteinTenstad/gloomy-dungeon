#![cfg(test)]
use std::iter;

use crate::{
    cards::Card,
    data_model::Character,
    hex_grid::PosAxial,
    items::Item,
    test::tools::{
        end_and_begin_turn, play_card_with_inputs, resolve_remaining_abilities,
        single_movement_input,
    },
};

#[test]
pub fn test_item_stillroot_plate() {
    let character_with_item_moving = &mut Character {
        stamina_current: 10,
        stamina_max: 20,
        equipped_items: vec![Item::StillrootLegs],
        ..Default::default()
    };

    let character_with_item_still = &mut Character {
        stamina_current: 10,
        stamina_max: 20,
        equipped_items: vec![Item::StillrootLegs],
        ..Default::default()
    };

    let character_without_item_still = &mut Character {
        stamina_current: 10,
        stamina_max: 20,
        equipped_items: vec![],
        ..Default::default()
    };

    play_card_with_inputs(
        Card::Step,
        character_with_item_moving,
        &mut vec![],
        single_movement_input(vec![PosAxial::new(0, 1)]).iter(),
    )
    .unwrap();

    play_card_with_inputs(
        Card::Step,
        character_with_item_still,
        &mut vec![],
        single_movement_input(vec![]).iter(),
    )
    .unwrap();

    play_card_with_inputs(
        Card::Step,
        character_without_item_still,
        &mut vec![],
        single_movement_input(vec![]).iter(),
    )
    .unwrap();

    end_and_begin_turn(character_with_item_moving);
    end_and_begin_turn(character_with_item_still);
    end_and_begin_turn(character_without_item_still);

    resolve_remaining_abilities(character_with_item_moving, &mut vec![], iter::empty()).unwrap();
    resolve_remaining_abilities(character_with_item_still, &mut vec![], iter::empty()).unwrap();
    resolve_remaining_abilities(character_without_item_still, &mut vec![], iter::empty()).unwrap();

    assert_eq!(
        character_with_item_moving.stamina_current,
        character_without_item_still.stamina_current
    );

    assert_eq!(
        character_with_item_moving.stamina_current + 1,
        character_with_item_still.stamina_current
    );
}
