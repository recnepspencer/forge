#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DenseSlotBitSet {
    words: Vec<u64>,
}

impl DenseSlotBitSet {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            words: vec![0; capacity.div_ceil(64)],
        }
    }

    fn ensure_capacity(&mut self, slot: usize) {
        let required = slot / 64 + 1;
        if self.words.len() < required {
            self.words.resize(required, 0);
        }
    }

    pub(crate) fn set(&mut self, slot: usize, value: bool) {
        self.ensure_capacity(slot);
        let word = slot / 64;
        let bit = slot % 64;
        if value {
            self.words[word] |= 1 << bit;
        } else {
            self.words[word] &= !(1 << bit);
        }
    }

    pub(crate) fn count_ones(&self) -> usize {
        self.words
            .iter()
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
            let Some(word) = self.words.get(start_word).copied() else {
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

        if let Some(word) = self.words.get(start_word).copied() {
            total += (word & ((!0u64) << start_bit)).count_ones() as usize;
        }

        for word_index in (start_word + 1)..end_word {
            total += self
                .words
                .get(word_index)
                .copied()
                .unwrap_or(0)
                .count_ones() as usize;
        }

        if let Some(word) = self.words.get(end_word).copied() {
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
        for (word_index, word) in self.words.iter().copied().enumerate() {
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
        Self { words }
    }

    pub(crate) fn words(&self) -> &[u64] {
        &self.words
    }
}
