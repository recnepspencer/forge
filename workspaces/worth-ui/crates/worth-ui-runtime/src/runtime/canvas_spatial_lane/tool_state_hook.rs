use crate::runtime::WorthUiExtensionHookAdmission;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiSpatialToolStateHook {
    hook_id: String,
    selection_identity_digest: u64,
}

impl WorthUiSpatialToolStateHook {
    pub(crate) fn from_admission(admission: &WorthUiExtensionHookAdmission) -> Self {
        Self {
            hook_id: admission.hook().hook_id().to_string(),
            selection_identity_digest: fold(0xcbf29ce484222325, admission.hook().hook_id()),
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn selection_identity_digest(&self) -> u64 {
        self.selection_identity_digest
    }
}

fn fold(mut digest: u64, text: &str) -> u64 {
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100000001b3);
    }
    digest
}
