mod basis_authority_binding;
mod constructors;

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::query_context::QueryContextExecutionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionSourceFamily {
    QueryReadReceipt,
    QueryLiveReadReceipt,
    QueryWriteReceipt,
    QueryContextExecution,
    RelationalRowSet,
    RelationalGroupedProjection,
    BridgeTruthViewRowSet,
    BridgeGroupedTruthView,
    RetainedDerivedArtifactBinding,
    LiveArtifactBinding,
}

impl ProjectionSourceFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryReadReceipt => "query_read_receipt",
            Self::QueryLiveReadReceipt => "query_live_read_receipt",
            Self::QueryWriteReceipt => "query_write_receipt",
            Self::QueryContextExecution => "query_context_execution",
            Self::RelationalRowSet => "relational_row_set",
            Self::RelationalGroupedProjection => "relational_grouped_projection",
            Self::BridgeTruthViewRowSet => "bridge_truth_view_row_set",
            Self::BridgeGroupedTruthView => "bridge_grouped_truth_view",
            Self::RetainedDerivedArtifactBinding => "retained_derived_artifact_binding",
            Self::LiveArtifactBinding => "live_artifact_binding",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionSourceExecutionPosture {
    Current,
    Branch,
    Historical,
    PreviewDerived,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ProjectionWriteReceiptCapabilities {
    has_target_identity: bool,
    has_source_reference: bool,
    has_effect_continuity: bool,
    has_relation_endpoint: bool,
}

impl ProjectionWriteReceiptCapabilities {
    pub(crate) fn has_target_identity(&self) -> bool {
        self.has_target_identity
    }

    pub(crate) fn has_source_reference(&self) -> bool {
        self.has_source_reference
    }

    pub(crate) fn has_effect_continuity(&self) -> bool {
        self.has_effect_continuity
    }

    pub(crate) fn has_relation_endpoint(&self) -> bool {
        self.has_relation_endpoint
    }

    pub(crate) fn synthetic(
        has_target_identity: bool,
        has_source_reference: bool,
        has_effect_continuity: bool,
        has_relation_endpoint: bool,
    ) -> Self {
        Self {
            has_target_identity,
            has_source_reference,
            has_effect_continuity,
            has_relation_endpoint,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        has_target_identity: bool,
        has_source_reference: bool,
        has_effect_continuity: bool,
        has_relation_endpoint: bool,
    ) -> Self {
        Self::synthetic(
            has_target_identity,
            has_source_reference,
            has_effect_continuity,
            has_relation_endpoint,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionSourceCapabilityProfile {
    QueryReadReceipt {
        execution_posture: ProjectionSourceExecutionPosture,
    },
    QueryLiveReadReceipt {
        execution_posture: ProjectionSourceExecutionPosture,
    },
    QueryWriteReceipt {
        capabilities: ProjectionWriteReceiptCapabilities,
    },
    QueryContextExecution {
        execution_posture: ProjectionSourceExecutionPosture,
    },
    RelationalRowSet,
    RelationalGroupedProjection,
    BridgeTruthViewRowSet,
    BridgeGroupedTruthView,
    RetainedDerivedArtifactBinding,
    LiveArtifactBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceIdentity {
    identity: ProjectionSourceIdentityKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ProjectionSourceIdentityKind {
    Evidence(WorthQueryEvidenceIdentity),
    Artifact(String),
}

impl ProjectionSourceIdentity {
    pub fn from_evidence_identity(identity: WorthQueryEvidenceIdentity) -> Self {
        Self {
            identity: ProjectionSourceIdentityKind::Evidence(identity),
        }
    }

    pub fn artifact(identity: impl Into<String>) -> Self {
        Self {
            identity: ProjectionSourceIdentityKind::Artifact(identity.into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match &self.identity {
            ProjectionSourceIdentityKind::Evidence(identity) => identity.as_str(),
            ProjectionSourceIdentityKind::Artifact(identity) => identity,
        }
    }

    pub fn evidence_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        match &self.identity {
            ProjectionSourceIdentityKind::Evidence(identity) => Some(identity),
            ProjectionSourceIdentityKind::Artifact(_) => None,
        }
    }
}

impl From<&str> for ProjectionSourceIdentity {
    fn from(value: &str) -> Self {
        Self::artifact(value)
    }
}

impl From<String> for ProjectionSourceIdentity {
    fn from(value: String) -> Self {
        Self::artifact(value)
    }
}

impl std::fmt::Display for ProjectionSourceIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceBasisAuthority {
    kind: ProjectionSourceBasisAuthorityKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ProjectionSourceBasisAuthorityKind {
    RuntimeSnapshot(WorthQuerySnapshotIdentity),
    QueryContext {
        family: QueryContextExecutionFamily,
        basis_digest: String,
    },
    Certification(WorthQueryEvidenceIdentity),
}

impl ProjectionSourceBasisAuthority {
    pub fn snapshot_identity(&self) -> Option<&WorthQuerySnapshotIdentity> {
        match &self.kind {
            ProjectionSourceBasisAuthorityKind::RuntimeSnapshot(identity) => Some(identity),
            ProjectionSourceBasisAuthorityKind::QueryContext { .. }
            | ProjectionSourceBasisAuthorityKind::Certification(_) => None,
        }
    }

    pub fn query_context_family(&self) -> Option<&QueryContextExecutionFamily> {
        match &self.kind {
            ProjectionSourceBasisAuthorityKind::QueryContext { family, .. } => Some(family),
            ProjectionSourceBasisAuthorityKind::RuntimeSnapshot(_)
            | ProjectionSourceBasisAuthorityKind::Certification(_) => None,
        }
    }

    pub fn terminal_projection_for_reporting(&self) -> String {
        match &self.kind {
            ProjectionSourceBasisAuthorityKind::RuntimeSnapshot(identity) => {
                identity.evidence_identity().as_str().to_string()
            }
            ProjectionSourceBasisAuthorityKind::QueryContext { basis_digest, .. } => {
                basis_digest.clone()
            }
            ProjectionSourceBasisAuthorityKind::Certification(identity) => {
                identity.as_str().to_string()
            }
        }
    }

    pub(crate) fn runtime_snapshot(identity: WorthQuerySnapshotIdentity) -> Self {
        Self {
            kind: ProjectionSourceBasisAuthorityKind::RuntimeSnapshot(identity),
        }
    }

    pub(crate) fn query_context(
        family: QueryContextExecutionFamily,
        basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            kind: ProjectionSourceBasisAuthorityKind::QueryContext {
                family,
                basis_digest: basis_digest.into(),
            },
        }
    }

    pub(crate) fn certification(identity: WorthQueryEvidenceIdentity) -> Self {
        Self {
            kind: ProjectionSourceBasisAuthorityKind::Certification(identity),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceReferenceIdentity {
    label: &'static str,
    identity: String,
}

impl ProjectionSourceReferenceIdentity {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn synthetic(label: &'static str, identity: impl Into<String>) -> Self {
        Self {
            label,
            identity: identity.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(label: &'static str, identity: impl Into<String>) -> Self {
        Self::synthetic(label, identity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSource {
    family: ProjectionSourceFamily,
    capability_profile: ProjectionSourceCapabilityProfile,
    query_digest: Option<String>,
    basis_digest: Option<String>,
    basis_authority: ProjectionSourceBasisAuthority,
    result_digest: Option<String>,
    result_shape_digest: Option<String>,
    source_identity: ProjectionSourceIdentity,
    source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
}

impl ProjectionConsumptionSource {
    pub fn family(&self) -> ProjectionSourceFamily {
        self.family
    }

    pub(crate) fn capability_profile(&self) -> &ProjectionSourceCapabilityProfile {
        &self.capability_profile
    }

    pub fn query_digest(&self) -> Option<&str> {
        self.query_digest.as_deref()
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn basis_authority(&self) -> &ProjectionSourceBasisAuthority {
        &self.basis_authority
    }

    pub fn result_digest(&self) -> Option<&str> {
        self.result_digest.as_deref()
    }

    pub fn result_shape_digest(&self) -> Option<&str> {
        self.result_shape_digest.as_deref()
    }

    pub fn source_identity(&self) -> &str {
        self.source_identity.as_str()
    }

    pub fn source_identity_handle(&self) -> &ProjectionSourceIdentity {
        &self.source_identity
    }

    pub fn source_reference_identities(&self) -> &[ProjectionSourceReferenceIdentity] {
        &self.source_reference_identities
    }

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub(crate) fn synthetic_for_certification(
        family: ProjectionSourceFamily,
        capability_profile: ProjectionSourceCapabilityProfile,
        source_identity: impl Into<ProjectionSourceIdentity>,
        source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    ) -> Self {
        Self {
            family,
            capability_profile,
            query_digest: None,
            basis_digest: None,
            basis_authority: ProjectionSourceBasisAuthority::certification(
                certification_basis_identity("projection-source-certification-basis"),
            ),
            result_digest: None,
            result_shape_digest: None,
            source_identity: source_identity.into(),
            source_reference_identities,
            materialized_fact_posture: None,
        }
    }

    pub(crate) fn intent_admission_certification(
        family: ProjectionSourceFamily,
        capability_profile: ProjectionSourceCapabilityProfile,
        query_digest: Option<String>,
        basis_digest: Option<String>,
        result_digest: Option<String>,
        result_shape_digest: Option<String>,
        source_identity: impl Into<ProjectionSourceIdentity>,
        source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    ) -> Self {
        Self {
            family,
            capability_profile,
            query_digest,
            basis_digest,
            basis_authority: ProjectionSourceBasisAuthority::certification(
                certification_basis_identity("projection-source-intent-certification-basis"),
            ),
            result_digest,
            result_shape_digest,
            source_identity: source_identity.into(),
            source_reference_identities,
            materialized_fact_posture: None,
        }
    }
}

fn certification_basis_identity(label: &'static str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::ProjectionConsumptionIdentity)
        .field_shape(WorthQueryEvidenceTag::new("certification_basis"), label)
        .seal()
}

#[cfg(test)]
#[path = "../tests/source_test_support.rs"]
#[cfg(test)]
mod test_support;
