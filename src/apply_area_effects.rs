#![allow(dead_code)]
use crate::{
    character_filter::filter_character,
    data_model::{
        Ability, AreaEffect, Character, Condition, ConditionEffect, EffectOnCharacter,
        ModifyGainedConditions, TriggeredAbilities,
    },
    hex_grid::{Pos, pos_in_area},
};

pub fn apply_area_effects<'b, C, E>(
    effects: E,
    target: &Pos,
    characters: &mut C,
    source_character: &mut Character,
) where
    for<'c> &'c mut C: IntoIterator<Item = &'c mut Character>,
    E: Iterator<Item = &'b AreaEffect>,
{
    for effect in effects {
        apply_area_effect(
            effect,
            target,
            source_character,
            (&mut *characters).into_iter(),
        );
    }
}

pub fn apply_area_effect<'a, 'b, C>(
    area_effect: &AreaEffect,
    target: &Pos,
    source_character: &mut Character,
    other_characters: C,
) where
    C: Iterator<Item = &'a mut Character>,
{
    if pos_in_area(&source_character.pos, &area_effect.area, target)
        && area_effect
            .filter
            .as_ref()
            .is_none_or(|filter| filter_character(source_character, filter, source_character))
    {
        for effect in &area_effect.effects {
            apply_effect_to_character_with_same_source_character(effect, source_character);
        }
    }
    for character in other_characters
        .into_iter()
        .filter(|character| pos_in_area(&character.pos, &area_effect.area, target))
    {
        if area_effect
            .filter
            .as_ref()
            .is_none_or(|filter| filter_character(character, filter, source_character))
        {
            for effect in &area_effect.effects {
                apply_effect_to_character(effect, character, source_character);
            }
        }
    }
}

pub fn apply_effect_to_character(
    effect: &EffectOnCharacter,
    character: &mut Character,
    source_character: &mut Character,
) {
    match effect {
        EffectOnCharacter::Damage(damage) => {
            let net_damage = net_damage(*damage, character, source_character);
            deal_damage(net_damage, character);
            deal_damage(
                character.conditions.get(&Condition::Retaliate),
                source_character,
            );
        }
        EffectOnCharacter::DamageWithLifesteal(damage) => {
            let net_damage = net_damage(*damage, character, source_character);
            deal_damage(net_damage, character);
            deal_damage(
                character.conditions.get(&Condition::Retaliate),
                source_character,
            );
            restore_health(net_damage, source_character);
        }
        EffectOnCharacter::Heal(health) => {
            restore_health(*health, character);
        }
        EffectOnCharacter::Condition(condition) => {
            apply_condition_effect(condition, character);
        }
        EffectOnCharacter::GainStamina(stamina) => {
            character.stamina_current =
                usize::min(character.stamina_current + stamina, character.stamina_max);
        }
    }
}

pub fn apply_effect_to_character_with_same_source_character(
    effect: &EffectOnCharacter,
    character: &mut Character,
) {
    match effect {
        EffectOnCharacter::Damage(damage) => {
            let net_damage = net_damage(*damage, character, character);
            deal_damage(net_damage, character);
            deal_damage(character.conditions.get(&Condition::Retaliate), character);
        }
        EffectOnCharacter::DamageWithLifesteal(damage) => {
            let net_damage = net_damage(*damage, character, character);
            deal_damage(net_damage, character);
            deal_damage(character.conditions.get(&Condition::Retaliate), character);
            restore_health(net_damage, character);
        }
        EffectOnCharacter::Heal(health) => {
            restore_health(*health, character);
        }
        EffectOnCharacter::Condition(condition) => {
            apply_condition_effect(condition, character);
        }
        EffectOnCharacter::GainStamina(stamina) => {
            character.stamina_current =
                usize::min(character.stamina_current + stamina, character.stamina_max);
        }
    }
}

#[rustfmt::skip]
pub fn net_damage(
    gross_damage: usize,
    character: &Character,
    source_character: &Character,
) -> usize {
    (
        (
            gross_damage
                + source_character.conditions.get(&Condition::Strong)
                - source_character.conditions.get(&Condition::Weak)
        ) as f64
        * if source_character.conditions.has(&Condition::Empowered) { 2.0 } else { 1.0 }
        * if source_character.conditions.has(&Condition::Enfeebled) { 0.5 } else { 1.0 }
    ) as usize
        + character.conditions.get(&Condition::Fragile)
        - character.conditions.get(&Condition::Fortified)
}

pub fn deal_damage(net_damage: usize, character: &mut Character) {
    character.health_current = usize::saturating_sub(character.health_current, net_damage);
    if net_damage > 0 {
        push_triggered_abilities(character, |x| x.damage_taken);
    }
}

pub fn restore_health(health: usize, character: &mut Character) {
    character.health_current = usize::min(character.health_current + health, character.health_max);
}

pub fn push_triggered_abilities<F>(character: &mut Character, f: F)
where
    F: Fn(TriggeredAbilities) -> Vec<Ability>,
{
    for ability in character
        .equipped_items
        .iter()
        .map(move |item| f(item.data().triggered_abilities))
    {
        character.remaining_abilities.extend_from_slice(&ability);
    }
}

pub fn apply_condition_effect(condition_effect: &ConditionEffect, character: &mut Character) {
    let mut condition_effect = condition_effect.clone();
    for modify_gained_conditions in character
        .equipped_items
        .iter()
        .flat_map(|item| item.data().passives.modify_gained_conditions)
    {
        condition_effect =
            apply_modify_gained_conditions(&modify_gained_conditions, condition_effect.clone())
    }
    character
        .conditions
        .increment(condition_effect.condition_type, condition_effect.value);
}

pub fn apply_modify_gained_conditions(
    modify_gained_conditions: &ModifyGainedConditions,
    condition_effect: ConditionEffect,
) -> ConditionEffect {
    if modify_gained_conditions
        .applies_only_to
        .as_ref()
        .is_some_and(|applies_only_to| *applies_only_to != condition_effect.condition_type)
    {
        condition_effect
    } else {
        ConditionEffect {
            condition_type: *modify_gained_conditions
                .transform_into
                .as_ref()
                .unwrap_or(&condition_effect.condition_type),
            value: (condition_effect.value as f32 * modify_gained_conditions.multiplicative_factor)
                as isize
                + modify_gained_conditions.additive_factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    pub fn test_apply_modify_gained_conditions() {
        assert_eq!(
            apply_modify_gained_conditions(
                &ModifyGainedConditions {
                    applies_only_to: Some(Condition::Poison),
                    transform_into: Some(Condition::Regen),
                    additive_factor: 0,
                    multiplicative_factor: 1.0
                },
                ConditionEffect {
                    condition_type: Condition::Poison,
                    value: 3
                }
            ),
            ConditionEffect {
                condition_type: Condition::Regen,
                value: 3
            }
        )
    }

    #[test]
    pub fn test_apply_condition_effect() {
        let mut character = Character {
            ..Default::default()
        };
        apply_condition_effect(
            &ConditionEffect {
                condition_type: Condition::Disarmed,
                value: 1,
            },
            &mut character,
        );
        assert_eq!(character.conditions.get(&Condition::Disarmed), 1);
    }

    #[test]
    pub fn test_apply_area_effect_condition() {
        let mut character = Character {
            ..Default::default()
        };
        apply_area_effect(
            &AreaEffect {
                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                    condition_type: Condition::Disarmed,
                    value: 1,
                })],
                ..Default::default()
            },
            &Pos::default(),
            &mut character,
            iter::empty(),
        );
        assert_eq!(character.conditions.get(&Condition::Disarmed), 1);
    }
}
