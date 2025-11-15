#![cfg(test)]
use crate::{
    data_model::{Card, Character, Condition},
    hex_grid::Pos,
    items::Item,
    test::tools::{play_card_with_inputs, single_targeted_input},
};

#[test]
pub fn test_item_shroud_of_the_poison_feeder() {
    let character = &mut Character {
        stamina_current: 10,
        health_current: 1,
        health_max: 10,
        equipped_items: vec![Item::ShroudOfThePoisonFeeder],
        ..Default::default()
    };
    play_card_with_inputs(
        Card::PoisonCloud,
        character,
        &mut vec![],
        single_targeted_input(Pos::default()).iter(),
    )
    .unwrap();

    // Test fails because the condition is transformed both ways within the same effect resolution.
    // Needs design work to define show this should work.
    assert_eq!(character.conditions.get(&Condition::Poison), 0);
    assert_eq!(character.conditions.get(&Condition::Regen), 3);
}
