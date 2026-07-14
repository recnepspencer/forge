use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_continuation_index(candidate_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_index_build_count: 1,
            subscription_continuation_candidate_count: candidate_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_continuation_decision(child_record_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_candidate_index_lookup_count: 1,
            subscription_continuation_decision_count: 1,
            subscription_continuation_child_record_count: child_record_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_continuation_rejection(candidate_index_lookup: bool, branch_leak: bool) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_candidate_index_lookup_count: usize::from(
                candidate_index_lookup,
            ),
            subscription_continuation_rejection_count: 1,
            subscription_branch_leak_rejection_count: usize::from(branch_leak),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_basis_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_basis_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_basis_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_basis_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_activation() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_activation_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_residue_scope_index(artifact_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_residue_check_count: artifact_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_discard(residue_check_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_discard_count: 1,
            subscription_preview_residue_check_count: residue_check_count,
            subscription_preview_residue_scope_index_lookup_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_discard_rejection(
        nonzero_residue: bool,
        residue_check_count: usize,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_discard_rejection_count: 1,
            subscription_preview_residue_check_count: residue_check_count,
            subscription_preview_residue_scope_index_lookup_count: 1,
            subscription_preview_residue_nonzero_count: usize::from(nonzero_residue),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_lifecycle_residue_envelope() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_lifecycle_residue_envelope_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_promotion() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_promotion_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_promotion_rejection(
        crossed_completion: bool,
        temporal_evidence_drift: bool,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_promotion_rejection_count: 1,
            subscription_preview_crossed_completion_rejection_count: usize::from(
                crossed_completion,
            ),
            subscription_preview_temporal_evidence_drift_rejection_count: usize::from(
                temporal_evidence_drift,
            ),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_authoritative_readmission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_authoritative_readmission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }
}
