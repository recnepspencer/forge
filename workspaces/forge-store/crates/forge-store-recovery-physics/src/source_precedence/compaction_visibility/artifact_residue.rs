use super::{
    AdmittedCompactionCutoverRecord, CompactionGenerationIdentity,
    CompactionVisibleProductEvidence, RecoverableOldCompactionGeneration,
};
use crate::source_precedence::RecoveryCandidateDiscoveryTrace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionArtifactResidueRejection {
    reason: CompactionArtifactResidueReason,
    trace: RecoveryCandidateDiscoveryTrace,
}

impl CompactionArtifactResidueRejection {
    pub fn new(
        reason: CompactionArtifactResidueReason,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self { reason, trace }
    }

    pub const fn reason(&self) -> CompactionArtifactResidueReason {
        self.reason
    }

    pub const fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        &self.trace
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionArtifactResidueReason {
    MissingGenerationIdentity,
    MissingAdmittedCutover,
    OldGenerationNotRecoverable,
    CutoverDurabilityNotAdmitted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactionGenerationVisibility {
    VisibleAfterAdmittedCutover { generation: u64 },
    ResidueRejected(CompactionArtifactResidueRejection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionCutoverRecoveryPosture {
    visibility: CompactionGenerationVisibility,
}

impl CompactionCutoverRecoveryPosture {
    pub fn admit_visible_product(evidence: CompactionVisibleProductEvidence) -> Self {
        Self::visible_after_admitted_cutover(evidence.generation().generation())
    }

    pub fn missing_generation_identity(trace: RecoveryCandidateDiscoveryTrace) -> Self {
        Self::residue_rejected(CompactionArtifactResidueRejection::new(
            CompactionArtifactResidueReason::MissingGenerationIdentity,
            trace,
        ))
    }

    pub fn missing_admitted_cutover(
        _: CompactionGenerationIdentity,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::residue_rejected(CompactionArtifactResidueRejection::new(
            CompactionArtifactResidueReason::MissingAdmittedCutover,
            trace,
        ))
    }

    pub fn old_generation_not_recoverable(
        _: CompactionGenerationIdentity,
        _: AdmittedCompactionCutoverRecord,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::residue_rejected(CompactionArtifactResidueRejection::new(
            CompactionArtifactResidueReason::OldGenerationNotRecoverable,
            trace,
        ))
    }

    pub fn cutover_durability_not_admitted(
        _: CompactionGenerationIdentity,
        _: AdmittedCompactionCutoverRecord,
        _: RecoverableOldCompactionGeneration,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::residue_rejected(CompactionArtifactResidueRejection::new(
            CompactionArtifactResidueReason::CutoverDurabilityNotAdmitted,
            trace,
        ))
    }

    pub(crate) fn visible_after_admitted_cutover(generation: u64) -> Self {
        Self {
            visibility: CompactionGenerationVisibility::VisibleAfterAdmittedCutover { generation },
        }
    }

    pub fn residue_rejected(rejection: CompactionArtifactResidueRejection) -> Self {
        Self {
            visibility: CompactionGenerationVisibility::ResidueRejected(rejection),
        }
    }

    pub const fn visibility(&self) -> &CompactionGenerationVisibility {
        &self.visibility
    }

    pub const fn is_visible(&self) -> bool {
        matches!(
            self.visibility,
            CompactionGenerationVisibility::VisibleAfterAdmittedCutover { .. }
        )
    }
}
