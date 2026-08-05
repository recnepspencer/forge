use super::super::FrontierPredictionDriftOutcome;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierRouteCounters {
    route_lowering_invocation_count: usize,
    route_surface_digest_count: usize,
    route_parallel_admission_count: usize,
    route_serial_fallback_count: usize,
    route_prediction_drift_count: usize,
}

impl FrontierRouteCounters {
    pub fn route_lowering_invocation_count(&self) -> usize {
        self.route_lowering_invocation_count
    }

    pub fn route_surface_digest_count(&self) -> usize {
        self.route_surface_digest_count
    }

    pub fn route_parallel_admission_count(&self) -> usize {
        self.route_parallel_admission_count
    }

    pub fn route_serial_fallback_count(&self) -> usize {
        self.route_serial_fallback_count
    }

    pub fn route_prediction_drift_count(&self) -> usize {
        self.route_prediction_drift_count
    }

    pub(in crate::frontier_planning::testing) fn parallel(
        drift_outcome: &FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_lowering_invocation_count: 1,
            route_surface_digest_count: 1,
            route_parallel_admission_count: 1,
            route_serial_fallback_count: 0,
            route_prediction_drift_count: usize::from(
                *drift_outcome != FrontierPredictionDriftOutcome::WithinBudget,
            ),
        }
    }

    pub(in crate::frontier_planning::testing) fn serial(
        drift_outcome: &FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_lowering_invocation_count: 1,
            route_surface_digest_count: 1,
            route_parallel_admission_count: 0,
            route_serial_fallback_count: 1,
            route_prediction_drift_count: usize::from(
                *drift_outcome != FrontierPredictionDriftOutcome::WithinBudget,
            ),
        }
    }
}
