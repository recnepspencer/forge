use super::application_artifact::WorthUiPreparedApplicationArtifactIdentity;
use super::query_binding_plan_identity::WorthUiPreparedQueryBindingPlanIdentity;
use super::{WorthUiHostSessionPlan, WorthUiPreparedDeclarationSourceIdentity};
use crate::capability::CapabilitySnapshotDigest;
use std::rc::Rc;

/// Comparison-safe identity of exactly one prepared application generation.
/// No public constructor accepts component digests or promotes candidate truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPreparedApplicationGenerationIdentity {
    inner: Rc<WorthUiPreparedApplicationGenerationIdentityInner>,
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiPreparedApplicationGenerationIdentityInner {
    capability_snapshot: CapabilitySnapshotDigest,
    canonical_artifact: WorthUiPreparedApplicationArtifactIdentity,
    declaration_source: WorthUiPreparedDeclarationSourceIdentity,
    semantic_package: worth_ui_dsl::WorthUiSemanticPackageIdentity,
    graph_authority_digest: u64,
    query_binding: WorthUiPreparedQueryBindingPlanIdentity,
    host_session_plan: WorthUiHostSessionPlan,
}

impl WorthUiPreparedApplicationGenerationIdentity {
    pub(super) fn derive(
        capability_snapshot: CapabilitySnapshotDigest,
        canonical_artifact: WorthUiPreparedApplicationArtifactIdentity,
        declaration_source: WorthUiPreparedDeclarationSourceIdentity,
        semantic_package: worth_ui_dsl::WorthUiSemanticPackageIdentity,
        graph_authority_digest: u64,
        query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        host_session_plan: &WorthUiHostSessionPlan,
    ) -> Self {
        Self {
            inner: Rc::new(WorthUiPreparedApplicationGenerationIdentityInner {
                capability_snapshot,
                canonical_artifact,
                declaration_source,
                semantic_package,
                graph_authority_digest,
                query_binding: WorthUiPreparedQueryBindingPlanIdentity::derive(query_binding_plan),
                host_session_plan: host_session_plan.clone(),
            }),
        }
    }

    pub fn semantic_package_identity(&self) -> &worth_ui_dsl::WorthUiSemanticPackageIdentity {
        &self.inner.semantic_package
    }
}
