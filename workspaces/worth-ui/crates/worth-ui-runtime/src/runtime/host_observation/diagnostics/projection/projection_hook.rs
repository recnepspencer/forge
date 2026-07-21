#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticProjectionHook {
    hook_id: String,
    projection_digest: u64,
}

impl WorthUiDiagnosticProjectionHook {
    pub fn projection(hook_id: impl Into<String>) -> Self {
        let hook_id = hook_id.into();
        let projection_digest = stable_text_digest(&hook_id);
        Self {
            hook_id,
            projection_digest,
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn projection_digest(&self) -> u64 {
        self.projection_digest
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
