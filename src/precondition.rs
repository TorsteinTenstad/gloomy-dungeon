use std::iter;

use crate::{
    character_filter::{CharacterFilter, filter_character},
    data_model::{Character, Comparison},
    turn_stats::TurnStat,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precondition {
    FilteredCount {
        filter: CharacterFilter,
        comparison: Comparison,
        value: usize,
    },
    TurnStat {
        turn_index_relative: usize,
        stat: TurnStat,
        comparison: Comparison,
        value: usize,
    },
}

pub fn optional_precondition_is_met<'a, C>(
    precondition: Option<&Precondition>,
    characters: C,
    source_character: &'a Character,
) -> bool
where
    C: IntoIterator<Item = &'a Character>,
{
    precondition
        .is_none_or(|precondition| precondition_is_met(precondition, characters, source_character))
}
pub fn precondition_is_met<'a, C>(
    precondition: &Precondition,
    characters: C,
    source_character: &'a Character,
) -> bool
where
    C: IntoIterator<Item = &'a Character>,
{
    match precondition {
        Precondition::FilteredCount {
            filter,
            comparison,
            value,
        } => {
            let count = characters
                .into_iter()
                .chain(iter::once(source_character))
                .filter(|character| filter_character(character, filter, source_character))
                .count();
            comparison.compare(&count, value)
        }
        Precondition::TurnStat {
            turn_index_relative,
            stat,
            comparison,
            value,
        } => comparison.compare(
            &source_character.turn_stats.get(*turn_index_relative, stat),
            value,
        ),
    }
}
