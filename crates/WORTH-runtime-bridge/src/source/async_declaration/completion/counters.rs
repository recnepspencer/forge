#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeAsyncCompletionCounters {
    completion_envelope_validation_count: usize,
    completion_admission_count: usize,
    completion_denial_count: usize,
    completion_supersession_classification_count: usize,
    invalid_completion_envelope_rejection_count: usize,
    signal_completion_denial_count: usize,
    request_response_completion_count: usize,
    subscription_backed_completion_count: usize,
    completion_rejection_count: usize,
    truth_basis_supersession_count: usize,
    branch_drift_supersession_count: usize,
    preview_basis_drift_supersession_count: usize,
    preview_discarded_supersession_count: usize,
    subscription_instance_supersession_count: usize,
    signal_generation_supersession_count: usize,
}

impl BridgeAsyncCompletionCounters {
    pub(crate) fn envelope_validated() -> Self {
        Self {
            completion_envelope_validation_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn admitted_request_response() -> Self {
        Self {
            completion_admission_count: 1,
            request_response_completion_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn admitted_subscription_backed() -> Self {
        Self {
            completion_admission_count: 1,
            subscription_backed_completion_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_request_response() -> Self {
        Self {
            completion_denial_count: 1,
            signal_completion_denial_count: 1,
            request_response_completion_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_subscription_backed() -> Self {
        Self {
            completion_denial_count: 1,
            signal_completion_denial_count: 1,
            subscription_backed_completion_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn invalid_envelope() -> Self {
        Self {
            invalid_completion_envelope_rejection_count: 1,
            completion_rejection_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn rejected() -> Self {
        Self {
            completion_rejection_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_truth_basis_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            truth_basis_supersession_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_branch_drift_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            branch_drift_supersession_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_preview_basis_drift_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            preview_basis_drift_supersession_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_preview_discarded_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            preview_discarded_supersession_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_subscription_instance_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            subscription_instance_supersession_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn classified_signal_generation_supersession() -> Self {
        Self {
            completion_supersession_classification_count: 1,
            signal_generation_supersession_count: 1,
            ..Self::default()
        }
    }

    pub fn completion_envelope_validation_count(&self) -> usize {
        self.completion_envelope_validation_count
    }

    pub fn completion_admission_count(&self) -> usize {
        self.completion_admission_count
    }

    pub fn completion_denial_count(&self) -> usize {
        self.completion_denial_count
    }

    pub fn invalid_completion_envelope_rejection_count(&self) -> usize {
        self.invalid_completion_envelope_rejection_count
    }

    pub fn completion_supersession_classification_count(&self) -> usize {
        self.completion_supersession_classification_count
    }

    pub fn signal_completion_denial_count(&self) -> usize {
        self.signal_completion_denial_count
    }

    pub fn request_response_completion_count(&self) -> usize {
        self.request_response_completion_count
    }

    pub fn subscription_backed_completion_count(&self) -> usize {
        self.subscription_backed_completion_count
    }

    pub fn completion_rejection_count(&self) -> usize {
        self.completion_rejection_count
    }

    pub fn truth_basis_supersession_count(&self) -> usize {
        self.truth_basis_supersession_count
    }

    pub fn branch_drift_supersession_count(&self) -> usize {
        self.branch_drift_supersession_count
    }

    pub fn preview_basis_drift_supersession_count(&self) -> usize {
        self.preview_basis_drift_supersession_count
    }

    pub fn preview_discarded_supersession_count(&self) -> usize {
        self.preview_discarded_supersession_count
    }

    pub fn subscription_instance_supersession_count(&self) -> usize {
        self.subscription_instance_supersession_count
    }

    pub fn signal_generation_supersession_count(&self) -> usize {
        self.signal_generation_supersession_count
    }
}
