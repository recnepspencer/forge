use super::construction::classify_degenerate_loop_outcomes;
use super::counters::PlanarBooleanDegenerateLoopOutcomeBoundaryCounters;
use super::input::PlanarBooleanDegenerateLoopOutcomeBoundaryInput;
use super::row::PlanarBooleanDegenerateLoopOutcome;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDegenerateLoopOutcomeSet {
    set_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanDegenerateLoopOutcome>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDegenerateLoopOutcomeBoundary {
    outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
    counters: PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
}

impl PlanarBooleanDegenerateLoopOutcomeBoundary {
    pub fn classify(input: PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'_>) -> Self {
        classify_degenerate_loop_outcomes(input)
    }

    pub(crate) fn new(
        outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
        counters: PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
    ) -> Self {
        Self { outcomes, counters }
    }

    pub fn outcomes(&self) -> &PlanarBooleanDegenerateLoopOutcomeSet {
        &self.outcomes
    }

    pub fn counters(&self) -> PlanarBooleanDegenerateLoopOutcomeBoundaryCounters {
        self.counters
    }
}

impl PlanarBooleanDegenerateLoopOutcomeSet {
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanDegenerateLoopOutcome>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            rows,
        }
    }

    pub fn degenerate_loop_outcome_set_identity(&self) -> &str {
        &self.set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanDegenerateLoopOutcome] {
        &self.rows
    }
}
