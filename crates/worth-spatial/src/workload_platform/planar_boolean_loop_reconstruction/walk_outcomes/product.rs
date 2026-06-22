use super::construction::classify_walk_outcomes;
use super::counters::PlanarBooleanWalkOutcomeCounters;
use super::input::PlanarBooleanWalkOutcomeSetInput;
use super::row::{PlanarBooleanWalkOutcomeKind, PlanarBooleanWalkOutcomeRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanWalkOutcomeSet {
    walk_outcome_set_identity: String,
    request_identity: String,
    continuation_index_identity: String,
    rows: Vec<PlanarBooleanWalkOutcomeRow>,
    counters: PlanarBooleanWalkOutcomeCounters,
}

impl PlanarBooleanWalkOutcomeSet {
    pub fn classify(input: PlanarBooleanWalkOutcomeSetInput<'_>) -> Self {
        classify_walk_outcomes(input)
    }

    pub(crate) fn new(
        walk_outcome_set_identity: String,
        request_identity: String,
        continuation_index_identity: String,
        rows: Vec<PlanarBooleanWalkOutcomeRow>,
        counters: PlanarBooleanWalkOutcomeCounters,
    ) -> Self {
        Self {
            walk_outcome_set_identity,
            request_identity,
            continuation_index_identity,
            rows,
            counters,
        }
    }

    pub fn walk_outcome_set_identity(&self) -> &str {
        &self.walk_outcome_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn continuation_index_identity(&self) -> &str {
        &self.continuation_index_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanWalkOutcomeRow] {
        &self.rows
    }

    pub fn closed_rows(&self) -> impl Iterator<Item = &PlanarBooleanWalkOutcomeRow> {
        self.rows
            .iter()
            .filter(|row| row.kind() == PlanarBooleanWalkOutcomeKind::Closed)
    }

    pub fn counters(&self) -> PlanarBooleanWalkOutcomeCounters {
        self.counters
    }
}
