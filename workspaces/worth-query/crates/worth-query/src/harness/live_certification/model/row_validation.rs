use super::{LiveCertificationRow, LiveHostileExpectation, LiveRejectionRow};

impl LiveCertificationRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        match self.hostile_expectation {
            LiveHostileExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.replay_digest == self.hostile_lane.replay_digest
                    && self.control_lane.family == self.hostile_lane.family
                    && self.control_lane.outcome_kind == self.hostile_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.hostile_lane.outcome_digest
                    && self.control_lane.basis_digest == self.hostile_lane.basis_digest
                    && self.control_lane.subscription_digest
                        == self.hostile_lane.subscription_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
                    && self.control_lane.replay_digest == self.parity_lane.replay_digest
                    && self.control_lane.family == self.parity_lane.family
                    && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
                    && self.control_lane.basis_digest == self.parity_lane.basis_digest
                    && self.control_lane.subscription_digest == self.parity_lane.subscription_digest
            }
            LiveHostileExpectation::ReplayEndStateEquivalent => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.family == self.hostile_lane.family
                    && self.control_lane.outcome_kind == self.hostile_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.hostile_lane.outcome_digest
                    && self.control_lane.basis_digest == self.hostile_lane.basis_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
                    && self.control_lane.family == self.parity_lane.family
                    && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
                    && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
                    && self.control_lane.basis_digest == self.parity_lane.basis_digest
            }
            LiveHostileExpectation::ReplayStepwiseEquivalent => {
                !self.control_lane.replay_step_delivery_digests.is_empty()
                    && self.control_lane.replay_step_delivery_digests
                        == self.hostile_lane.replay_step_delivery_digests
                    && self.control_lane.replay_step_delivery_digests
                        == self.parity_lane.replay_step_delivery_digests
                    && self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.delivery_digest == self.hostile_lane.delivery_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
                    && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
            }
        }
    }
}

impl LiveRejectionRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_digest.is_empty()
            && (self
                .hostile_lane
                .counter_snapshot
                .live_refresh_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_coalescing_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_patch_width_overflow_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_non_monotonic_sequence_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_change_sequence_gap_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_invalid_promotion_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .live_unsupported_patch_family_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_breadth_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_bridge_slice_incompatibility_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .stream_window_width_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_unsupported_family_rejection_count()
                + self
                    .hostile_lane
                    .counter_snapshot
                    .locality_unsupported_predicate_rejection_count()
                > 0)
    }

    pub fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_digest == self.parity_lane.result_digest
            && self.control_lane.delivery_digest == self.parity_lane.delivery_digest
            && self.control_lane.replay_digest == self.parity_lane.replay_digest
            && self.control_lane.family == self.parity_lane.family
            && self.control_lane.outcome_kind == self.parity_lane.outcome_kind
            && self.control_lane.outcome_digest == self.parity_lane.outcome_digest
            && self.control_lane.basis_digest == self.parity_lane.basis_digest
            && self.control_lane.subscription_digest == self.parity_lane.subscription_digest
    }
}
