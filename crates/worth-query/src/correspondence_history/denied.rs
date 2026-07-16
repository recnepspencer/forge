use crate::correspondence::{CorrespondenceDenied, CorrespondenceEvidenceResolved};
use crate::historical::{
    HistoricalCounterSnapshot, HistoricalEvaluationAdmission, HistoricalEvaluationError,
    HistoricalEvaluationRequest, HistoricalPathCompatibilityOutcome, HistoricalPathCostPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    denied: CorrespondenceDenied,
}

impl CorrespondenceHistoricalDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn denied(&self) -> &CorrespondenceDenied {
        &self.denied
    }

    #[cfg(test)]
    pub(crate) fn new(
        correspondence: CorrespondenceEvidenceResolved,
        denied: CorrespondenceDenied,
    ) -> Self {
        Self {
            correspondence,
            denied,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    admission: HistoricalEvaluationAdmission,
    error: HistoricalEvaluationError,
    denial_cost_posture: HistoricalPathCostPosture,
    counters: HistoricalCounterSnapshot,
    compatibility_outcome: HistoricalPathCompatibilityOutcome,
}

impl HistoricalPathDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn admission(&self) -> &HistoricalEvaluationAdmission {
        &self.admission
    }

    pub fn error(&self) -> &HistoricalEvaluationError {
        &self.error
    }

    pub fn denial_cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.denial_cost_posture
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub fn compatibility_outcome(&self) -> &HistoricalPathCompatibilityOutcome {
        &self.compatibility_outcome
    }

    #[cfg(test)]
    pub(crate) fn new(
        correspondence: CorrespondenceEvidenceResolved,
        admission: HistoricalEvaluationAdmission,
        error: HistoricalEvaluationError,
    ) -> Self {
        let denial_cost_posture = error.denial_cost_posture();
        let compatibility_outcome = match error.failure_class() {
            crate::historical::HistoricalEvaluationFailureClass::HiddenPathSubstitutionDenied => {
                HistoricalPathCompatibilityOutcome::SubstitutionDenied
            }
            _ => HistoricalPathCompatibilityOutcome::Denied,
        };
        let counters = match compatibility_outcome {
            HistoricalPathCompatibilityOutcome::SubstitutionDenied => admission
                .counters()
                .clone()
                .with_hidden_path_substitution_denial(),
            HistoricalPathCompatibilityOutcome::Denied => {
                admission.counters().clone().with_path_denial()
            }
            HistoricalPathCompatibilityOutcome::Admitted => admission.counters().clone(),
        };
        Self {
            correspondence,
            admission,
            error,
            denial_cost_posture,
            counters,
            compatibility_outcome,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathAdmissionDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    request: HistoricalEvaluationRequest,
    error: HistoricalEvaluationError,
    denial_cost_posture: HistoricalPathCostPosture,
    counters: HistoricalCounterSnapshot,
}

impl HistoricalPathAdmissionDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn request(&self) -> &HistoricalEvaluationRequest {
        &self.request
    }

    pub fn error(&self) -> &HistoricalEvaluationError {
        &self.error
    }

    pub fn denial_cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.denial_cost_posture
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub fn compatibility_outcome(&self) -> HistoricalPathCompatibilityOutcome {
        HistoricalPathCompatibilityOutcome::Denied
    }

    #[cfg(test)]
    pub(crate) fn new(
        correspondence: CorrespondenceEvidenceResolved,
        request: HistoricalEvaluationRequest,
        error: HistoricalEvaluationError,
    ) -> Self {
        let denial_cost_posture = error.denial_cost_posture();
        let counters = HistoricalCounterSnapshot::denied(
            request.replay_budget().max_replay_events(),
            request.reconstruction_budget().max_reconstruction_scope(),
        );
        Self {
            correspondence,
            request,
            error,
            denial_cost_posture,
            counters,
        }
    }
}
