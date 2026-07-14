use crate::diagnostics::{FoundationalDiagnosticNamedGap, FoundationalDiagnosticRowFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticCertifiedCoverageClass {
    HostileCoveragePresent,
    PartialWithNamedGaps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticCertifiedCoverageDenial {
    CoverageIncompleteDenied,
    HappyPathOnlyDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticCoverageFamilyStatus {
    AbsentFromBundle,
    HostileRowsPresent { row_count: u32 },
    PartialWithNamedGap(FoundationalDiagnosticNamedGap),
    Denied(FoundationalDiagnosticCertifiedCoverageDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticCoverageMatrix {
    decision: FoundationalDiagnosticCoverageFamilyStatus,
    failure: FoundationalDiagnosticCoverageFamilyStatus,
    comparison: FoundationalDiagnosticCoverageFamilyStatus,
    support: FoundationalDiagnosticCoverageFamilyStatus,
    provenance_ready: FoundationalDiagnosticCoverageFamilyStatus,
}

impl FoundationalDiagnosticCoverageMatrix {
    pub fn new(
        decision: FoundationalDiagnosticCoverageFamilyStatus,
        failure: FoundationalDiagnosticCoverageFamilyStatus,
        comparison: FoundationalDiagnosticCoverageFamilyStatus,
        support: FoundationalDiagnosticCoverageFamilyStatus,
        provenance_ready: FoundationalDiagnosticCoverageFamilyStatus,
    ) -> Self {
        Self {
            decision,
            failure,
            comparison,
            support,
            provenance_ready,
        }
    }

    pub fn for_family(
        &self,
        family: FoundationalDiagnosticRowFamily,
    ) -> &FoundationalDiagnosticCoverageFamilyStatus {
        match family {
            FoundationalDiagnosticRowFamily::Decision => &self.decision,
            FoundationalDiagnosticRowFamily::Failure => &self.failure,
            FoundationalDiagnosticRowFamily::Comparison => &self.comparison,
            FoundationalDiagnosticRowFamily::Support => &self.support,
            FoundationalDiagnosticRowFamily::ProvenanceReady => &self.provenance_ready,
        }
    }

    pub fn decision(&self) -> &FoundationalDiagnosticCoverageFamilyStatus {
        &self.decision
    }

    pub fn failure(&self) -> &FoundationalDiagnosticCoverageFamilyStatus {
        &self.failure
    }

    pub fn comparison(&self) -> &FoundationalDiagnosticCoverageFamilyStatus {
        &self.comparison
    }

    pub fn support(&self) -> &FoundationalDiagnosticCoverageFamilyStatus {
        &self.support
    }

    pub fn provenance_ready(&self) -> &FoundationalDiagnosticCoverageFamilyStatus {
        &self.provenance_ready
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCertifiedDiagnosticSourceKind {
    CurrentBasisCommittedAuthority,
    CurrentBasisCommitReceipt,
    CurrentBasisBoundaryArtifact,
    CurrentBasisBoundaryBundle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCertifiedDiagnosticProvenanceHook {
    TransitionEvidenceOriginAttachment,
    BoundaryArtifactEvidenceOriginAttachment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticCertifiedAttachmentDenial {
    MissingSourceDigest,
    CoveredFamilyMustExposeHostileRows,
    CoveredFamilyCannotBeAbsentFromBundle,
    PartialCoverageRequiresNamedBundleGaps,
    PartialCoverageRequiresTypedNamedGap,
    TypedNamedGapMustBelongToBundle,
    CoverageIncompleteDenied,
    HappyPathOnlyDenied,
}
