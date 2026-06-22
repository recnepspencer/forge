use crate::runtime::WorthUiExtensionHookAdmission;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiCanvasDrawHook {
    hook_id: String,
    preserved_support_digest: u64,
}

impl WorthUiCanvasDrawHook {
    pub(crate) fn from_admission(admission: &WorthUiExtensionHookAdmission) -> Self {
        Self {
            hook_id: admission.hook().hook_id().to_string(),
            preserved_support_digest: admission.preserved_lane_support().support_contract_digest(),
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn preserved_support_digest(&self) -> u64 {
        self.preserved_support_digest
    }
}
