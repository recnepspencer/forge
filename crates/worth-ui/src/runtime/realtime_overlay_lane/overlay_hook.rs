use crate::runtime::{WorthUiExtensionHookAdmission, WorthUiLaneAdapterHookKind};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRealtimeOverlayHook {
    hook_id: String,
    support_digest: u64,
}

impl WorthUiRealtimeOverlayHook {
    pub(crate) fn from_admission(admission: &WorthUiExtensionHookAdmission) -> Self {
        debug_assert_eq!(
            admission.hook().kind(),
            WorthUiLaneAdapterHookKind::RealtimeOverlayMechanics
        );
        Self {
            hook_id: admission.hook().hook_id().to_owned(),
            support_digest: admission.preserved_lane_support().support_contract_digest(),
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }

    pub(crate) fn canonical_digest(&self) -> u64 {
        self.hook_id
            .as_bytes()
            .iter()
            .fold(self.support_digest, |digest, byte| {
                fold(digest, u64::from(*byte))
            })
    }
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
