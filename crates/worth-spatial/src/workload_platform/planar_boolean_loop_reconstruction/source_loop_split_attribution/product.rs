use super::construction::attribute_source_loop_splits;
use super::counters::PlanarBooleanSourceLoopSplitAttributionCounters;
use super::input::PlanarBooleanSourceLoopSplitAttributionInput;
use super::row::PlanarBooleanSourceLoopSplitAttributionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSourceLoopSplitAttribution {
    attribution_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanSourceLoopSplitAttributionRow>,
    counters: PlanarBooleanSourceLoopSplitAttributionCounters,
}

impl PlanarBooleanSourceLoopSplitAttribution {
    pub fn attribute(input: PlanarBooleanSourceLoopSplitAttributionInput<'_>) -> Self {
        attribute_source_loop_splits(input)
    }

    pub(crate) fn new(
        attribution_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanSourceLoopSplitAttributionRow>,
        counters: PlanarBooleanSourceLoopSplitAttributionCounters,
    ) -> Self {
        Self {
            attribution_identity,
            request_identity,
            rows,
            counters,
        }
    }

    pub fn attribution_identity(&self) -> &str {
        &self.attribution_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanSourceLoopSplitAttributionRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanSourceLoopSplitAttributionCounters {
        self.counters
    }
}
