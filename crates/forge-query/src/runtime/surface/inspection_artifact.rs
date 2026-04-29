use super::super::{ForgeQueryAuthorityLane, ForgeQueryRuntimeInspectionEvidence};
use super::mutation::ForgeQueryWriteReceipt;

pub struct ForgeQueryArtifactInspector<'a> {
    pub(in crate::runtime) receipt: &'a ForgeQueryWriteReceipt,
    pub(in crate::runtime) runtime_evidence: ForgeQueryRuntimeInspectionEvidence,
}

impl<'a> ForgeQueryArtifactInspector<'a> {
    pub fn canonical(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "canonical",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn workflow(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "workflow",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn bridge_authority(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.receipt.authority_lane()
    }

    pub fn runtime_evidence(&self) -> &ForgeQueryRuntimeInspectionEvidence {
        &self.runtime_evidence
    }

    pub fn live_patch_artifacts(&self) -> Vec<String> {
        self.receipt
            .deltas()
            .iter()
            .map(|delta| format!("{}:{}", delta.collection, delta.entity_identity))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInspectedArtifact {
    pub(super) family: String,
    pub(super) identity: String,
    pub(super) basis: String,
}

impl ForgeQueryInspectedArtifact {
    pub(in crate::runtime) fn new(
        family: impl Into<String>,
        identity: impl Into<String>,
        basis: impl Into<String>,
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
        &self.identity
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }
}
