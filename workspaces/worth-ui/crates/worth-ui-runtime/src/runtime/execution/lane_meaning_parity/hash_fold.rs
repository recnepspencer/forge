pub(super) struct WorthUiLaneParityHashFold {
    value: u64,
}

impl WorthUiLaneParityHashFold {
    pub(super) fn new(seed: u64) -> Self {
        Self { value: seed }
    }

    pub(super) fn fold(&mut self, value: u64) {
        self.value ^= value.wrapping_add(0x9e37_79b9_7f4a_7c15);
        self.value = self
            .value
            .rotate_left(21)
            .wrapping_mul(0x94d0_49bb_1331_11eb);
    }

    pub(super) fn fold_str(&mut self, value: &str) {
        self.fold(value.len() as u64);
        for byte in value.as_bytes() {
            self.fold(u64::from(*byte));
        }
    }

    pub(super) fn finish(self) -> u64 {
        self.value ^ self.value.rotate_right(29)
    }
}
