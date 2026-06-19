#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSourceLoopSplitAttributionCounters {
    island_rows_consumed: usize,
    attribution_rows_emitted: usize,
}

impl PlanarBooleanSourceLoopSplitAttributionCounters {
    pub(crate) fn consumed_island_row(&mut self) {
        self.island_rows_consumed += 1;
    }

    pub(crate) fn emitted_attribution_row(&mut self) {
        self.attribution_rows_emitted += 1;
    }

    pub fn island_rows_consumed(self) -> usize {
        self.island_rows_consumed
    }

    pub fn attribution_rows_emitted(self) -> usize {
        self.attribution_rows_emitted
    }
}
