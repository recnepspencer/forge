use super::*;

impl UiAllocationStreamCompositionReceipt {
    pub(crate) fn families(&self) -> &[UiAllocationStreamFamily] {
        &self.families
    }
    pub(crate) fn into_resolution_parts(
        self,
    ) -> (
        UiResolvedAllocationStreamPolicy,
        Box<[UiAllocationIntermediatePolicyVerdict]>,
        Box<[UiResolvedAllocationPolicyBranch]>,
        UiAllocationStreamCompositionCounters,
    ) {
        (self.policy, self.intermediate, self.branches, self.counters)
    }
}

impl UiResolvedAllocationStreamPolicy {
    pub(crate) fn commit_lane(self) -> UiAllocationResolvedCommitLane {
        self.commit_lane
    }
    pub fn target(self) -> UiAllocationCommitTarget {
        self.target
    }
    pub fn cadence(self) -> UiAllocationCadenceKind {
        self.cadence
    }
    pub fn budget(self) -> UiAllocationCadenceBudget {
        self.budget
    }
    pub fn evidence_cadence(self) -> UiAllocationEvidenceCadence {
        self.evidence_cadence
    }
    pub fn collapse_law(self) -> UiAllocationStreamCollapseLaw {
        self.collapse_law
    }
    pub fn partial_settlement_law(self) -> UiAllocationPartialSettlementLaw {
        self.partial_settlement_law
    }

    pub(crate) fn mix_canonical_identity(self, mut digest: u64) -> u64 {
        const FNV_PRIME: u64 = 0x100000001b3;
        let budget = self.budget;
        let commit_lane = match self.commit_lane {
            UiAllocationResolvedCommitLane::Ordinary => 1,
            UiAllocationResolvedCommitLane::ViewportDerived => 2,
            UiAllocationResolvedCommitLane::ResizePreview => 3,
            UiAllocationResolvedCommitLane::DurableResize => 4,
            UiAllocationResolvedCommitLane::DragResize => 5,
        };
        let words = [
            0x776f7274682d706fu64,
            commit_lane,
            self.target as u64,
            self.cadence as u64,
            self.evidence_cadence as u64,
            self.collapse_law as u64,
            self.partial_settlement_law as u64,
            budget.ingress_window() as u64,
            budget.max_resolved_plans() as u64,
            budget.max_committed_receipts() as u64,
            budget.max_invalidation_targets() as u64,
            budget.max_durable_mutations() as u64,
            budget.max_lag_frames() as u64,
        ];
        for word in words {
            digest ^= word;
            digest = digest.wrapping_mul(FNV_PRIME);
        }
        digest
    }

    #[cfg(test)]
    pub(crate) fn with_commit_lane_for_identity_test(
        mut self,
        commit_lane: UiAllocationResolvedCommitLane,
    ) -> Self {
        self.commit_lane = commit_lane;
        self
    }
}

impl UiResolvedAllocationPolicyBranch {
    pub fn families(&self) -> &[UiAllocationStreamFamily] {
        &self.families
    }
    pub fn policy(&self) -> UiResolvedAllocationStreamPolicy {
        self.policy
    }
}

impl UiAllocationIntermediatePolicyVerdict {
    pub fn left(&self) -> UiAllocationStreamFamily {
        self.left
    }
    pub fn right(&self) -> UiAllocationStreamFamily {
        self.right
    }
    pub fn outcome(&self) -> UiAllocationFamilyPairOutcome {
        self.outcome
    }
    pub fn resolved(&self) -> UiResolvedAllocationStreamPolicy {
        self.resolved
    }
}

impl UiAllocationStreamCompositionCounters {
    pub fn admitted_family_count(self) -> u8 {
        self.admitted_family_count
    }
    pub fn pair_contract_evaluations(self) -> u8 {
        self.pair_contract_evaluations
    }
    pub fn admitted_input_count(self) -> u16 {
        self.admitted_input_count
    }
    pub fn pair_policy_joins(self) -> u8 {
        self.pair_policy_joins
    }
    pub fn n_way_policy_joins(self) -> u8 {
        self.n_way_policy_joins
    }
    pub fn branch_policy_joins(self) -> u8 {
        self.branch_policy_joins
    }
}
