#![cfg(test)]
use crate::{
    cards::Card,
    data_model::Character,
    hex_grid::PosAxial,
    test::tools::{play_card_with_inputs, single_targeted_input},
};

#[test]
pub fn test_item_shroud_of_the_poison_feeder() {
    let health_starting = 10;

    let character = &mut Character {
        health_current: health_starting,
        stamina_current: 10,
        ..Default::default()
    };
    let mut characters: Vec<_> = [
        (-2, 0),
        (-2, 1),
        (-2, 2),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (-1, 2),
        (0, -2),
        (0, -1),
        (0, 0),
        (0, 1),
        (0, 2),
        (1, -2),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, -2),
        (2, -1),
        (2, 0),
    ]
    .iter()
    .map(|(r, q)| Character {
        pos: PosAxial::new(*r, *q),
        health_current: health_starting,
        ..Default::default()
    })
    .collect();

    play_card_with_inputs(
        Card::RainOfArrows,
        character,
        &mut characters,
        single_targeted_input(PosAxial::default()).iter(),
    )
    .unwrap();

    assert!(characters[0].health_current == health_starting);
    assert!(characters[1].health_current == health_starting);
    assert!(characters[2].health_current == health_starting);

    assert!(characters[3].health_current == health_starting);
    assert!(characters[4].health_current < health_starting);
    assert!(characters[5].health_current < health_starting);
    assert!(characters[6].health_current == health_starting);

    assert!(characters[7].health_current == health_starting);
    assert!(characters[8].health_current < health_starting);
    assert!(characters[9].health_current < health_starting);
    assert!(characters[10].health_current < health_starting);
    assert!(characters[11].health_current == health_starting);

    assert!(characters[12].health_current == health_starting);
    assert!(characters[13].health_current < health_starting);
    assert!(characters[14].health_current < health_starting);
    assert!(characters[15].health_current == health_starting);

    assert!(characters[16].health_current == health_starting);
    assert!(characters[17].health_current == health_starting);
    assert!(characters[18].health_current == health_starting);
}
