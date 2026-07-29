use std::rc::Rc;

use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::WorthUiRuntimeDiagnosticPolicy;
use crate::source::{WorthUiArtifact, WorthUiArtifactDigest};

use super::{WorthUiHostSessionPlan, WorthUiPreparedApplicationLoweringAuthority};

/// Move-only launch proof derived from one complete prepared application.
/// It is crate-private so component digests cannot be promoted into launch
/// authority by ordinary callers.
pub(crate) struct WorthUiPreparedLaunchAdmission {
    pub(crate) lowering_authority: WorthUiPreparedApplicationLoweringAuthority,
    pub(crate) initial_allocation_commit:
        crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommit,
    pub(crate) artifact: Rc<WorthUiArtifact>,
    pub(crate) artifact_digest: WorthUiArtifactDigest,
    pub(crate) snapshot_digest: CapabilitySnapshotDigest,
    pub(crate) diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    pub(crate) query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    pub(crate) host_session_plan: WorthUiHostSessionPlan,
    pub(crate) change_profile: crate::runtime::rebind::UiChangeProfile,
}
