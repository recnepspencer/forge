#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncForwardCausalityCounters {
    retry_after_timeout: u32,
    retry_after_cancellation: u32,
    revalidation_after_truth_basis_drift: u32,
    revalidation_after_preview_basis_drift: u32,
    revalidation_after_subscription_instance_drift: u32,
    rejection_count: u32,
}

impl BridgeAsyncForwardCausalityCounters {
    pub(crate) fn one_retry_after_timeout() -> Self {
        Self {
            retry_after_timeout: 1,
            retry_after_cancellation: 0,
            revalidation_after_truth_basis_drift: 0,
            revalidation_after_preview_basis_drift: 0,
            revalidation_after_subscription_instance_drift: 0,
            rejection_count: 0,
        }
    }

    pub(crate) fn one_retry_after_cancellation() -> Self {
        Self {
            retry_after_timeout: 0,
            retry_after_cancellation: 1,
            revalidation_after_truth_basis_drift: 0,
            revalidation_after_preview_basis_drift: 0,
            revalidation_after_subscription_instance_drift: 0,
            rejection_count: 0,
        }
    }

    pub(crate) fn one_revalidation_after_truth_basis_drift() -> Self {
        Self {
            retry_after_timeout: 0,
            retry_after_cancellation: 0,
            revalidation_after_truth_basis_drift: 1,
            revalidation_after_preview_basis_drift: 0,
            revalidation_after_subscription_instance_drift: 0,
            rejection_count: 0,
        }
    }

    pub(crate) fn one_revalidation_after_preview_basis_drift() -> Self {
        Self {
            retry_after_timeout: 0,
            retry_after_cancellation: 0,
            revalidation_after_truth_basis_drift: 0,
            revalidation_after_preview_basis_drift: 1,
            revalidation_after_subscription_instance_drift: 0,
            rejection_count: 0,
        }
    }

    pub(crate) fn one_revalidation_after_subscription_instance_drift() -> Self {
        Self {
            retry_after_timeout: 0,
            retry_after_cancellation: 0,
            revalidation_after_truth_basis_drift: 0,
            revalidation_after_preview_basis_drift: 0,
            revalidation_after_subscription_instance_drift: 1,
            rejection_count: 0,
        }
    }

    pub fn retry_after_timeout(&self) -> u32 {
        self.retry_after_timeout
    }

    pub fn retry_after_cancellation(&self) -> u32 {
        self.retry_after_cancellation
    }

    pub fn revalidation_after_truth_basis_drift(&self) -> u32 {
        self.revalidation_after_truth_basis_drift
    }

    pub fn revalidation_after_preview_basis_drift(&self) -> u32 {
        self.revalidation_after_preview_basis_drift
    }

    pub fn revalidation_after_subscription_instance_drift(&self) -> u32 {
        self.revalidation_after_subscription_instance_drift
    }

    pub fn rejection_count(&self) -> u32 {
        self.rejection_count
    }
}
