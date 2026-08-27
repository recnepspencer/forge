use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DenseSlotBitSet {
    words: BTreeMap<usize, u64>,
}

impl DenseSlotBitSet {
    pub(crate) fn with_capacity(_capacity: usize) -> Self {
        Self {
            words: BTreeMap::new(),
        }
    }

    pub(crate) fn set(&mut self, slot: usize, value: bool) {
        let word = slot / 64;
        let bit = slot % 64;
        if value {
            *self.words.entry(word).or_default() |= 1 << bit;
        } else {
            let remove = self.words.get_mut(&word).is_some_and(|value| {
                *value &= !(1 << bit);
                *value == 0
            });
            if remove {
                self.words.remove(&word);
            }
        }
    }

    pub(crate) fn count_ones(&self) -> usize {
        self.words
            .values()
            .map(|word| word.count_ones() as usize)
            .sum()
    }

    pub(crate) fn count_ones_in_range(&self, start: usize, end: usize) -> usize {
        if start >= end {
            return 0;
        }
        let start_word = start / 64;
        let end_word = (end - 1) / 64;
        let start_bit = start % 64;
        let end_bit = (end - 1) % 64;
        let mut total = 0usize;

        if start_word == end_word {
            let Some(word) = self.words.get(&start_word).copied() else {
                return 0;
            };
            let lower_mask = (!0u64) << start_bit;
            let upper_mask = if end_bit == 63 {
                !0u64
            } else {
                (1u64 << (end_bit + 1)) - 1
            };
            return (word & lower_mask & upper_mask).count_ones() as usize;
        }

        if let Some(word) = self.words.get(&start_word).copied() {
            total += (word & ((!0u64) << start_bit)).count_ones() as usize;
        }

        for word_index in (start_word + 1)..end_word {
            total += self
                .words
                .get(&word_index)
                .copied()
                .unwrap_or(0)
                .count_ones() as usize;
        }

        if let Some(word) = self.words.get(&end_word).copied() {
            let upper_mask = if end_bit == 63 {
                !0u64
            } else {
                (1u64 << (end_bit + 1)) - 1
            };
            total += (word & upper_mask).count_ones() as usize;
        }

        total
    }

    pub(crate) fn iter_set_slots(&self) -> Vec<usize> {
        let mut slots = Vec::new();
        for (&word_index, &word) in &self.words {
            let mut remaining = word;
            while remaining != 0 {
                let bit = remaining.trailing_zeros() as usize;
                slots.push(word_index * 64 + bit);
                remaining &= remaining - 1;
            }
        }
        slots
    }

    pub(crate) fn from_words(words: Vec<u64>) -> Self {
        Self {
            words: words
                .into_iter()
                .enumerate()
                .filter(|(_, word)| *word != 0)
                .collect(),
        }
    }

    pub(crate) fn from_sparse_words(words: Vec<(u64, u64)>) -> Self {
        Self {
            words: words
                .into_iter()
                .filter(|(_, word)| *word != 0)
                .map(|(index, word)| (index as usize, word))
                .collect(),
        }
    }

    pub(crate) fn sparse_words(&self) -> Vec<(u64, u64)> {
        self.words
            .iter()
            .map(|(&index, &word)| (index as u64, word))
            .collect()
    }

    pub(crate) fn represented_slot_capacity(&self) -> usize {
        self.words
            .last_key_value()
            .map_or(0, |(&word, _)| (word + 1).saturating_mul(64))
    }

    pub(crate) fn authoritative_allocation_bytes(&self) -> u64 {
        (self.words.len() as u64)
            .saturating_mul((std::mem::size_of::<usize>() + std::mem::size_of::<u64>()) as u64)
    }
}
