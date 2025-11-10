#![allow(dead_code)]

use crate::data_model::{Character, Condition};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterFilter {
    IsEnemy,
    IsSelf,
    NoAdjacentEnemies,
    WithCondition(Condition),
    WithoutCondition(Condition),
    And(Vec<CharacterFilter>),
    Or(Vec<CharacterFilter>),
}

pub fn filter_character(
    character: &Character,
    filter: &CharacterFilter,
    source_character: &Character,
) -> bool {
    match &filter {
        CharacterFilter::IsEnemy => character.team != source_character.team,
        CharacterFilter::IsSelf => character == source_character,
        CharacterFilter::NoAdjacentEnemies => todo!(),
        CharacterFilter::WithCondition(condition) => character.conditions.has(condition),
        CharacterFilter::WithoutCondition(condition) => character.conditions.has(condition),
        CharacterFilter::And(sub_filters) => sub_filters
            .iter()
            .all(|filter| filter_character(character, filter, source_character)),
        &CharacterFilter::Or(sub_filters) => sub_filters
            .iter()
            .any(|filter| filter_character(character, filter, source_character)),
    }
}
