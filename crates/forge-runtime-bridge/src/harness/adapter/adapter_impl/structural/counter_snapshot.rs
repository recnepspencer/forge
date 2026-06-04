use super::*;
use crate::diagnostics::BridgeStructuralCounters;
use crate::structural::{
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralMatchCandidateKind,
    StructuralMatchOutcomeClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StructuralHarnessCounterSnapshot {
    pub(super) structural_declaration_count: usize,
    pub(super) structural_contract_count: usize,
    pub(super) structural_fingerprint_count: usize,
    pub(super) structural_match_packet_count: usize,
    pub(super) structural_candidate_count: usize,
    pub(super) structural_candidate_cohort_count: usize,
    pub(super) structural_exact_match_count: usize,
    pub(super) structural_ambiguity_count: usize,
    pub(super) structural_mismatch_count: usize,
    pub(super) structural_identity_conflict_count: usize,
    pub(super) structural_lineage_divergence_count: usize,
    pub(super) structural_reuse_publication_count: usize,
    pub(super) branch_comparison_count: usize,
    pub(super) branch_comparison_diff_count: usize,
    pub(super) branch_comparison_drift_rejection_count: usize,
    pub(super) structural_widened_scan_count: usize,
    pub(super) structural_replay_request_count: usize,
    pub(super) structural_replay_mismatch_count: usize,
}

impl StructuralHarnessCounterSnapshot {
    pub(super) fn from_counters(
        counters: &BridgeStructuralCounters,
        replay_requested: bool,
    ) -> Self {
        let counters = if replay_requested {
            counters.with_replay_request()
        } else {
            *counters
        };
        Self {
            structural_declaration_count: counters.structural_declaration_count(),
            structural_contract_count: counters.structural_contract_count(),
            structural_fingerprint_count: counters.structural_fingerprint_count(),
            structural_match_packet_count: counters.structural_match_packet_count(),
            structural_candidate_count: counters.structural_candidate_count(),
            structural_candidate_cohort_count: counters.structural_candidate_cohort_count(),
            structural_exact_match_count: counters.structural_exact_match_count(),
            structural_ambiguity_count: counters.structural_ambiguity_count(),
            structural_mismatch_count: counters.structural_mismatch_count(),
            structural_identity_conflict_count: counters.structural_identity_conflict_count(),
            structural_lineage_divergence_count: counters.structural_lineage_divergence_count(),
            structural_reuse_publication_count: counters.structural_reuse_publication_count(),
            branch_comparison_count: counters.branch_comparison_count(),
            branch_comparison_diff_count: counters.branch_comparison_diff_count(),
            branch_comparison_drift_rejection_count: counters
                .branch_comparison_drift_rejection_count(),
            structural_widened_scan_count: counters.structural_widened_scan_count(),
            structural_replay_request_count: counters.structural_replay_request_count(),
            structural_replay_mismatch_count: counters.structural_replay_mismatch_count(),
        }
    }

    pub(super) fn from_rejection(
        contract: &AdmittedStructuralComparisonContract,
        planned: &PlannedStructuralMatchPacketSet,
        reduced: &ReducedStructuralMatchSet,
    ) -> Self {
        Self {
            structural_declaration_count: 1,
            structural_contract_count: 1,
            structural_fingerprint_count: rejection_fingerprint_count(planned),
            structural_match_packet_count: 1,
            structural_candidate_count: planned.candidate_count(),
            structural_candidate_cohort_count: planned.candidate_count(),
            structural_exact_match_count: count_candidates(
                planned,
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            ),
            structural_ambiguity_count: usize::from(
                reduced.outcome_class()
                    == StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch,
            ),
            structural_mismatch_count: usize::from(
                reduced.outcome_class().mismatch_class().is_some(),
            ),
            structural_identity_conflict_count: count_candidates(
                planned,
                StructuralMatchCandidateKind::IdentityAuthorityConflict,
            ),
            structural_lineage_divergence_count: count_candidates(
                planned,
                StructuralMatchCandidateKind::LineageStructuralDivergence,
            ),
            structural_reuse_publication_count: 0,
            branch_comparison_count: usize::from(
                contract
                    .validated_declaration()
                    .declaration()
                    .comparison_mode()
                    == StructuralComparisonMode::BranchComparison,
            ),
            branch_comparison_diff_count: count_candidates(
                planned,
                StructuralMatchCandidateKind::BranchDiff,
            ),
            branch_comparison_drift_rejection_count: 0,
            structural_widened_scan_count: usize::from(
                contract
                    .validated_declaration()
                    .declaration()
                    .candidate_scope()
                    == StructuralCandidateSearchScope::ExplicitWidenedDebtScan,
            ),
            structural_replay_request_count: 0,
            structural_replay_mismatch_count: 0,
        }
    }
}

fn rejection_fingerprint_count(planned: &PlannedStructuralMatchPacketSet) -> usize {
    planned.target_fingerprint().iter().count()
        + planned.comparison_fingerprint().iter().count()
        + planned
            .candidates()
            .iter()
            .filter(|candidate| candidate.fingerprint().is_some())
            .count()
}

fn count_candidates(
    planned: &PlannedStructuralMatchPacketSet,
    kind: StructuralMatchCandidateKind,
) -> usize {
    planned
        .candidates()
        .iter()
        .filter(|candidate| candidate.candidate_kind() == kind)
        .count()
}
