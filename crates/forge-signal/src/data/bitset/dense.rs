/// Dense bitset for deterministic, allocation-light index marking.
#[derive(Debug, Clone, Default)]
pub struct DenseBitset {
    words: Vec<u64>,
}

impl DenseBitset {
    /// Create an empty bitset.
    pub fn new() -> Self {
        Self { words: Vec::new() }
    }

    /// Ensure capacity for indices up to `len`.
    pub fn ensure_len(&mut self, len: usize) {
        let word_len = len.div_ceil(64);
        if self.words.len() < word_len {
            self.words.resize(word_len, 0);
        }
    }

    /// Mark one index. Returns true if this call changed the bit.
    pub fn mark(&mut self, idx: usize) -> bool {
        let word_idx = idx / 64;
        if word_idx >= self.words.len() {
            self.words.resize(word_idx + 1, 0);
        }
        let bit = 1u64 << (idx % 64);
        let before = self.words[word_idx];
        self.words[word_idx] |= bit;
        before != self.words[word_idx]
    }

    /// Clear one index.
    pub fn clear(&mut self, idx: usize) {
        let word_idx = idx / 64;
        if word_idx >= self.words.len() {
            return;
        }
        let bit = 1u64 << (idx % 64);
        self.words[word_idx] &= !bit;
    }

    /// Return whether index is marked.
    pub fn contains(&self, idx: usize) -> bool {
        let word_idx = idx / 64;
        if word_idx >= self.words.len() {
            return false;
        }
        let bit = 1u64 << (idx % 64);
        (self.words[word_idx] & bit) != 0
    }

    /// Clear all bits.
    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// Merge `other` into this bitset.
    pub fn merge(&mut self, other: &Self) {
        if self.words.len() < other.words.len() {
            self.words.resize(other.words.len(), 0);
        }
        for (idx, word) in other.words.iter().copied().enumerate() {
            self.words[idx] |= word;
        }
    }

    /// Whether this bitset has any marked bits.
    pub fn any(&self) -> bool {
        self.words.iter().any(|w| *w != 0)
    }

    /// Return marked indices in ascending order.
    pub fn marked_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for (word_idx, word) in self.words.iter().copied().enumerate() {
            if word == 0 {
                continue;
            }
            let mut pending = word;
            while pending != 0 {
                let bit = pending.trailing_zeros() as usize;
                out.push(word_idx * 64 + bit);
                pending &= pending - 1;
            }
        }
        out
    }
}
