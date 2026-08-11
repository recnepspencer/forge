use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_resume_admission_count(&self) -> usize {
        self.values.subscription_resume_admission_count
    }

    pub fn subscription_resume_admission_rejection_count(&self) -> usize {
        self.values.subscription_resume_admission_rejection_count
    }

    pub fn subscription_resume_plan_count(&self) -> usize {
        self.values.subscription_resume_plan_count
    }

    pub fn subscription_resume_basis_capture_count(&self) -> usize {
        self.values.subscription_resume_basis_capture_count
    }

    pub fn subscription_resume_temporal_basis_count(&self) -> usize {
        self.values.subscription_resume_temporal_basis_count
    }

    pub fn subscription_resume_inflight_async_basis_count(&self) -> usize {
        self.values.subscription_resume_inflight_async_basis_count
    }

    pub fn subscription_resume_delivery_basis_count(&self) -> usize {
        self.values.subscription_resume_delivery_basis_count
    }

    pub fn subscription_resume_basis_admission_count(&self) -> usize {
        self.values.subscription_resume_basis_admission_count
    }

    pub fn subscription_resume_basis_rejection_count(&self) -> usize {
        self.values.subscription_resume_basis_rejection_count
    }

    pub fn subscription_resume_replay_readiness_count(&self) -> usize {
        self.values.subscription_resume_replay_readiness_count
    }

    pub fn subscription_resume_cross_branch_rejection_count(&self) -> usize {
        self.values.subscription_resume_cross_branch_rejection_count
    }

    pub fn subscription_resume_delivery_mismatch_rejection_count(&self) -> usize {
        self.values
            .subscription_resume_delivery_mismatch_rejection_count
    }

    pub fn subscription_resume_inflight_async_generation_rejection_count(&self) -> usize {
        self.values
            .subscription_resume_inflight_async_generation_rejection_count
    }
}
