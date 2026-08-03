use crate::{
    source_precedence::RecoverySourcePrecedenceGraph, AdmittedRecoverySource,
    RecoverySourceCandidate, RecoverySourceLayoutReport,
};

use super::{RecoveryBudget, RecoveryBudgetDenial, RecoveryBudgetDenialKind};

#[derive(Debug)]
pub struct BoundedRecoverySourcePrecedenceGraph<'runtime> {
    budget: RecoveryBudget<'runtime>,
    graph: RecoverySourcePrecedenceGraph,
    evidence: RecoveryWorkBudgetEvidence,
}

impl<'runtime> BoundedRecoverySourcePrecedenceGraph<'runtime> {
    pub(crate) fn new(budget: RecoveryBudget<'runtime>, profile: impl Into<String>) -> Self {
        Self {
            budget,
            graph: RecoverySourcePrecedenceGraph::new(profile),
            evidence: RecoveryWorkBudgetEvidence::new(),
        }
    }

    pub fn discover(
        self,
        candidate: RecoverySourceCandidate,
    ) -> Result<Self, RecoveryBudgetDenial> {
        let evidence = self.evidence.observe_candidate(&candidate);
        self.budget
            .require_source_candidate_count(self.graph.candidate_count() + 1)?;
        Ok(Self {
            budget: self.budget,
            graph: self.graph.discover(candidate),
            evidence,
        })
    }

    pub fn reject_full_store_scan(self, attempted_pages: u64) -> ForbiddenFullStoreScanRejection {
        let evidence = self.evidence.observe_forbidden_full_store_scan();
        ForbiddenFullStoreScanRejection {
            attempted_pages,
            checkpoint_interval_frames: self.budget.checkpoint_interval_frame_limit(),
            wal_tail_frame_limit: self.budget.wal_tail_frame_limit(),
            evidence,
        }
    }

    pub fn admit_sources(self) -> BoundedRecoverySourceAdmission {
        let source = self.graph.admit_sources();
        let layout_report = crate::layout_projection::project_recovery_source_layout(&source);
        BoundedRecoverySourceAdmission {
            source,
            layout_report,
            evidence: self.evidence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenFullStoreScanRejection {
    attempted_pages: u64,
    checkpoint_interval_frames: usize,
    wal_tail_frame_limit: usize,
    evidence: RecoveryWorkBudgetEvidence,
}

impl ForbiddenFullStoreScanRejection {
    pub const fn attempted_pages(self) -> u64 {
        self.attempted_pages
    }

    pub const fn checkpoint_interval_frames(self) -> usize {
        self.checkpoint_interval_frames
    }

    pub const fn wal_tail_frame_limit(self) -> usize {
        self.wal_tail_frame_limit
    }

    pub const fn forbidden_full_store_scans(self) -> u64 {
        self.evidence.forbidden_full_store_scans()
    }

    pub fn into_denial(self) -> RecoveryBudgetDenial {
        RecoveryBudgetDenial::new(RecoveryBudgetDenialKind::ForbiddenFullStoreScan {
            attempted_pages: self.attempted_pages,
            checkpoint_interval_frames: self.checkpoint_interval_frames,
            wal_tail_frame_limit: self.wal_tail_frame_limit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedRecoverySourceAdmission {
    source: AdmittedRecoverySource,
    layout_report: RecoverySourceLayoutReport,
    evidence: RecoveryWorkBudgetEvidence,
}

impl BoundedRecoverySourceAdmission {
    pub const fn source(&self) -> &AdmittedRecoverySource {
        &self.source
    }

    pub const fn layout_report(&self) -> &RecoverySourceLayoutReport {
        &self.layout_report
    }

    pub(crate) fn into_parts(self) -> (AdmittedRecoverySource, RecoveryWorkBudgetEvidence) {
        (self.source, self.evidence)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecoveryWorkBudgetEvidence {
    source_candidate_count: usize,
    validated_checkpoints: u64,
    scanned_segments: usize,
    residue_rejections: usize,
    forbidden_full_store_scans: u64,
}

impl RecoveryWorkBudgetEvidence {
    pub(crate) const fn new() -> Self {
        Self {
            source_candidate_count: 0,
            validated_checkpoints: 0,
            scanned_segments: 0,
            residue_rejections: 0,
            forbidden_full_store_scans: 0,
        }
    }

    pub(crate) const fn validated_checkpoints(self) -> u64 {
        self.validated_checkpoints
    }

    pub(crate) const fn scanned_segments(self) -> usize {
        self.scanned_segments
    }

    pub(crate) const fn residue_rejections(self) -> usize {
        self.residue_rejections
    }

    pub(crate) const fn forbidden_full_store_scans(self) -> u64 {
        self.forbidden_full_store_scans
    }

    fn observe_candidate(self, candidate: &RecoverySourceCandidate) -> Self {
        match candidate {
            RecoverySourceCandidate::CheckpointBase { admission, .. } => Self {
                source_candidate_count: self.source_candidate_count + 1,
                validated_checkpoints: self.validated_checkpoints
                    + admission.counters().manifest_validation_count(),
                ..self
            },
            RecoverySourceCandidate::WalTail { .. } => Self {
                source_candidate_count: self.source_candidate_count + 1,
                scanned_segments: self.scanned_segments + 1,
                ..self
            },
            RecoverySourceCandidate::OrphanedCheckpointManifest { .. }
            | RecoverySourceCandidate::BackendResidue { .. } => Self {
                source_candidate_count: self.source_candidate_count + 1,
                residue_rejections: self.residue_rejections + 1,
                ..self
            },
            _ => Self {
                source_candidate_count: self.source_candidate_count + 1,
                ..self
            },
        }
    }

    const fn observe_forbidden_full_store_scan(self) -> Self {
        Self {
            forbidden_full_store_scans: self.forbidden_full_store_scans + 1,
            ..self
        }
    }
}
