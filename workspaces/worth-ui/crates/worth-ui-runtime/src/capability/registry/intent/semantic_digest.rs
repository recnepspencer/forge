pub(crate) struct UiIntentSemanticDigest {
    accumulator: u64,
}

impl UiIntentSemanticDigest {
    pub(crate) const fn new(seed: u64) -> Self {
        Self { accumulator: seed }
    }

    pub(crate) fn field(mut self, name: &'static str, value: &[u8]) -> Self {
        self.fold_framed(name.as_bytes());
        self.fold_framed(value);
        self
    }

    pub(crate) fn u16(self, name: &'static str, value: u16) -> Self {
        self.field(name, &value.to_le_bytes())
    }

    pub(crate) fn usize(self, name: &'static str, value: usize) -> Self {
        let value = u64::try_from(value).expect("intent semantic width exceeds u64");
        self.field(name, &value.to_le_bytes())
    }

    pub(crate) const fn finish(self) -> u64 {
        self.accumulator
    }

    fn fold_framed(&mut self, bytes: &[u8]) {
        self.fold_bytes(
            &u64::try_from(bytes.len())
                .expect("intent semantic field exceeds u64")
                .to_le_bytes(),
        );
        self.fold_bytes(bytes);
    }

    fn fold_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.accumulator ^= u64::from(*byte);
            self.accumulator = self.accumulator.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}
