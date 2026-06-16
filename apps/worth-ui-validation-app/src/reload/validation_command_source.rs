#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationCommandSource {
    source_text: String,
}

impl ValidationCommandSource {
    pub fn new(source_text: impl Into<String>) -> Self {
        Self {
            source_text: source_text.into(),
        }
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    pub fn source_digest(&self) -> u64 {
        fold_bytes(0xcbf2_9ce4_8422_2325, self.source_text.as_bytes())
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
