pub(super) struct WorthUiExecutionPlanHashFold {
    value: u64,
}

impl WorthUiExecutionPlanHashFold {
    pub(super) fn new(seed: u64) -> Self {
        Self { value: seed }
    }

    pub(super) fn fold_tag(&mut self, tag: u64) {
        self.fold_u64(tag ^ 0xa9b1_2c3d_4e5f_6071);
    }

    pub(super) fn fold_len(&mut self, len: usize) {
        self.fold_u64(len as u64);
    }

    pub(super) fn fold_bool(&mut self, value: bool) {
        self.fold_u64(u64::from(value));
    }

    pub(super) fn fold_u64(&mut self, value: u64) {
        self.value ^= value.wrapping_add(0x9e37_79b9_7f4a_7c15);
        self.value = self
            .value
            .rotate_left(27)
            .wrapping_mul(0x94d0_49bb_1331_11eb);
    }

    pub(super) fn finish(self) -> u64 {
        self.value ^ self.value.rotate_right(31)
    }
}
