pub(super) struct WorthUiActivationGateDigestFold {
    value: u64,
}

impl WorthUiActivationGateDigestFold {
    pub(super) fn new(seed: u64) -> Self {
        Self { value: seed }
    }

    pub(super) fn fold_u64(&mut self, value: u64) {
        self.value ^= value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        self.value = self.value.rotate_left(13);
    }

    pub(super) fn fold_usize(&mut self, value: usize) {
        self.fold_u64(value as u64);
    }

    pub(super) fn fold_tag(&mut self, tag: u64) {
        self.fold_u64(tag);
    }

    pub(super) fn fold_text(&mut self, text: &str) {
        self.fold_usize(text.len());
        for byte in text.as_bytes() {
            self.value ^= u64::from(*byte);
            self.value = self.value.rotate_left(5);
            self.value = self.value.wrapping_mul(0x100_0000_01b3);
        }
    }

    pub(super) fn finish(self) -> u64 {
        self.value ^ 0xa47f_2b19_63d5_81ceu64
    }
}
