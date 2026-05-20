use forge_proof::TransitionOutcome;

use super::lineage::{
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceBranchDivergencePosture,
    FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    FoundationalBoundaryEvidenceLineageConstructionDenial,
    FoundationalBoundaryEvidenceLineageOutcomeKind,
    FoundationalBoundaryEvidenceLineagePartialityPosture,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidenceLineageSubjectSet,
    FoundationalBoundaryEvidencePartialLineageArtifact,
    FoundationalBoundaryEvidencePromotedLineageArtifact,
    FoundationalBoundaryEvidencePromotionPosture,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
};
use super::primitives::FoundationalBoundaryEvidenceLocality;
use super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::receipts::{
    FoundationalBoundaryEvidenceExecutedReceiptArtifact, FoundationalBoundaryEvidenceReceiptKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceLineageFrontDoor;

impl FoundationalBoundaryEvidenceLineageFrontDoor {
    pub fn continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceAttestedLineageStep {
        FoundationalBoundaryEvidenceAttestedLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::SingularContinuity,
            subject,
        )
    }

    pub fn plural_successor_predecessor(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceAttestedLineageStep {
        FoundationalBoundaryEvidenceAttestedLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::PluralSuccessorPredecessor,
            subject,
        )
    }

    pub fn merge_successor(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceAttestedLineageStep {
        FoundationalBoundaryEvidenceAttestedLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::MergeSuccessor,
            subject,
        )
    }

    pub fn identity_break(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceAttestedLineageStep {
        FoundationalBoundaryEvidenceAttestedLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::IdentityBreak,
            subject,
        )
    }

    pub fn transient_within_boundary_closure(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceAttestedLineageStep {
        FoundationalBoundaryEvidenceAttestedLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::TransientWithinBoundaryClosure,
            subject,
        )
    }

    pub fn branch_local_replacement(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceBranchLocalLineageStep {
        FoundationalBoundaryEvidenceBranchLocalLineageStep::new(subject)
    }

    pub fn replay_derived_continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceReplayDerivedLineageStep {
        FoundationalBoundaryEvidenceReplayDerivedLineageStep::new(subject)
    }

    pub fn restored_continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceRestoredLineageStep {
        FoundationalBoundaryEvidenceRestoredLineageStep::new(subject)
    }

    pub fn reconstructed_equivalence(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceReconstructedEquivalenceStep {
        FoundationalBoundaryEvidenceReconstructedEquivalenceStep::new(subject)
    }

    pub fn named_gap_partial_continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidencePartialLineageStep {
        FoundationalBoundaryEvidencePartialLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::NamedGapPartialContinuity,
            FoundationalBoundaryEvidenceLineagePartialityPosture::NamedGap,
            subject,
        )
    }

    pub fn withheld_redacted_continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidencePartialLineageStep {
        FoundationalBoundaryEvidencePartialLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::WithheldRedactedContinuity,
            FoundationalBoundaryEvidenceLineagePartialityPosture::WithheldRedacted,
            subject,
        )
    }

    pub fn denied_continuity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidencePartialLineageStep {
        FoundationalBoundaryEvidencePartialLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::Denial,
            FoundationalBoundaryEvidenceLineagePartialityPosture::Denied,
            subject,
        )
    }

    pub fn advisory_correspondence_candidate(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidencePartialLineageStep {
        FoundationalBoundaryEvidencePartialLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::AdvisoryCorrespondenceCandidate,
            FoundationalBoundaryEvidenceLineagePartialityPosture::NamedGap,
            subject,
        )
    }

    pub fn ambiguity(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidencePartialLineageStep {
        FoundationalBoundaryEvidencePartialLineageStep::new(
            FoundationalBoundaryEvidenceLineageOutcomeKind::Ambiguity,
            FoundationalBoundaryEvidenceLineagePartialityPosture::NamedGap,
            subject,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceAttestedLineageStep {
    outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
    subject: FoundationalBoundaryEvidenceLineageSubject,
    related_subjects: Option<FoundationalBoundaryEvidenceLineageSubjectSet>,
}

impl FoundationalBoundaryEvidenceAttestedLineageStep {
    fn new(
        outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> Self {
        Self {
            outcome_kind,
            subject,
            related_subjects: None,
        }
    }

    pub fn related_subjects(
        mut self,
        related_subjects: FoundationalBoundaryEvidenceLineageSubjectSet,
    ) -> Self {
        self.related_subjects = Some(related_subjects);
        self
    }

    pub fn attested_by(
        self,
        executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> FoundationalBoundaryEvidenceAttestedLineageArtifact {
        FoundationalBoundaryEvidenceAttestedLineageArtifact::new(
            self.outcome_kind,
            self.subject,
            self.related_subjects,
            executed_receipt,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceBranchLocalLineageStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    divergence_posture: FoundationalBoundaryEvidenceBranchDivergencePosture,
}

impl FoundationalBoundaryEvidenceBranchLocalLineageStep {
    fn new(subject: FoundationalBoundaryEvidenceLineageSubject) -> Self {
        Self {
            subject,
            divergence_posture:
                FoundationalBoundaryEvidenceBranchDivergencePosture::BranchLocalOnly,
        }
    }

    pub fn with_divergence(
        mut self,
        divergence_posture: FoundationalBoundaryEvidenceBranchDivergencePosture,
    ) -> Self {
        self.divergence_posture = divergence_posture;
        self
    }

    pub fn attested_by(
        self,
        branch_local_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> FoundationalBoundaryEvidenceBranchLocalLineageArtifact {
        FoundationalBoundaryEvidenceBranchLocalLineageArtifact::new(
            self.subject,
            self.divergence_posture,
            branch_local_receipt,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceReplayDerivedLineageStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
}

impl FoundationalBoundaryEvidenceReplayDerivedLineageStep {
    fn new(subject: FoundationalBoundaryEvidenceLineageSubject) -> Self {
        Self { subject }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
        FoundationalBoundaryEvidenceLineageConstructionDenial,
    > {
        if provenance.locality() != FoundationalBoundaryEvidenceLocality::ReplayDerived {
            return TransitionOutcome::denied(
                FoundationalBoundaryEvidenceLineageConstructionDenial::ReplayDerivedContinuityRequiresReplayDerivedProvenance,
            );
        }

        TransitionOutcome::success(
            FoundationalBoundaryEvidenceReplayDerivedLineageArtifact::new(self.subject, provenance),
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceRestoredLineageStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
}

impl FoundationalBoundaryEvidenceRestoredLineageStep {
    fn new(subject: FoundationalBoundaryEvidenceLineageSubject) -> Self {
        Self { subject }
    }

    pub fn attested_by(
        self,
        restoration_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidenceRestoredLineageArtifact,
        FoundationalBoundaryEvidenceLineageConstructionDenial,
    > {
        match restoration_receipt.receipt_kind() {
            FoundationalBoundaryEvidenceReceiptKind::Restoration
            | FoundationalBoundaryEvidenceReceiptKind::CheckpointResume => {
                TransitionOutcome::success(FoundationalBoundaryEvidenceRestoredLineageArtifact::new(
                    self.subject,
                    restoration_receipt,
                ))
            }
            _ => TransitionOutcome::denied(
                FoundationalBoundaryEvidenceLineageConstructionDenial::RestoredContinuityRequiresRestorationOrCheckpointReceipt,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceReconstructedEquivalenceStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
}

impl FoundationalBoundaryEvidenceReconstructedEquivalenceStep {
    fn new(subject: FoundationalBoundaryEvidenceLineageSubject) -> Self {
        Self { subject }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
        FoundationalBoundaryEvidenceLineageConstructionDenial,
    > {
        match provenance.locality() {
            FoundationalBoundaryEvidenceLocality::ReplayDerived
            | FoundationalBoundaryEvidenceLocality::RestoredReadmitted => {
                TransitionOutcome::success(
                    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact::new(
                        self.subject,
                        provenance,
                    ),
                )
            }
            _ => TransitionOutcome::denied(
                FoundationalBoundaryEvidenceLineageConstructionDenial::ReconstructedEquivalenceRequiresReplayOrRestoredProvenance,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidencePartialLineageStep {
    outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
    partiality_posture: FoundationalBoundaryEvidenceLineagePartialityPosture,
    subject: FoundationalBoundaryEvidenceLineageSubject,
}

impl FoundationalBoundaryEvidencePartialLineageStep {
    fn new(
        outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
        partiality_posture: FoundationalBoundaryEvidenceLineagePartialityPosture,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> Self {
        Self {
            outcome_kind,
            partiality_posture,
            subject,
        }
    }

    pub fn with_provenance(
        self,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> FoundationalBoundaryEvidencePartialLineageArtifact {
        FoundationalBoundaryEvidencePartialLineageArtifact::new(
            self.outcome_kind,
            self.partiality_posture,
            self.subject,
            provenance,
        )
    }
}

impl FoundationalBoundaryEvidenceBranchLocalLineageArtifact {
    pub fn promote_with(
        self,
        promotion_posture: FoundationalBoundaryEvidencePromotionPosture,
        promotion_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidencePromotedLineageArtifact,
        FoundationalBoundaryEvidenceLineageConstructionDenial,
    > {
        match self.into_promoted(promotion_posture, promotion_receipt) {
            Ok(promoted) => TransitionOutcome::success(promoted),
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}
