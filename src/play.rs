use crate::{
    apply_area_effects::{deal_damage, push_triggered_abilities, restore_health},
    data_model::{CardData, Character, Condition},
};

pub fn play_card_unchecked(character: &mut Character, card_data: CardData) -> usize {
    debug_assert!(character.stamina_current >= card_data.stamina_cost);
    character.stamina_current =
        usize::saturating_sub(character.stamina_current, card_data.stamina_cost);
    let mut abilities = card_data.abilities;
    let number_of_abilities_gained = abilities.len();
    character.remaining_abilities.append(&mut abilities);
    number_of_abilities_gained
}

pub fn end_turn(character: &mut Character) {
    deal_damage(character.conditions.get(&Condition::Poison), character);
    restore_health(character.conditions.get(&Condition::Regen), character);
    character.turn_stats.end_turn();
    push_triggered_abilities(character, |x| x.end_of_turn);
}

pub fn begin_turn(character: &mut Character) {
    character.conditions.decrement_all();
    push_triggered_abilities(character, |x| x.beginning_of_turn);
}
