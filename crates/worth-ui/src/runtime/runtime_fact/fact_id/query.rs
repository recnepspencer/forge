use crate::capability::ViewBindingId;

use super::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

impl WorthUiRuntimeFactId {
    pub fn query_computed_view(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryComputedView,
            view_binding_id.as_str(),
        )
    }

    pub fn query_state_snapshot(snapshot_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryStateSnapshot,
            snapshot_identity,
        )
    }

    pub fn query_effect_posture(effect_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryEffectPosture,
            effect_identity,
        )
    }

    pub fn query_recovery_posture(recovery_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryRecoveryPosture,
            recovery_identity,
        )
    }

    pub fn query_inspection_target(inspection_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryInspectionTarget,
            inspection_identity,
        )
    }
}
