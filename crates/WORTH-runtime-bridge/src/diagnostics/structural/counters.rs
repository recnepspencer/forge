use crate::structural::{
    AdmittedStructuralComparisonContract, PlannedStructuralMatchPacketSet,
    ReducedStructuralMatchSet, StructuralComparisonMode, StructuralMatchCandidateKind,
    StructuralMatchOutcomeClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeStructuralCounters {
    structural_declaration_count: usize,
    structural_contract_count: usize,
    structural_fingerprint_count: usize,
    structural_match_packet_count: usize,
    structural_candidate_count: usize,
    structural_candidate_cohort_count: usize,
    structural_exact_match_count: usize,
    structural_ambiguity_count: usize,
    structural_mismatch_count: usize,
    structural_identity_conflict_count: usize,
    structural_lineage_divergence_count: usize,
    structural_reuse_publication_count: usize,
    branch_comparison_count: usize,
    branch_comparison_diff_count: usize,
    branch_comparison_drift_rejection_count: usize,
    structural_widened_scan_count: usize,
    structural_replay_request_count: usize,
    structural_replay_mismatch_count: usize,
}

impl BridgeStructuralCounters {
    pub(crate) fn from_structural_outcome(
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Self {
        let declaration = contract.validated_declaration().declaration();
        let identity_conflict_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::IdentityAuthorityConflict
                )
            })
            .count();
        let lineage_divergence_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::LineageStructuralDivergence
                )
            })
            .count();
        let mismatch_count =
            usize::from(reduced_match_set.outcome_class().mismatch_class().is_some());

        Self {
            structural_declaration_count: 1,
            structural_contract_count: 1,
            structural_fingerprint_count: planned_packet_set.target_fingerprint().iter().count()
                + planned_packet_set.comparison_fingerprint().iter().count()
                + planned_packet_set
                    .candidates()
                    .iter()
                    .filter(|candidate| candidate.fingerprint().is_some())
                    .count(),
            structural_match_packet_count: 1,
            structural_candidate_count: planned_packet_set.candidate_count(),
            structural_candidate_cohort_count: planned_packet_set.candidate_count(),
            structural_exact_match_count: reduced_match_set.exact_match_count(),
            structural_ambiguity_count: reduced_match_set.ambiguity_count(),
            structural_mismatch_count: mismatch_count,
            structural_identity_conflict_count: identity_conflict_count,
            structural_lineage_divergence_count: lineage_divergence_count,
            structural_reuse_publication_count: usize::from(matches!(
                reduced_match_set.outcome_class(),
                StructuralMatchOutcomeClass::AdvisoryReuseCandidate
            )),
            branch_comparison_count: usize::from(
                planned_packet_set.comparison_mode() == StructuralComparisonMode::BranchComparison,
            ),
            branch_comparison_diff_count: reduced_match_set.branch_diff_count(),
            branch_comparison_drift_rejection_count: 0,
            structural_widened_scan_count: usize::from(matches!(
                declaration.candidate_scope(),
                crate::structural::StructuralCandidateSearchScope::ExplicitWidenedDebtScan
            )),
            structural_replay_request_count: 0,
            structural_replay_mismatch_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_replay_request(mut self) -> Self {
        self.structural_replay_request_count = 1;
        self
    }

    pub fn structural_declaration_count(&self) -> usize {
        self.structural_declaration_count
    }

    pub fn structural_contract_count(&self) -> usize {
        self.structural_contract_count
    }

    pub fn structural_fingerprint_count(&self) -> usize {
        self.structural_fingerprint_count
    }

    pub fn structural_match_packet_count(&self) -> usize {
        self.structural_match_packet_count
    }

    pub fn structural_candidate_count(&self) -> usize {
        self.structural_candidate_count
    }

    pub fn structural_candidate_cohort_count(&self) -> usize {
        self.structural_candidate_cohort_count
    }

    pub fn structural_exact_match_count(&self) -> usize {
        self.structural_exact_match_count
    }

    pub fn structural_ambiguity_count(&self) -> usize {
        self.structural_ambiguity_count
    }

    pub fn structural_mismatch_count(&self) -> usize {
        self.structural_mismatch_count
    }

    pub fn structural_identity_conflict_count(&self) -> usize {
        self.structural_identity_conflict_count
    }

    pub fn structural_lineage_divergence_count(&self) -> usize {
        self.structural_lineage_divergence_count
    }

    pub fn structural_reuse_publication_count(&self) -> usize {
        self.structural_reuse_publication_count
    }

    pub fn branch_comparison_count(&self) -> usize {
        self.branch_comparison_count
    }

    pub fn branch_comparison_diff_count(&self) -> usize {
        self.branch_comparison_diff_count
    }

    pub fn branch_comparison_drift_rejection_count(&self) -> usize {
        self.branch_comparison_drift_rejection_count
    }

    pub fn structural_widened_scan_count(&self) -> usize {
        self.structural_widened_scan_count
    }

    pub fn structural_replay_request_count(&self) -> usize {
        self.structural_replay_request_count
    }

    pub fn structural_replay_mismatch_count(&self) -> usize {
        self.structural_replay_mismatch_count
    }
}
