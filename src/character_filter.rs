#![allow(dead_code)]

use crate::{
    data_model::{Character, Condition},
    hex_grid::{DistanceRange, distance, distance_within_range},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterFilter {
    IsEnemy,
    IsSelf,
    WithinDistance(DistanceRange),
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
        CharacterFilter::WithinDistance(distance_range) => {
            distance_within_range(&character.pos, &source_character.pos, distance_range)
        }
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
