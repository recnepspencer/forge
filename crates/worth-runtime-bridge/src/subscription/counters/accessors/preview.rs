use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_preview_residue_nonzero_count(&self) -> usize {
        self.values.subscription_preview_residue_nonzero_count
    }

    pub fn subscription_preview_basis_admission_count(&self) -> usize {
        self.values.subscription_preview_basis_admission_count
    }

    pub fn subscription_preview_basis_rejection_count(&self) -> usize {
        self.values.subscription_preview_basis_rejection_count
    }

    pub fn subscription_preview_activation_count(&self) -> usize {
        self.values.subscription_preview_activation_count
    }

    pub fn subscription_preview_discard_count(&self) -> usize {
        self.values.subscription_preview_discard_count
    }

    pub fn subscription_preview_discard_rejection_count(&self) -> usize {
        self.values.subscription_preview_discard_rejection_count
    }

    pub fn subscription_preview_residue_check_count(&self) -> usize {
        self.values.subscription_preview_residue_check_count
    }

    pub fn subscription_preview_residue_scope_index_lookup_count(&self) -> usize {
        self.values
            .subscription_preview_residue_scope_index_lookup_count
    }

    pub fn subscription_preview_non_scope_registry_scan_count(&self) -> usize {
        self.values
            .subscription_preview_non_scope_registry_scan_count
    }

    pub fn subscription_preview_lifecycle_residue_envelope_count(&self) -> usize {
        self.values
            .subscription_preview_lifecycle_residue_envelope_count
    }

    pub fn subscription_preview_promotion_count(&self) -> usize {
        self.values.subscription_preview_promotion_count
    }

    pub fn subscription_preview_promotion_rejection_count(&self) -> usize {
        self.values.subscription_preview_promotion_rejection_count
    }

    pub fn subscription_preview_authoritative_readmission_count(&self) -> usize {
        self.values
            .subscription_preview_authoritative_readmission_count
    }

    pub fn subscription_preview_crossed_completion_rejection_count(&self) -> usize {
        self.values
            .subscription_preview_crossed_completion_rejection_count
    }

    pub fn subscription_preview_temporal_evidence_drift_rejection_count(&self) -> usize {
        self.values
            .subscription_preview_temporal_evidence_drift_rejection_count
    }
}
