mod inspection_foreign_evidence_citation;
mod inspection_foreign_evidence_ref;

use crate::UiInspectionSupportWorld;

pub use inspection_foreign_evidence_citation::{
    UiInspectionForeignEvidenceCitation, UiInspectionQueryForeignEvidenceCitation,
};
pub use inspection_foreign_evidence_ref::{
    UiInspectionForeignEvidenceRef, UiInspectionQueryForeignEvidenceArtifactKind,
    UiInspectionQueryForeignEvidenceKind, UiInspectionQueryForeignEvidenceRef,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceFamily {
    Declaration,
    Admission,
    Graph,
    Planning,
    Aspect,
    Obligation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceAuthorityKind {
    DeclarationArtifact,
    AdmissionReport,
    GraphSnapshot,
    AllocationPlanning,
    AspectAuthority,
    ObligationAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceAuthorityGeneration {
    value: u64,
}

impl UiEvidenceAuthorityGeneration {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn as_u64(self) -> u64 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceAuthorityArtifactIdentity {
    kind: UiEvidenceAuthorityKind,
    digest: u64,
}

impl UiEvidenceAuthorityArtifactIdentity {
    pub const fn new(kind: UiEvidenceAuthorityKind, digest: u64) -> Self {
        Self { kind, digest }
    }

    pub const fn kind(self) -> UiEvidenceAuthorityKind {
        self.kind
    }

    pub const fn digest(self) -> u64 {
        self.digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEvidenceAuthorityBinding {
    artifact_identity: UiEvidenceAuthorityArtifactIdentity,
    authority_generation: UiEvidenceAuthorityGeneration,
    world: Option<UiInspectionSupportWorld>,
}

impl UiEvidenceAuthorityBinding {
    pub const fn new(
        artifact_identity: UiEvidenceAuthorityArtifactIdentity,
        authority_generation: UiEvidenceAuthorityGeneration,
        world: Option<UiInspectionSupportWorld>,
    ) -> Self {
        Self {
            artifact_identity,
            authority_generation,
            world,
        }
    }

    pub const fn artifact_identity(self) -> UiEvidenceAuthorityArtifactIdentity {
        self.artifact_identity
    }

    pub const fn authority_generation(self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }

    pub const fn world(self) -> Option<UiInspectionSupportWorld> {
        self.world
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceMaterializationPosture {
    RefsOnly,
    SummaryAvailable,
    DetailAvailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceRetentionPosture {
    CurrentGenerationOnly,
    RetainedForInspection,
    RetainedForReplay,
    RetainedUntilCloseout,
    DiscardedWithTombstone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceExpansionOutcome {
    Available,
    Discarded {
        retention: UiEvidenceRetentionPosture,
    },
    WrongGeneration {
        requested_generation: UiEvidenceAuthorityGeneration,
        current_generation: UiEvidenceAuthorityGeneration,
    },
    NotMaterialized {
        posture: UiEvidenceMaterializationPosture,
    },
    Unsupported,
}

impl UiEvidenceExpansionOutcome {
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}
