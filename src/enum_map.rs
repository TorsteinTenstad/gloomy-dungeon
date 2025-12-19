use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMap<K: Eq + Hash>(HashMap<K, usize>);

impl<K: Eq + Hash> EnumMap<K> {
    pub fn with_incremented(self, key: K, increment: isize) -> Self {
        let mut this = self;
        this.increment(key, increment);
        this
    }
    pub fn get(&self, key: &K) -> usize {
        *self.0.get(key).unwrap_or(&0)
    }

    pub fn get_mut(&mut self, key: K) -> &mut usize {
        self.0.entry(key).or_default()
    }

    pub fn has(&self, key: &K) -> bool {
        self.get(key) > 0
    }

    pub fn decrement_all(&mut self) {
        for v in self.0.values_mut() {
            *v = v.saturating_sub(1);
        }
    }

    pub fn increment(&mut self, key: K, increment: isize) {
        let v = self.get_mut(key);
        if increment >= 0 {
            *v = v.saturating_add(increment as usize);
        } else {
            *v = v.saturating_sub((-increment) as usize);
        }
    }
}

impl<K: Eq + Hash> Default for EnumMap<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}
