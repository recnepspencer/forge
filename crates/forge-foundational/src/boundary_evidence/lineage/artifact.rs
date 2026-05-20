use crate::BoundaryHandle;

use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::super::receipts::FoundationalBoundaryEvidenceExecutedReceiptArtifact;
use super::definitions::{
    FoundationalBoundaryEvidenceBranchDivergencePosture,
    FoundationalBoundaryEvidenceLineageOutcomeKind,
    FoundationalBoundaryEvidenceLineagePartialityPosture,
    FoundationalBoundaryEvidencePromotionPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBoundaryEvidenceLineageSubject(BoundaryHandle);

impl FoundationalBoundaryEvidenceLineageSubject {
    pub const fn new(handle: BoundaryHandle) -> Self {
        Self(handle)
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceLineageSubjectSet(
    Vec<FoundationalBoundaryEvidenceLineageSubject>,
);

impl FoundationalBoundaryEvidenceLineageSubjectSet {
    pub fn new(
        mut subjects: Vec<FoundationalBoundaryEvidenceLineageSubject>,
    ) -> Result<Self, FoundationalBoundaryEvidenceLineageConstructionDenial> {
        subjects.sort();
        subjects.dedup();

        if subjects.is_empty() {
            return Err(
                FoundationalBoundaryEvidenceLineageConstructionDenial::RelatedSubjectSetMustNotBeEmpty,
            );
        }

        Ok(Self(subjects))
    }

    pub fn subjects(&self) -> &[FoundationalBoundaryEvidenceLineageSubject] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceLineageConstructionDenial {
    ReplayDerivedContinuityRequiresReplayDerivedProvenance,
    RestoredContinuityRequiresRestorationOrCheckpointReceipt,
    ReconstructedEquivalenceRequiresReplayOrRestoredProvenance,
    PromotionDeniedDoesNotProduceGlobalContinuity,
    RelatedSubjectSetMustNotBeEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceAttestedLineageArtifact {
    outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
    subject: FoundationalBoundaryEvidenceLineageSubject,
    related_subjects: Option<FoundationalBoundaryEvidenceLineageSubjectSet>,
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl FoundationalBoundaryEvidenceAttestedLineageArtifact {
    pub(crate) fn new(
        outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
        subject: FoundationalBoundaryEvidenceLineageSubject,
        related_subjects: Option<FoundationalBoundaryEvidenceLineageSubjectSet>,
        executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> Self {
        Self {
            outcome_kind,
            subject,
            related_subjects,
            executed_receipt,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        self.outcome_kind
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn related_subjects(&self) -> Option<&FoundationalBoundaryEvidenceLineageSubjectSet> {
        self.related_subjects.as_ref()
    }

    pub fn executed_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.executed_receipt
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.executed_receipt.provenance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceBranchLocalLineageArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    divergence_posture: FoundationalBoundaryEvidenceBranchDivergencePosture,
    branch_local_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl FoundationalBoundaryEvidenceBranchLocalLineageArtifact {
    pub(crate) fn new(
        subject: FoundationalBoundaryEvidenceLineageSubject,
        divergence_posture: FoundationalBoundaryEvidenceBranchDivergencePosture,
        branch_local_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> Self {
        Self {
            subject,
            divergence_posture,
            branch_local_receipt,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        FoundationalBoundaryEvidenceLineageOutcomeKind::BranchLocalReplacement
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub const fn divergence_posture(&self) -> FoundationalBoundaryEvidenceBranchDivergencePosture {
        self.divergence_posture
    }

    pub fn branch_local_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.branch_local_receipt
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.branch_local_receipt.provenance()
    }

    pub(crate) fn into_promoted(
        self,
        promotion_posture: FoundationalBoundaryEvidencePromotionPosture,
        promotion_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> Result<
        FoundationalBoundaryEvidencePromotedLineageArtifact,
        FoundationalBoundaryEvidenceLineageConstructionDenial,
    > {
        if matches!(
            promotion_posture,
            FoundationalBoundaryEvidencePromotionPosture::PromotionDenied
        ) {
            return Err(
                FoundationalBoundaryEvidenceLineageConstructionDenial::PromotionDeniedDoesNotProduceGlobalContinuity,
            );
        }

        Ok(FoundationalBoundaryEvidencePromotedLineageArtifact {
            subject: self.subject,
            divergence_posture: self.divergence_posture,
            promotion_posture,
            branch_local_receipt: self.branch_local_receipt,
            promotion_receipt,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidencePromotedLineageArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    divergence_posture: FoundationalBoundaryEvidenceBranchDivergencePosture,
    promotion_posture: FoundationalBoundaryEvidencePromotionPosture,
    branch_local_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    promotion_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl FoundationalBoundaryEvidencePromotedLineageArtifact {
    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        FoundationalBoundaryEvidenceLineageOutcomeKind::SingularContinuity
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub const fn divergence_posture(&self) -> FoundationalBoundaryEvidenceBranchDivergencePosture {
        self.divergence_posture
    }

    pub const fn promotion_posture(&self) -> FoundationalBoundaryEvidencePromotionPosture {
        self.promotion_posture
    }

    pub fn branch_local_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.branch_local_receipt
    }

    pub fn promotion_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.promotion_receipt
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.promotion_receipt.provenance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceReplayDerivedLineageArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
}

impl FoundationalBoundaryEvidenceReplayDerivedLineageArtifact {
    pub(crate) fn new(
        subject: FoundationalBoundaryEvidenceLineageSubject,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        Self {
            subject,
            provenance,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        FoundationalBoundaryEvidenceLineageOutcomeKind::SingularContinuity
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceRestoredLineageArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    restoration_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl FoundationalBoundaryEvidenceRestoredLineageArtifact {
    pub(crate) fn new(
        subject: FoundationalBoundaryEvidenceLineageSubject,
        restoration_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> Self {
        Self {
            subject,
            restoration_receipt,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        FoundationalBoundaryEvidenceLineageOutcomeKind::RestoredContinuity
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn restoration_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.restoration_receipt
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.restoration_receipt.provenance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
}

impl FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact {
    pub(crate) fn new(
        subject: FoundationalBoundaryEvidenceLineageSubject,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        Self {
            subject,
            provenance,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        FoundationalBoundaryEvidenceLineageOutcomeKind::ReconstructedEquivalence
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidencePartialLineageArtifact {
    outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
    partiality_posture: FoundationalBoundaryEvidenceLineagePartialityPosture,
    subject: FoundationalBoundaryEvidenceLineageSubject,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
}

impl FoundationalBoundaryEvidencePartialLineageArtifact {
    pub(crate) fn new(
        outcome_kind: FoundationalBoundaryEvidenceLineageOutcomeKind,
        partiality_posture: FoundationalBoundaryEvidenceLineagePartialityPosture,
        subject: FoundationalBoundaryEvidenceLineageSubject,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        Self {
            outcome_kind,
            partiality_posture,
            subject,
            provenance,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalBoundaryEvidenceLineageOutcomeKind {
        self.outcome_kind
    }

    pub const fn partiality_posture(&self) -> FoundationalBoundaryEvidenceLineagePartialityPosture {
        self.partiality_posture
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }
}
