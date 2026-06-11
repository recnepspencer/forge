#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildParityCounters {
    local_neighborhood_rows: usize,
    rebinding_continuity_rows: usize,
    parity_views_compared: usize,
    source_receipts_consumed: usize,
    denied_substitute_rows: usize,
}

impl PlanarLocalRebuildParityCounters {
    pub(crate) fn certified(
        local_neighborhood_rows: usize,
        rebinding_continuity_rows: usize,
        parity_views_compared: usize,
        source_receipts_consumed: usize,
    ) -> Self {
        Self {
            local_neighborhood_rows,
            rebinding_continuity_rows,
            parity_views_compared,
            source_receipts_consumed,
            denied_substitute_rows: 0,
        }
    }

    pub fn local_neighborhood_rows(self) -> usize {
        self.local_neighborhood_rows
    }

    pub fn rebinding_continuity_rows(self) -> usize {
        self.rebinding_continuity_rows
    }

    pub fn parity_views_compared(self) -> usize {
        self.parity_views_compared
    }

    pub fn source_receipts_consumed(self) -> usize {
        self.source_receipts_consumed
    }

    pub fn denied_substitute_rows(self) -> usize {
        self.denied_substitute_rows
    }
}
