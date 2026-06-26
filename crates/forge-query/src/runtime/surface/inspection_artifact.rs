use super::super::{ForgeQueryAuthorityLane, ForgeQueryRuntimeInspectionEvidence};
use super::mutation::ForgeQueryWriteReceipt;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

pub struct ForgeQueryArtifactInspector<'a> {
    pub(in crate::runtime) receipt: &'a ForgeQueryWriteReceipt,
    pub(in crate::runtime) runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
}

impl<'a> ForgeQueryArtifactInspector<'a> {
    pub fn canonical(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "canonical",
            self.receipt.commit_evidence_identity().clone(),
            self.receipt.snapshot_evidence_identity().clone(),
        )
    }

    pub fn workflow(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "workflow",
            self.receipt.commit_evidence_identity().clone(),
            self.receipt.snapshot_evidence_identity().clone(),
        )
    }

    pub fn bridge_authority(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            self.receipt.commit_evidence_identity().clone(),
            self.receipt.snapshot_evidence_identity().clone(),
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.receipt.authority_lane()
    }

    pub fn runtime_evidence(&self) -> &ForgeQueryRuntimeInspectionEvidence {
        &self.runtime_evidence
    }

    pub fn live_patch_artifacts(&self) -> Vec<ForgeQueryEvidenceIdentity> {
        self.receipt
            .deltas()
            .iter()
            .map(|delta| {
                forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::WriteReceiptInspectionArtifact,
                )
                .field_shape(ForgeQueryEvidenceTag::new("role"), "live-patch-artifact")
                .field_value(ForgeQueryEvidenceTag::new("collection"), delta.collection())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("entity_identity"),
                    &delta.entity_identity.evidence_identity(),
                )
                .seal()
            })
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInspectedArtifact {
    pub(super) family: String,
    pub(super) identity: ForgeQueryEvidenceIdentity,
    pub(super) basis: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryInspectedArtifact {
    pub(in crate::runtime) fn new(
        family: impl Into<String>,
        identity: ForgeQueryEvidenceIdentity,
        basis: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            family: family.into(),
            identity: identity.into(),
            basis: basis.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    pub fn basis(&self) -> &str {
        self.basis.as_str()
    }
}
