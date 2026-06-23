use super::{WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeChangeFamilyRow};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeChangeEvidenceDigest {
    value: u64,
}

impl WorthUiRuntimeChangeEvidenceDigest {
    pub(crate) fn from_rows(
        runtime_instance_witness: u64,
        posture: WorthUiRuntimeChangeActivationPosture,
        rows: &[WorthUiRuntimeChangeFamilyRow],
    ) -> Self {
        let mut hasher = StableRuntimeChangeHasher::new();
        hasher.write_u64(runtime_instance_witness);
        hasher.write_str(posture_token(posture));
        for row in rows {
            hasher.write_str(row.family().token());
            hasher.write_str(row.status().token());
            hasher.write_u64(row.changed_facts().digest().value());
            hasher.write_u64(row.payload_digest());
            hasher.write_optional_str(row.denial_detail());
        }
        Self {
            value: hasher.finish(),
        }
    }

    pub fn value(self) -> u64 {
        self.value
    }
}

struct StableRuntimeChangeHasher {
    value: u64,
}

impl StableRuntimeChangeHasher {
    fn new() -> Self {
        Self {
            value: 0xcbf29ce484222325,
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    fn write_optional_str(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                self.write_u64(1);
                self.write_str(value);
            }
            None => self.write_u64(0),
        }
    }

    fn write_str(&mut self, value: &str) {
        self.write_u64(value.len() as u64);
        self.write_bytes(value.as_bytes());
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(self) -> u64 {
        self.value
    }
}

fn posture_token(posture: WorthUiRuntimeChangeActivationPosture) -> &'static str {
    match posture {
        WorthUiRuntimeChangeActivationPosture::EquivalentNoOp => "equivalent",
        WorthUiRuntimeChangeActivationPosture::ReadyForFrameBoundary => "ready",
        WorthUiRuntimeChangeActivationPosture::Activated => "activated",
        WorthUiRuntimeChangeActivationPosture::Denied => "denied",
        WorthUiRuntimeChangeActivationPosture::Mixed(_) => "mixed",
    }
}
