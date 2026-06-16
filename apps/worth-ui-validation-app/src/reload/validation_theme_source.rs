#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationThemeSource {
    source_text: String,
    source_digest: u64,
}

impl ValidationThemeSource {
    pub fn new(source_text: impl Into<String>) -> Self {
        let source_text = source_text.into();
        let source_digest = digest_bytes(source_text.as_bytes());
        Self {
            source_text,
            source_digest,
        }
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }
}

fn digest_bytes(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |mut digest, byte| {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
            digest
        })
}
