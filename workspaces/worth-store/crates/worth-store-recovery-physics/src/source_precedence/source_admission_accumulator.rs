use super::{
    BackendResidueKind, BackendResidueRejection, CheckpointBaseAdmission,
    CompactionGenerationVisibility, RecoverySourceApplicationRole, RecoverySourceCandidate,
    RecoverySourceDecisionOutcome, RecoverySourceDecisionRow, WalTailRedoSource,
};
use crate::RecoveryBlockedByIntegrityDamage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecoverySourceSelectionInput {
    pub(super) candidate_count: usize,
    pub(super) checkpoint_bases: Vec<CheckpointBaseAdmission>,
    pub(super) wal_tails: Vec<WalTailRedoSource>,
    pub(super) roles: Vec<RecoverySourceApplicationRole>,
    pub(super) residue_rejections: Vec<BackendResidueRejection>,
    pub(super) decision_rows: Vec<RecoverySourceDecisionRow>,
    pub(super) recovery_blocked: Option<RecoveryBlockedByIntegrityDamage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecoverySourceAdmissionAccumulator {
    input: RecoverySourceSelectionInput,
}

impl RecoverySourceAdmissionAccumulator {
    pub(super) fn from_candidates(candidates: Vec<RecoverySourceCandidate>) -> Self {
        let mut accumulator = Self::new(candidates.len());
        for candidate in candidates {
            accumulator.admit_candidate(candidate);
            if accumulator.input.recovery_blocked.is_some() {
                break;
            }
        }
        accumulator
    }

    pub(super) fn into_selection_input(self) -> RecoverySourceSelectionInput {
        self.input
    }

    fn new(candidate_count: usize) -> Self {
        Self {
            input: RecoverySourceSelectionInput {
                candidate_count,
                checkpoint_bases: Vec::new(),
                wal_tails: Vec::new(),
                roles: Vec::new(),
                residue_rejections: Vec::new(),
                decision_rows: Vec::new(),
                recovery_blocked: None,
            },
        }
    }

    fn admit_candidate(&mut self, candidate: RecoverySourceCandidate) {
        match candidate {
            RecoverySourceCandidate::RecoveryBlocked { damage, trace } => {
                self.push_decision(
                    trace,
                    RecoverySourceApplicationRole::RecoveryBlocked,
                    RecoverySourceDecisionOutcome::RecoveryBlocked,
                );
                self.input.recovery_blocked = Some(damage);
            }
            RecoverySourceCandidate::CheckpointBase { admission, trace } => {
                self.push_decision(
                    trace,
                    RecoverySourceApplicationRole::CheckpointBase,
                    RecoverySourceDecisionOutcome::AdmittedCandidate,
                );
                self.input.checkpoint_bases.push(admission);
            }
            RecoverySourceCandidate::WalTail { source, trace } => {
                self.push_decision(
                    trace,
                    RecoverySourceApplicationRole::WalTailRedo,
                    RecoverySourceDecisionOutcome::AdmittedCandidate,
                );
                self.input.wal_tails.push(source);
            }
            RecoverySourceCandidate::PageImage { trace, .. } => {
                self.push_decision(
                    trace,
                    RecoverySourceApplicationRole::PageSkipApply,
                    RecoverySourceDecisionOutcome::ApplicationRoleOnly,
                );
            }
            RecoverySourceCandidate::CompactionProduct { posture, trace } => {
                self.admit_compaction_product(posture.visibility(), trace);
            }
            RecoverySourceCandidate::BackendResidue { rejection, trace }
            | RecoverySourceCandidate::OrphanedCheckpointManifest { rejection, trace } => {
                self.push_decision(
                    trace,
                    RecoverySourceApplicationRole::ResidueDiscoveryOnly,
                    RecoverySourceDecisionOutcome::DiscoveryOnly,
                );
                self.input.residue_rejections.push(rejection);
            }
        }
    }

    fn admit_compaction_product(
        &mut self,
        visibility: &CompactionGenerationVisibility,
        trace: super::RecoveryCandidateDiscoveryTrace,
    ) {
        let role = RecoverySourceApplicationRole::CompactionVisibility;
        match visibility {
            CompactionGenerationVisibility::VisibleAfterAdmittedCutover { .. } => {
                self.push_decision(
                    trace,
                    role,
                    RecoverySourceDecisionOutcome::AdmittedCandidate,
                );
            }
            CompactionGenerationVisibility::ResidueRejected(rejection) => {
                self.push_decision(trace, role, RecoverySourceDecisionOutcome::RejectedResidue);
                self.input
                    .residue_rejections
                    .push(BackendResidueRejection::new(
                        BackendResidueKind::InvalidCompactionProduct,
                        rejection.trace().clone(),
                    ));
            }
        }
    }

    fn push_decision(
        &mut self,
        trace: super::RecoveryCandidateDiscoveryTrace,
        role: RecoverySourceApplicationRole,
        outcome: RecoverySourceDecisionOutcome,
    ) {
        self.input.roles.push(role);
        self.input
            .decision_rows
            .push(RecoverySourceDecisionRow::new(trace, role, outcome));
    }
}
