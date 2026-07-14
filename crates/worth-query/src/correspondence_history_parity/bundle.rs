use crate::historical::{HistoricalPathCompatibilityOutcome, PerformancePredictionDriftOutcome};
use crate::identity::{
    BasisDigest, CorrespondenceCostPostureDigest, CorrespondenceOutcomeDigest,
    CounterSnapshotDigest, FailureDigest, HistoricalCostPostureDigest, HistoricalPathClassDigest,
    LineageDigest, ResultDigest, ValidatedQueryDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalParityBundleError {
    MissingDeniedQueryDigest,
    MissingDeniedBasisDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalParityVariant {
    Success,
    Ambiguity,
    Disagreement,
    CorrespondenceDenied,
    HistoricalPathDenied,
}

impl CorrespondenceHistoricalParityVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Ambiguity => "ambiguity",
            Self::Disagreement => "disagreement",
            Self::CorrespondenceDenied => "correspondence_denied",
            Self::HistoricalPathDenied => "historical_path_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalParityBundle {
    pub(crate) parity_variant: CorrespondenceHistoricalParityVariant,
    pub(crate) query_digest: ValidatedQueryDigest,
    pub(crate) lineage_digest: LineageDigest,
    pub(crate) basis_digest: BasisDigest,
    pub(crate) result_digest: Option<ResultDigest>,
    pub(crate) failure_digest: Option<FailureDigest>,
    pub(crate) correspondence_outcome_digest: CorrespondenceOutcomeDigest,
    pub(crate) requested_path_digest: Option<HistoricalPathClassDigest>,
    pub(crate) admitted_path_digest: Option<HistoricalPathClassDigest>,
    pub(crate) resolved_path_digest: Option<HistoricalPathClassDigest>,
    pub(crate) historical_compatibility_outcome: Option<HistoricalPathCompatibilityOutcome>,
    pub(crate) correspondence_cost_posture_digest: CorrespondenceCostPostureDigest,
    pub(crate) historical_cost_posture_digest: Option<HistoricalCostPostureDigest>,
    pub(crate) counter_snapshot_digest: CounterSnapshotDigest,
    pub(crate) performance_prediction_drift_outcome: PerformancePredictionDriftOutcome,
}

impl CorrespondenceHistoricalParityBundle {
    pub fn parity_variant(&self) -> &CorrespondenceHistoricalParityVariant {
        &self.parity_variant
    }

    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> Option<&ResultDigest> {
        self.result_digest.as_ref()
    }

    pub fn failure_digest(&self) -> Option<&FailureDigest> {
        self.failure_digest.as_ref()
    }

    pub fn correspondence_outcome_digest(&self) -> &CorrespondenceOutcomeDigest {
        &self.correspondence_outcome_digest
    }

    pub fn requested_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.requested_path_digest.as_ref()
    }

    pub fn admitted_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.admitted_path_digest.as_ref()
    }

    pub fn resolved_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.resolved_path_digest.as_ref()
    }

    pub fn historical_compatibility_outcome(&self) -> Option<&HistoricalPathCompatibilityOutcome> {
        self.historical_compatibility_outcome.as_ref()
    }

    pub fn correspondence_cost_posture_digest(&self) -> &CorrespondenceCostPostureDigest {
        &self.correspondence_cost_posture_digest
    }

    pub fn historical_cost_posture_digest(&self) -> Option<&HistoricalCostPostureDigest> {
        self.historical_cost_posture_digest.as_ref()
    }

    pub fn counter_snapshot_digest(&self) -> &CounterSnapshotDigest {
        &self.counter_snapshot_digest
    }

    pub fn performance_prediction_drift_outcome(&self) -> &PerformancePredictionDriftOutcome {
        &self.performance_prediction_drift_outcome
    }
}
