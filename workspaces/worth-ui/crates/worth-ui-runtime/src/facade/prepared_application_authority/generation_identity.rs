use super::application_artifact::WorthUiPreparedApplicationArtifactIdentity;
use super::query_binding_plan_identity::WorthUiPreparedQueryBindingPlanIdentity;
use super::{WorthUiHostSessionPlan, WorthUiPreparedDeclarationSourceIdentity};
use crate::capability::CapabilitySnapshotDigest;

/// Comparison-safe identity of exactly one prepared application generation.
/// No public constructor accepts component digests or promotes candidate truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPreparedApplicationGenerationIdentity {
    capability_snapshot: CapabilitySnapshotDigest,
    canonical_artifact: WorthUiPreparedApplicationArtifactIdentity,
    declaration_source: WorthUiPreparedDeclarationSourceIdentity,
    graph_authority_digest: u64,
    query_binding: WorthUiPreparedQueryBindingPlanIdentity,
    host_session_plan: WorthUiHostSessionPlan,
}

impl WorthUiPreparedApplicationGenerationIdentity {
    pub(super) fn derive(
        capability_snapshot: CapabilitySnapshotDigest,
        canonical_artifact: WorthUiPreparedApplicationArtifactIdentity,
        declaration_source: WorthUiPreparedDeclarationSourceIdentity,
        graph_authority_digest: u64,
        query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        host_session_plan: &WorthUiHostSessionPlan,
    ) -> Self {
        Self {
            capability_snapshot,
            canonical_artifact,
            declaration_source,
            graph_authority_digest,
            query_binding: WorthUiPreparedQueryBindingPlanIdentity::derive(query_binding_plan),
            host_session_plan: host_session_plan.clone(),
        }
    }
}
