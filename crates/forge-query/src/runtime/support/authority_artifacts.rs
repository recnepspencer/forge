use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::ForgeQueryAuthorityLane;
use forge_runtime_bridge::facade::BridgeIdentityEvidence;
use std::cmp::Ordering;

#[path = "authority_artifacts/basis_admission.rs"]
mod basis_admission;

#[path = "authority_artifacts/bridge_imports.rs"]
mod bridge_imports;

pub use basis_admission::{
    ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryBranchBasisAdmission,
    ForgeQueryPreviewBasisAdmission,
};

#[derive(Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeEvidenceAuthority {
    _private: (),
}

impl ForgeQueryRuntimeEvidenceAuthority {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationAuthorityIdentity {
    label: String,
    identity: ForgeQueryEvidenceIdentity,
}

macro_rules! mutation_authority_label_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            label: String,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(label: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
                Ok(Self {
                    label: normalize_non_empty_authority_label(label.into())?,
                })
            }

            fn as_str(&self) -> &str {
                &self.label
            }
        }
    };
}

mutation_authority_label_type!(ForgeQueryExistingTruthBindingAuthorityLabel);
mutation_authority_label_type!(ForgeQueryNamingAttachmentAuthorityLabel);
mutation_authority_label_type!(ForgeQueryNamingPriorAuthorityLabel);
mutation_authority_label_type!(ForgeQueryNamingTargetAuthorityLabel);
mutation_authority_label_type!(ForgeQueryContinuityPriorAuthorityLabel);
mutation_authority_label_type!(ForgeQueryContinuitySuccessorAuthorityLabel);

impl ForgeQueryMutationAuthorityIdentity {
    pub(crate) fn new(role: &'static str, label: impl Into<String>) -> Self {
        let label = label.into();
        let identity = mutation_label_identity(
            ForgeQueryEvidenceScope::MutationEvidenceAuthorityIdentity,
            role,
            &label,
        );
        Self { label, identity }
    }

    pub fn existing_truth_binding_authority(
        label: ForgeQueryExistingTruthBindingAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new(
            "existing-truth-binding-authority",
            label.as_str(),
        ))
    }

    pub fn naming_attachment(
        label: ForgeQueryNamingAttachmentAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new("naming-attachment", label.as_str()))
    }

    pub fn naming_prior_authority(
        label: ForgeQueryNamingPriorAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new("naming-prior", label.as_str()))
    }

    pub fn naming_target_authority(
        label: ForgeQueryNamingTargetAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new("naming-target", label.as_str()))
    }

    pub fn continuity_prior_authority(
        label: ForgeQueryContinuityPriorAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new("continuity-prior", label.as_str()))
    }

    pub fn continuity_successor_authority(
        label: ForgeQueryContinuitySuccessorAuthorityLabel,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::new("continuity-successor", label.as_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.label
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }

    pub fn bridge_admission_evidence(&self) -> BridgeIdentityEvidence {
        self.identity.bridge_evidence_identity()
    }

    pub fn terminal_projection_for_reporting(&self) -> &str {
        self.identity.reporting_projection()
    }
}

fn normalize_non_empty_authority_label(value: String) -> Result<String, ForgeQueryWorkspaceError> {
    if value.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(
            "mutation authority identity label may not be empty",
        ));
    }
    Ok(value)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationTargetCollectionIdentity {
    label: String,
    identity: ForgeQueryEvidenceIdentity,
}

impl Ord for ForgeQueryMutationTargetCollectionIdentity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label
            .cmp(&other.label)
            .then_with(|| self.identity.as_str().cmp(other.identity.as_str()))
    }
}

impl PartialOrd for ForgeQueryMutationTargetCollectionIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ForgeQueryMutationTargetCollectionIdentity {
    pub(crate) fn new(role: &'static str, label: impl Into<String>) -> Self {
        let label = label.into();
        let identity = mutation_label_identity(
            ForgeQueryEvidenceScope::MutationEvidenceTargetCollectionIdentity,
            role,
            &label,
        );
        Self { label, identity }
    }

    pub fn as_str(&self) -> &str {
        &self.label
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }

    pub fn same_target_collection_as(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationSymbolIdentity {
    label: String,
    identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryMutationSymbolIdentity {
    pub(crate) fn new(role: &'static str, label: impl Into<String>) -> Self {
        let label = label.into();
        let identity = mutation_label_identity(
            ForgeQueryEvidenceScope::MutationEvidenceSymbolIdentity,
            role,
            &label,
        );
        Self { label, identity }
    }

    pub fn as_str(&self) -> &str {
        &self.label
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }
}

impl Ord for ForgeQueryMutationSymbolIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for ForgeQueryMutationSymbolIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationEvidenceDigest {
    digest: String,
    identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryMutationEvidenceDigest {
    pub(crate) fn source_identity(role: &'static str, digest: &ForgeQueryEvidenceIdentity) -> Self {
        Self {
            digest: digest.as_str().to_string(),
            identity: forge_query_evidence_identity(
                ForgeQueryEvidenceScope::MutationEvidenceSourceDigest,
            )
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_evidence_identity(ForgeQueryEvidenceTag::new("digest"), digest)
            .seal(),
        }
    }

    pub(crate) fn aggregate(role: &'static str, digest: ForgeQueryEvidenceIdentity) -> Self {
        Self {
            digest: digest.as_str().to_string(),
            identity: mutation_digest_identity(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                role,
                &digest,
            ),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.digest
    }

    pub fn is_empty(&self) -> bool {
        self.digest.is_empty()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.digest.starts_with(prefix)
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity
    }
}

fn mutation_label_identity(
    scope: ForgeQueryEvidenceScope,
    role: &'static str,
    label: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(scope)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value(ForgeQueryEvidenceTag::new("label"), label)
        .seal()
}

fn mutation_digest_identity(
    scope: ForgeQueryEvidenceScope,
    role: &'static str,
    digest: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(scope)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("digest"), digest)
        .seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeInspectionEvidence {
    artifact_family: String,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
}

impl ForgeQueryRuntimeInspectionEvidence {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        artifact_family: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            artifact_family: artifact_family.into(),
            authority_lane,
            evidence: evidence.into_iter().map(Into::into).collect(),
        }
    }

    pub fn artifact_family(&self) -> &str {
        &self.artifact_family
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }
}
