#![cfg(test)]
use crate::{
    cards::Card,
    data_model::{Character, Condition},
    hex_grid::Pos,
    items::Item,
    test::tools::{
        play_card_with_inputs, resolve_remaining_abilities, single_movement_input,
        single_targeted_input,
    },
};

#[test]
pub fn test_item_monks_robe() {
    let pos_character = Pos::new(0, 0);
    let pos_other = Pos::new(0, 2);
    let pos_next_to_other = Pos::new(0, 1);

    let character = &mut Character {
        stamina_current: 10,
        pos: pos_character,
        equipped_items: vec![Item::MonksRobe],
        ..Default::default()
    };
    let other_characters = &mut [Character {
        pos: pos_other.clone(),
        ..Default::default()
    }];

    play_card_with_inputs(
        Card::Step,
        character,
        other_characters,
        single_movement_input(vec![pos_next_to_other]).iter(),
    )
    .unwrap();

    resolve_remaining_abilities(
        character,
        other_characters,
        single_targeted_input(pos_other).iter(),
    )
    .unwrap();

    assert_eq!(character.conditions.get(&Condition::Disarmed), 1);
    assert_eq!(other_characters[0].conditions.get(&Condition::Stunned), 1);
}
