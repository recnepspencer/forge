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
    visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
}

pub(super) struct WorthUiPreparedGenerationIdentityInput<'plan> {
    pub(super) capability_snapshot: CapabilitySnapshotDigest,
    pub(super) canonical_artifact: WorthUiPreparedApplicationArtifactIdentity,
    pub(super) declaration_source: WorthUiPreparedDeclarationSourceIdentity,
    pub(super) semantic_package: worth_ui_dsl::WorthUiSemanticPackageIdentity,
    pub(super) graph_authority_digest: u64,
    pub(super) query_binding_plan: &'plan worth_ui_query_binding::WorthUiQueryBindingPlan,
    pub(super) host_session_plan: &'plan WorthUiHostSessionPlan,
    pub(super) visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
}

impl WorthUiPreparedApplicationGenerationIdentity {
    pub(super) fn derive(input: WorthUiPreparedGenerationIdentityInput<'_>) -> Self {
        Self {
            inner: Rc::new(WorthUiPreparedApplicationGenerationIdentityInner {
                capability_snapshot: input.capability_snapshot,
                canonical_artifact: input.canonical_artifact,
                declaration_source: input.declaration_source,
                semantic_package: input.semantic_package,
                graph_authority_digest: input.graph_authority_digest,
                query_binding: WorthUiPreparedQueryBindingPlanIdentity::derive(
                    input.query_binding_plan,
                ),
                host_session_plan: input.host_session_plan.clone(),
                visual_inspection_policy: input.visual_inspection_policy,
            }),
        }
    }

    pub fn semantic_package_identity(&self) -> &worth_ui_dsl::WorthUiSemanticPackageIdentity {
        &self.inner.semantic_package
    }
}
