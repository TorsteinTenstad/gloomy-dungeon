use crate::enum_map::EnumMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TurnStat {
    SpacesMoved,
    AttackActions,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TurnStats(Vec<EnumMap<TurnStat>>);

impl TurnStats {
    pub fn get_current_mut(&mut self, stat: TurnStat) -> &mut usize {
        if self.0.is_empty() {
            self.0.push(Default::default());
        }
        self.0.last_mut().unwrap().get_mut(stat)
    }

    fn try_get(&self, turn_index_relative: usize, stat: &TurnStat) -> Option<usize> {
        let idx = self.0.len().checked_sub(1 + turn_index_relative)?;
        let turn = self.0.get(idx)?;
        Some(turn.get(stat))
    }

    pub fn get(&self, turn_index_relative: usize, stat: &TurnStat) -> usize {
        self.try_get(turn_index_relative, stat).unwrap_or_default()
    }

    pub fn end_turn(&mut self) {
        self.0.push(Default::default());
    }
}
