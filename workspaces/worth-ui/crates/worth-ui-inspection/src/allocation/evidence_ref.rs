#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationInspectionEvidenceFamily {
    InvalidationArtifact,
    NeighborhoodSelectionArtifact,
    ReuseDecisionArtifact,
    DenialArtifact,
    GeometryArtifact,
}

/// Typed diagnostic citation. It has no authority in an operational lane.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionEvidenceRef {
    family: UiAllocationInspectionEvidenceFamily,
    identity: u64,
}

impl UiAllocationInspectionEvidenceRef {
    pub const fn diagnostic(family: UiAllocationInspectionEvidenceFamily, identity: u64) -> Self {
        Self { family, identity }
    }

    pub const fn family(self) -> UiAllocationInspectionEvidenceFamily {
        self.family
    }
    pub const fn identity(self) -> u64 {
        self.identity
    }
}
