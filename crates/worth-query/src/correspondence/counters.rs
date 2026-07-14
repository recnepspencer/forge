#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CorrespondenceCounterSnapshot {
    predicted_structural_candidate_count: usize,
    structural_candidate_count: usize,
    structural_candidate_rejection_count: usize,
    structural_ambiguity_count: usize,
    structural_unique_witness_count: usize,
    lineage_structural_disagreement_count: usize,
    structural_authority_promotion_denial_count: usize,
    predicted_correspondence_resolution_width: usize,
    structural_candidate_prediction_drift_count: usize,
    correspondence_executor_rediscovery_count: usize,
}

impl CorrespondenceCounterSnapshot {
    pub fn predicted_structural_candidate_count(&self) -> usize {
        self.predicted_structural_candidate_count
    }

    pub fn structural_candidate_count(&self) -> usize {
        self.structural_candidate_count
    }

    pub fn structural_candidate_rejection_count(&self) -> usize {
        self.structural_candidate_rejection_count
    }

    pub fn structural_ambiguity_count(&self) -> usize {
        self.structural_ambiguity_count
    }

    pub fn structural_unique_witness_count(&self) -> usize {
        self.structural_unique_witness_count
    }

    pub fn lineage_structural_disagreement_count(&self) -> usize {
        self.lineage_structural_disagreement_count
    }

    pub fn structural_authority_promotion_denial_count(&self) -> usize {
        self.structural_authority_promotion_denial_count
    }

    pub fn predicted_correspondence_resolution_width(&self) -> usize {
        self.predicted_correspondence_resolution_width
    }

    pub fn structural_candidate_prediction_drift_count(&self) -> usize {
        self.structural_candidate_prediction_drift_count
    }

    pub fn correspondence_executor_rediscovery_count(&self) -> usize {
        self.correspondence_executor_rediscovery_count
    }

    #[cfg(test)]
    pub(crate) fn vocabulary_baseline() -> Self {
        Self {
            predicted_structural_candidate_count: 1,
            structural_candidate_count: 0,
            structural_candidate_rejection_count: 0,
            structural_ambiguity_count: 0,
            structural_unique_witness_count: 0,
            lineage_structural_disagreement_count: 0,
            structural_authority_promotion_denial_count: 0,
            predicted_correspondence_resolution_width: 1,
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn lineage_direct() -> Self {
        Self {
            predicted_structural_candidate_count: 0,
            structural_candidate_count: 0,
            structural_candidate_rejection_count: 0,
            structural_ambiguity_count: 0,
            structural_unique_witness_count: 0,
            lineage_structural_disagreement_count: 0,
            structural_authority_promotion_denial_count: 0,
            predicted_correspondence_resolution_width: 1,
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn structural_unique(candidate_count: usize) -> Self {
        Self {
            predicted_structural_candidate_count: candidate_count,
            structural_candidate_count: candidate_count,
            structural_candidate_rejection_count: 0,
            structural_ambiguity_count: 0,
            structural_unique_witness_count: 1,
            lineage_structural_disagreement_count: 0,
            structural_authority_promotion_denial_count: 0,
            predicted_correspondence_resolution_width: 1,
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn structural_ambiguous(candidate_count: usize) -> Self {
        Self {
            predicted_structural_candidate_count: candidate_count,
            structural_candidate_count: candidate_count,
            structural_candidate_rejection_count: 0,
            structural_ambiguity_count: 1,
            structural_unique_witness_count: 0,
            lineage_structural_disagreement_count: 0,
            structural_authority_promotion_denial_count: 0,
            predicted_correspondence_resolution_width: candidate_count.max(1),
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn disagreement(candidate_count: usize) -> Self {
        Self {
            predicted_structural_candidate_count: candidate_count,
            structural_candidate_count: candidate_count,
            structural_candidate_rejection_count: 0,
            structural_ambiguity_count: usize::from(candidate_count > 1),
            structural_unique_witness_count: 0,
            lineage_structural_disagreement_count: 1,
            structural_authority_promotion_denial_count: 1,
            predicted_correspondence_resolution_width: candidate_count.max(1),
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn denied(
        predicted_structural_candidate_count: usize,
        structural_candidate_rejection_count: usize,
        structural_authority_promotion_denial_count: usize,
    ) -> Self {
        Self {
            predicted_structural_candidate_count,
            structural_candidate_count: predicted_structural_candidate_count,
            structural_candidate_rejection_count,
            structural_ambiguity_count: 0,
            structural_unique_witness_count: 0,
            lineage_structural_disagreement_count: 0,
            structural_authority_promotion_denial_count,
            predicted_correspondence_resolution_width: 1,
            structural_candidate_prediction_drift_count: 0,
            correspondence_executor_rediscovery_count: 0,
        }
    }
}
