use super::WorthUiRuntimeFactId;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiRuntimeFactSetDigest {
    value: u64,
}

impl WorthUiRuntimeFactSetDigest {
    pub(crate) fn from_facts<'a>(
        facts: impl IntoIterator<Item = &'a WorthUiRuntimeFactId>,
    ) -> Self {
        let mut hasher = StableFactSetHasher::new();
        for fact in facts {
            hasher.write_str(fact.family().token());
            hasher.write_str(fact.identity());
        }
        Self {
            value: hasher.finish(),
        }
    }

    pub fn value(self) -> u64 {
        self.value
    }
}

struct StableFactSetHasher {
    value: u64,
}

impl StableFactSetHasher {
    fn new() -> Self {
        Self {
            value: 0xcbf29ce484222325,
        }
    }

    fn write_str(&mut self, value: &str) {
        self.write_usize(value.len());
        for byte in value.as_bytes() {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn write_usize(&mut self, value: usize) {
        for byte in value.to_le_bytes() {
            self.value ^= u64::from(byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(self) -> u64 {
        self.value
    }
}
