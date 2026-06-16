use crate::sample_source::{VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationSourcePackage {
    module_path: String,
    source_text: String,
    source_digest: u64,
}

impl ValidationSourcePackage {
    pub fn sample() -> Self {
        Self::new(VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE)
    }

    pub fn new(module_path: impl Into<String>, source_text: impl Into<String>) -> Self {
        let module_path = module_path.into();
        let source_text = source_text.into();
        let source_digest = digest_text(&module_path, &source_text);
        Self {
            module_path,
            source_text,
            source_digest,
        }
    }

    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }
}

fn digest_text(module_path: &str, source_text: &str) -> u64 {
    [module_path.as_bytes(), source_text.as_bytes()]
        .into_iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, bytes| {
            fold_bytes(digest, bytes)
        })
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
