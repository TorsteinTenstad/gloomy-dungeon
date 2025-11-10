use crate::{
    character_filter::{CharacterFilter, filter_character},
    data_model::{Character, Comparison, RoundStat},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precondition {
    FilteredCount {
        filter: CharacterFilter,
        comparison: Comparison,
        value: usize,
    },
    RoundStat {
        round_index_relative: usize,
        stat: RoundStat,
        comparison: Comparison,
        value: usize,
    },
}

pub fn precondition_is_met<'a, C>(
    precondition: &Precondition,
    characters: C,
    source_character: &Character,
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
                .filter(|character| filter_character(character, filter, source_character))
                .count();
            comparison.compare(&count, value)
        }
        Precondition::RoundStat {
            round_index_relative,
            stat,
            comparison,
            value,
        } => source_character
            .round_stats
            .len()
            .checked_sub(*round_index_relative)
            .and_then(|round_index_absolute| source_character.round_stats.get(round_index_absolute))
            .is_some_and(|round_stats| comparison.compare(&round_stats.get(stat), value)),
    }
}
