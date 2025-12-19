#![cfg(test)]
use crate::{
    cards::Card,
    data_model::{Character, Condition, Conditions},
    hex_grid::PosAxial,
    test::tools::{play_card_with_inputs, single_targeted_input},
};

#[test]
pub fn test_item_shroud_of_the_poison_feeder() {
    let not_invisible = &mut Character {
        stamina_current: 10,
        conditions: Conditions::default(),
        ..Default::default()
    };
    let invisible = &mut Character {
        stamina_current: 10,
        conditions: Conditions::default().with_incremented(Condition::Invisible, 1),
        ..Default::default()
    };

    play_card_with_inputs(
        Card::Preparation,
        not_invisible,
        &mut vec![],
        single_targeted_input(PosAxial::new(1, 0)).iter(),
    )
    .unwrap();

    play_card_with_inputs(
        Card::Preparation,
        invisible,
        &mut vec![],
        single_targeted_input(PosAxial::new(1, 0)).iter(),
    )
    .unwrap();

    assert_eq!(not_invisible.conditions.get(&Condition::Empowered), 0);
    assert_eq!(invisible.conditions.get(&Condition::Empowered), 1);
}
